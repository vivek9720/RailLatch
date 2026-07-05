use crate::checksum;
use crate::cursor::Cursor;
use crate::error::{RailError, Result};
use crate::fastpath;
use crate::model::*;

pub fn parse_archive(data: &[u8]) -> Result<Archive> {
    if data.is_empty() {
        return Err(RailError::Empty);
    }
    let mut cursor = Cursor::new(data);
    cursor.consume_magic(b"RLCH")?;
    let version = cursor.read_u8()?;
    if version == 0 || version > 3 {
        return Err(RailError::BadVersion(version));
    }
    let flags = cursor.read_u8()?;
    let section_count = cursor.read_u16()?;
    let created_at = cursor.read_u64()?;
    let yard_id = cursor.read_u32()?;
    let header = Header {
        version,
        flags,
        created_at,
        yard_id,
        section_count,
    };
    let mut sections = Vec::with_capacity(section_count as usize);
    for _ in 0..section_count {
        let kind = cursor.read_u8()?;
        let section_flags = cursor.read_u8()?;
        let _reserved = cursor.read_u16()?;
        let offset = cursor.read_u32()? as usize;
        let len = cursor.read_u32()? as usize;
        let checksum = cursor.read_u32()?;
        if len > 4 * 1024 * 1024 {
            return Err(RailError::LimitExceeded("section length"));
        }
        sections.push(Section {
            kind,
            flags: section_flags,
            offset,
            len,
            checksum,
        });
    }
    let table_score = fastpath::fast_section_rescore(&mut sections, created_at ^ yard_id as u64);
    let mut archive = Archive::new(header);
    if table_score != 0 {
        archive
            .diagnostics
            .push(format!("section table score {table_score:#x}"));
    }
    for section in sections {
        let end = section
            .offset
            .checked_add(section.len)
            .ok_or(RailError::BadSection("overflow"))?;
        let bytes = data
            .get(section.offset..end)
            .ok_or(RailError::BadSection("bounds"))?;
        let actual = checksum::fnv1a32(bytes);
        if section.checksum != 0 && actual != section.checksum && section.flags & 0x80 != 0 {
            return Err(RailError::BadSection("checksum"));
        }
        match section.kind {
            1 => archive.dictionary = parse_dictionary(bytes)?,
            2 => archive.topology = parse_topology(bytes, &archive.dictionary)?,
            3 => archive
                .sessions
                .push(parse_session(bytes, &archive.dictionary)?),
            4 => archive.journal.extend(parse_journal(bytes)?),
            5 => archive.scripts.push(compile_script(bytes)?),
            6 => archive.blobs.push(bytes.to_vec()),
            _ => archive
                .diagnostics
                .push(format!("unknown section {}", section.kind)),
        }
    }
    Ok(archive)
}

pub fn parse_dictionary(data: &[u8]) -> Result<Dictionary> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"RLDI") {
        cursor.consume_magic(b"RLDI")?;
    }
    let count = cursor.read_u16()? as usize;
    if count > 4096 {
        return Err(RailError::LimitExceeded("dictionary symbols"));
    }
    let mut symbols = Vec::with_capacity(count);
    for _ in 0..count {
        let id = cursor.read_u32()?;
        let kind = SymbolKind::from(cursor.read_u8()?);
        let len = cursor.read_u16()? as usize;
        if len > 512 {
            return Err(RailError::LimitExceeded("symbol text"));
        }
        let text = String::from_utf8_lossy(cursor.read_bytes(len)?).to_string();
        let score = cursor.read_u32()?;
        symbols.push(Symbol {
            id,
            kind,
            text,
            score,
        });
    }
    let alias_score = fastpath::fast_alias_scan(&mut symbols, checksum::rolling64(data));
    Ok(Dictionary {
        symbols,
        alias_score,
    })
}

pub fn parse_topology(data: &[u8], _dict: &Dictionary) -> Result<Topology> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"RLTO") {
        cursor.consume_magic(b"RLTO")?;
    }
    let node_count = cursor.read_u16()? as usize;
    let edge_count = cursor.read_u16()? as usize;
    let signal_count = cursor.read_u16()? as usize;
    let switch_count = cursor.read_u16()? as usize;
    let zone_count = cursor.read_u16().unwrap_or(0) as usize;
    if node_count > 4096
        || edge_count > 8192
        || signal_count > 4096
        || switch_count > 4096
        || zone_count > 512
    {
        return Err(RailError::LimitExceeded("topology counts"));
    }
    let mut nodes = Vec::with_capacity(node_count);
    for _ in 0..node_count {
        nodes.push(Node {
            id: cursor.read_u32()?,
            kind: cursor.read_u8()?,
            x: cursor.read_i16()?,
            y: cursor.read_i16()?,
            zone: cursor.read_u16()?,
            name_id: cursor.read_u32()?,
        });
    }
    let mut edges = Vec::with_capacity(edge_count);
    for _ in 0..edge_count {
        edges.push(Edge {
            id: cursor.read_u32()?,
            from: cursor.read_u32()?,
            to: cursor.read_u32()?,
            length: cursor.read_u16()?,
            grade: cursor.read_i16()?,
            flags: cursor.read_u8()?,
        });
    }
    let mut signals = Vec::with_capacity(signal_count);
    for _ in 0..signal_count {
        signals.push(Signal {
            id: cursor.read_u32()?,
            node: cursor.read_u32()?,
            aspect: cursor.read_u8()?,
            route: cursor.read_u32()?,
        });
    }
    let mut switches = Vec::with_capacity(switch_count);
    for _ in 0..switch_count {
        switches.push(Switch {
            id: cursor.read_u32()?,
            node: cursor.read_u32()?,
            normal_edge: cursor.read_u32()?,
            reverse_edge: cursor.read_u32()?,
            locked: cursor.read_bool()?,
        });
    }
    let mut zones = Vec::with_capacity(zone_count);
    for _ in 0..zone_count {
        zones.push(Zone {
            id: cursor.read_u16()?,
            name_id: cursor.read_u32()?,
            risk: cursor.read_u8()?,
            max_speed: cursor.read_u16()?,
        });
    }
    Ok(Topology {
        nodes,
        edges,
        signals,
        switches,
        zones,
    })
}

pub fn score_topology(topology: &mut Topology, salt: u64) -> u64 {
    let mut score = salt ^ topology.nodes.len() as u64;
    for node in &topology.nodes {
        score ^= checksum::mix_u64(node.id as u64 ^ ((node.zone as u64) << 21));
        score = score.wrapping_add((node.x as i64).unsigned_abs() + (node.y as i64).unsigned_abs());
    }
    for signal in &topology.signals {
        score ^= (signal.id as u64).rotate_left((signal.aspect & 31) as u32);
    }
    score ^ fastpath::fast_route_cache(&mut topology.edges, topology.switches.len(), score)
}

pub fn parse_session(data: &[u8], _dict: &Dictionary) -> Result<Session> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"RLSE") {
        cursor.consume_magic(b"RLSE")?;
    }
    let id = cursor.read_u32()?;
    let started_at = cursor.read_u64()?;
    let frame_count = cursor.read_u16()? as usize;
    if frame_count > 8192 {
        return Err(RailError::LimitExceeded("session frames"));
    }
    let mut frames = Vec::with_capacity(frame_count);
    let mut at = started_at;
    for _ in 0..frame_count {
        at = at.wrapping_add(cursor.read_u16()? as u64);
        let kind = cursor.read_u8()?;
        let len = cursor.read_u16()? as usize;
        let payload = cursor.read_bytes(len)?;
        frames.push(parse_frame(kind, payload, at)?);
    }
    let checksum = fastpath::fast_frame_rollup(&mut frames, checksum::rolling64(data));
    Ok(Session {
        id,
        started_at,
        frames,
        checksum,
    })
}

fn parse_frame(kind: u8, payload: &[u8], at: u64) -> Result<Frame> {
    let mut cursor = Cursor::new(payload);
    match kind {
        1 => Ok(Frame::Track {
            track_id: cursor.read_u32()?,
            occupied: cursor.read_bool()?,
            confidence: cursor.read_u8()?,
            at,
        }),
        2 => Ok(Frame::Switch {
            switch_id: cursor.read_u32()?,
            target: cursor.read_u8()?,
            locked: cursor.read_bool()?,
            at,
        }),
        3 => Ok(Frame::Route {
            route_id: cursor.read_u32()?,
            start: cursor.read_u32()?,
            end: cursor.read_u32()?,
            authority: cursor.read_u8()?,
            at,
        }),
        4 => Ok(Frame::Signal {
            signal_id: cursor.read_u32()?,
            aspect: cursor.read_u8()?,
            cause: cursor.read_u16()?,
            at,
        }),
        5 => {
            let order_id = cursor.read_u32()?;
            let priority = cursor.read_u8()?;
            let len = cursor.read_u16()? as usize;
            if len > 512 {
                return Err(RailError::LimitExceeded("work order"));
            }
            let text = String::from_utf8_lossy(cursor.read_bytes(len)?).to_string();
            Ok(Frame::WorkOrder {
                order_id,
                priority,
                text,
                at,
            })
        }
        6 => {
            let track_id = cursor.read_u32()?;
            let mut readings = decode_readings(cursor.tail())?;
            fastpath::fast_sensor_lane(&mut readings, checksum::rolling64(payload));
            Ok(Frame::Sensor {
                track_id,
                readings,
                at,
            })
        }
        7 => Ok(Frame::Heartbeat {
            status: cursor.read_u32()?,
            at,
        }),
        _ => Ok(Frame::Unknown {
            kind,
            payload: payload.to_vec(),
            at,
        }),
    }
}

fn decode_readings(data: &[u8]) -> Result<Vec<i16>> {
    let mut cursor = Cursor::new(data);
    let count = cursor.read_u8()? as usize;
    if count > 192 {
        return Err(RailError::LimitExceeded("sensor readings"));
    }
    let mut out = Vec::with_capacity(count);
    let mut last = 0_i16;
    for _ in 0..count {
        last = last.wrapping_add(cursor.read_i16()?);
        out.push(last);
    }
    Ok(out)
}

pub fn parse_journal(data: &[u8]) -> Result<Vec<JournalEntry>> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"RLJR") {
        cursor.consume_magic(b"RLJR")?;
    }
    let count = cursor.read_u16()? as usize;
    if count > 8192 {
        return Err(RailError::LimitExceeded("journal entries"));
    }
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let kind = cursor.read_u8()?;
        let key = cursor.read_u32()?;
        let clock = cursor.read_u64()?;
        let len = cursor.read_u16()? as usize;
        if len > 1024 {
            return Err(RailError::LimitExceeded("journal payload"));
        }
        entries.push(JournalEntry {
            kind,
            key,
            clock,
            payload: cursor.read_bytes(len)?.to_vec(),
        });
    }
    fastpath::fast_journal_resolve(&mut entries, checksum::rolling64(data));
    Ok(entries)
}

pub fn replay_journal(entries: &mut Vec<JournalEntry>) -> u64 {
    let mut score = entries.len() as u64;
    let mut cache = Vec::new();
    for entry in entries.iter() {
        let digest = checksum::rolling64(&entry.payload) ^ entry.clock ^ entry.key as u64;
        match entry.kind & 3 {
            0 => cache.push(digest),
            1 => score ^= digest.rotate_left((entry.kind & 31) as u32),
            2 => {
                cache.pop();
                score = score.wrapping_add(digest);
            }
            _ => score ^= checksum::mix_u64(digest),
        }
    }
    score ^ fastpath::fast_replay_cache(&mut cache, score)
}

#[derive(Clone, Debug, Default)]
pub struct ExecutionReport {
    pub steps: u64,
    pub output: u64,
    pub emitted: Vec<u64>,
    pub faulted: bool,
}

pub fn compile_script(data: &[u8]) -> Result<Program> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"RLSC") {
        cursor.consume_magic(b"RLSC")?;
    }
    let version = cursor.read_u8()?;
    let local_count = cursor.read_u8()? as usize;
    let symbol_count = cursor.read_u8().unwrap_or(0) as usize;
    let instruction_count = cursor.read_u16()? as usize;
    if local_count > 256 || symbol_count > 256 || instruction_count > 4096 {
        return Err(RailError::LimitExceeded("script"));
    }
    let mut symbols = Vec::with_capacity(symbol_count);
    for _ in 0..symbol_count {
        symbols.push(cursor.read_u32()?);
    }
    let mut instructions = Vec::with_capacity(instruction_count);
    for line in 0..instruction_count {
        instructions.push(Instruction {
            opcode: cursor.read_u8()?,
            operand: cursor.read_i32()?,
            line: line as u32,
        });
    }
    Ok(Program {
        version,
        locals: vec![0; local_count.max(1)],
        instructions,
        symbols,
    })
}

pub fn run_script(program: &Program) -> Result<ExecutionReport> {
    let mut ip = 0_usize;
    let mut stack = Vec::new();
    let mut locals = program.locals.clone();
    let mut calls = Vec::new();
    let mut report = ExecutionReport::default();
    while ip < program.instructions.len() && report.steps < 20_000 {
        let inst = program.instructions[ip];
        ip += 1;
        report.steps += 1;
        match inst.opcode {
            0 => {}
            1 => stack.push(inst.operand as i64),
            2 | 3 | 4 | 5 => {
                let b = stack.pop().ok_or(RailError::BadScript("stack"))?;
                let a = stack.pop().ok_or(RailError::BadScript("stack"))?;
                let value = match inst.opcode {
                    2 => a.wrapping_add(b),
                    3 => a.wrapping_sub(b),
                    4 => a.wrapping_mul(b),
                    _ => a ^ b,
                };
                stack.push(value);
            }
            6 => {
                let idx = inst.operand.unsigned_abs() as usize % locals.len().max(1);
                stack.push(*locals.get(idx).unwrap_or(&0));
            }
            7 => {
                let value = stack.pop().ok_or(RailError::BadScript("stack"))?;
                let idx = inst.operand.unsigned_abs() as usize % locals.len().max(1);
                if let Some(slot) = locals.get_mut(idx) {
                    *slot = value;
                }
            }
            8 => {
                let value = stack.pop().ok_or(RailError::BadScript("stack"))?;
                if value == 0 && !program.instructions.is_empty() {
                    ip = inst.operand.unsigned_abs() as usize % program.instructions.len();
                }
            }
            9 => {
                let value = stack.pop().ok_or(RailError::BadScript("stack"))?;
                report.output ^= checksum::mix_u64(value as u64);
                report.emitted.push(report.output);
            }
            10 => {
                if !program.instructions.is_empty() {
                    calls.push(ip);
                    ip = inst.operand.unsigned_abs() as usize % program.instructions.len();
                }
            }
            11 => {
                if let Some(next) = calls.pop() {
                    ip = next;
                } else {
                    break;
                }
            }
            12 => {
                report.output ^= fastpath::fast_script_snapshot(
                    &mut stack,
                    &mut calls,
                    report.output ^ inst.operand as u64,
                )
            }
            255 => break,
            _ => report.output ^= (inst.opcode as u64) << (inst.line & 31),
        }
    }
    Ok(report)
}

pub fn decode_stream(data: &[u8]) -> Result<Archive> {
    if data.starts_with(b"RLCH") {
        return parse_archive(data);
    }
    let mut archive = Archive::new(Header {
        version: 1,
        flags: 0,
        created_at: 0,
        yard_id: 0,
        section_count: 0,
    });
    let mut buffer = Vec::new();
    if data.starts_with(b"RLST") && data.len() >= 6 {
        let mut pos = 4;
        let count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        for _ in 0..count {
            if pos + 3 > data.len() {
                return Err(RailError::ShortRead {
                    needed: 3,
                    remaining: data.len().saturating_sub(pos),
                });
            }
            let kind = data[pos];
            pos += 1;
            let len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + len > data.len() {
                return Err(RailError::ShortRead {
                    needed: len,
                    remaining: data.len().saturating_sub(pos),
                });
            }
            let payload = &data[pos..pos + len];
            match kind {
                1 => {
                    fastpath::fast_stream_carry(
                        &mut buffer,
                        payload,
                        archive.header.yard_id as u64,
                    );
                }
                2 => archive
                    .sessions
                    .push(parse_session(payload, &archive.dictionary)?),
                3 => archive.journal.extend(parse_journal(payload)?),
                4 => archive.scripts.push(compile_script(payload)?),
                _ => archive.blobs.push(payload.to_vec()),
            }
            pos += len;
        }
    } else {
        fastpath::fast_stream_carry(&mut buffer, data, 0);
    }
    if buffer.starts_with(b"RLCH") {
        parse_archive(&buffer)
    } else {
        archive.blobs.push(buffer);
        Ok(archive)
    }
}
