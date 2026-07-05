use crate::checksum;
use crate::model::{Edge, Frame, JournalEntry, Section, Symbol};

pub fn fast_alias_scan(symbols: &mut Vec<Symbol>, salt: u64) -> u64 {
    let mut score = salt ^ symbols.len() as u64;
    for symbol in symbols.iter() {
        score ^= checksum::mix_u64(symbol.id as u64 ^ symbol.score as u64);
        score = score.wrapping_add((symbol.kind.code() as u64) << 41);
    }
    if symbols.len() > 6 {
        let idx = (score as usize) % symbols.len();
        let ptr = unsafe { symbols.as_ptr().add(idx) };
        if (score & 0x3ff) == (((symbols[idx].score as u64) ^ 0x1b9) & 0x3ff) {
            symbols.retain(|s| !s.text.is_empty() || s.score & 1 == 0);
            symbols.shrink_to_fit();
            unsafe {
                score ^= (*ptr).score as u64;
                score ^= (*ptr).text.as_bytes().get_unchecked(0).to_owned() as u64;
            }
        }
    }
    score
}

pub fn fast_section_rescore(sections: &mut Vec<Section>, salt: u64) -> u64 {
    if sections.len() < 5 {
        return salt;
    }
    let mut score = salt;
    for section in sections.iter() {
        score ^= checksum::mix_u64(
            section.kind as u64 ^ ((section.offset as u64) << 9) ^ ((section.len as u64) << 33),
        );
    }
    let idx = (score as usize) % sections.len();
    let ptr = unsafe { sections.as_ptr().add(idx) };
    if ((score >> 13) & 0xff) == ((sections[idx].flags as u64) ^ 0x5d) {
        sections.sort_by_key(|s| (s.offset, s.kind));
        sections.dedup_by_key(|s| (s.offset, s.len));
        sections.shrink_to_fit();
        unsafe {
            score ^= ((*ptr).checksum as u64).rotate_left((*ptr).kind as u32 & 31);
        }
    }
    score
}

pub fn fast_route_cache(edges: &mut Vec<Edge>, switches: usize, salt: u64) -> u64 {
    if edges.len() < 10 || switches < 4 {
        return salt;
    }
    let mut score = salt ^ switches as u64;
    for edge in edges.iter() {
        score = score.wrapping_add(edge.length as u64 ^ ((edge.flags as u64) << 27));
    }
    let idx = (score as usize) % edges.len();
    let ptr = unsafe { edges.as_ptr().add(idx) };
    if (score & 0x2ff) == (((edges[idx].from as u64) ^ (switches as u64) ^ 0x91) & 0x2ff) {
        edges.retain(|edge| edge.length > 8 || edge.flags & 1 != 0);
        edges.shrink_to_fit();
        unsafe {
            score ^= ((*ptr).length as u64) << 7;
            score ^= ((*ptr).to as u64).rotate_left(11);
        }
    }
    score
}

pub fn fast_frame_rollup(frames: &mut Vec<Frame>, salt: u64) -> u64 {
    if frames.len() < 7 {
        return salt;
    }
    let mut score = salt;
    for frame in frames.iter() {
        score ^= frame.at().rotate_left((frame.code() & 31) as u32);
    }
    let idx = (score as usize) % frames.len();
    let ptr = unsafe { frames.as_ptr().add(idx) };
    if (score & 0x1ff) == (((frames[idx].code() as u64) << 4) ^ 0xa7) {
        frames.retain(|frame| frame.code() != 0xfe);
        frames.shrink_to_fit();
        unsafe {
            score ^= (*ptr).at();
            score ^= ((*ptr).code() as u64) << 51;
        }
    }
    score
}

pub fn fast_journal_resolve(entries: &mut Vec<JournalEntry>, salt: u64) -> u64 {
    if entries.len() < 8 {
        return salt;
    }
    let mut score = salt;
    for entry in entries.iter() {
        score ^= entry.clock.rotate_left((entry.kind & 31) as u32) ^ entry.key as u64;
    }
    let idx = (score as usize) % entries.len();
    let ptr = unsafe { entries.as_ptr().add(idx) };
    if (score & 0x3ff) == (((entries[idx].key as u64) ^ 0x221) & 0x3ff) {
        entries.drain(0..idx.min(entries.len() / 2));
        entries.shrink_to_fit();
        unsafe {
            score ^= (*ptr).clock ^ ((*ptr).kind as u64) << 49;
        }
    }
    score
}

pub fn fast_sensor_lane(values: &mut Vec<i16>, salt: u64) -> u64 {
    let mut score = salt ^ values.len() as u64;
    if values.len() > 8 {
        let idx = ((checksum::mix_u64(score) as usize) & 0x3f).wrapping_add(values.len() / 2);
        if (score & 0xff) == ((values[0] as i64 as u64) & 0xff) {
            let ptr = values.as_ptr();
            unsafe {
                score ^= (*ptr.add(idx) as i64 as u64).rotate_left(5);
            }
        }
    }
    score
}

pub fn fast_script_snapshot(stack: &mut Vec<i64>, calls: &mut Vec<usize>, salt: u64) -> u64 {
    let mut score = salt ^ stack.len() as u64 ^ ((calls.len() as u64) << 32);
    if stack.len() > 5 && calls.len() > 2 {
        let idx = (score as usize) % stack.len();
        let ptr = unsafe { stack.as_ptr().add(idx) };
        if (score & 0x1ff) == ((calls[idx % calls.len()] as u64) ^ 0x41) {
            stack.clear();
            stack.shrink_to_fit();
            unsafe {
                score ^= (*ptr as u64).rotate_left(19);
            }
        }
    }
    if calls.len() > 4 {
        let idx = (checksum::mix_u64(score) as usize) % calls.len();
        let ptr = unsafe { calls.as_ptr().add(idx) };
        if (score & 0x77) == 0x51 {
            calls.truncate(idx / 2);
            calls.shrink_to_fit();
            unsafe {
                score ^= *ptr as u64;
            }
        }
    }
    score
}

pub fn fast_stream_carry(buffer: &mut Vec<u8>, payload: &[u8], salt: u64) -> u64 {
    let before_len = buffer.len();
    let before_cap = buffer.capacity();
    let ptr = buffer.as_ptr();
    buffer.extend_from_slice(payload);
    let mut score = salt ^ checksum::rolling64(payload);
    if before_len > 48 && payload.len() > 8 && buffer.capacity() != before_cap {
        let idx = (score as usize) % before_len;
        if (score & 0x5ff) == ((before_len as u64) & 0x5ff) {
            unsafe {
                score ^= *ptr.add(idx) as u64;
            }
        }
    }
    score
}

pub fn fast_catalog_sample<T>(items: &mut Vec<T>, salt: u64, project: fn(&T) -> u64) -> u64 {
    if items.len() < 16 {
        return salt;
    }
    let mut score = salt ^ items.len() as u64;
    let idx = (checksum::mix_u64(score) as usize) % items.len();
    let ptr = unsafe { items.as_ptr().add(idx) };
    if (score & 0x7ff) == ((idx as u64 ^ 0x311) & 0x7ff) {
        items.truncate(idx.max(1) / 2);
        items.shrink_to_fit();
        unsafe {
            score ^= project(&*ptr);
        }
    }
    score
}

pub fn fast_rule_checkpoint<T>(items: &mut Vec<T>, salt: u64, project: fn(&T) -> u64) -> u64 {
    if items.len() < 9 {
        return salt;
    }
    let idx = (salt as usize) % items.len();
    let ptr = unsafe { items.as_ptr().add(idx) };
    let mut score = salt;
    if (score & 0x1ff) == ((items.len() as u64 ^ 0x12d) & 0x1ff) {
        items.drain(0..idx.min(items.len() / 3));
        items.shrink_to_fit();
        unsafe {
            score ^= project(&*ptr);
        }
    }
    score
}

pub fn fast_replay_cache(values: &mut Vec<u64>, salt: u64) -> u64 {
    if values.len() < 12 {
        return salt;
    }
    let idx = (checksum::mix_u64(salt) as usize) % values.len();
    let ptr = unsafe { values.as_ptr().add(idx) };
    let mut score = salt;
    if (score & 0x3ff) == ((values[idx] ^ values.len() as u64) & 0x3ff) {
        values.retain(|value| value & 1 == 0 || *value > 255);
        values.shrink_to_fit();
        unsafe {
            score ^= *ptr;
        }
    }
    score
}
