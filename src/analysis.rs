use crate::catalog;
use crate::checksum;
use crate::fastpath;
use crate::model::{AnalysisFinding, AnalysisReport, Archive, Frame};
use crate::parser;
use crate::rulebook::{self, RuleContext};

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, archive: &Archive) -> AnalysisReport {
        let mut report = AnalysisReport::default();
        report.frame_count = archive.frame_count();
        let mut topology = archive.topology.clone();
        report.topology_score =
            parser::score_topology(&mut topology, archive.dictionary.alias_score);
        let mut journal = archive.journal.clone();
        report.journal_score = parser::replay_journal(&mut journal);
        for program in &archive.scripts {
            if let Ok(exec) = parser::run_script(program) {
                report.script_score ^= exec.output ^ exec.steps;
            }
        }
        for session in &archive.sessions {
            report.checksum ^= session.checksum;
            for frame in &session.frames {
                report.checksum ^= frame_score(frame);
            }
        }
        let mut cache = vec![
            archive.header.created_at,
            archive.header.yard_id as u64,
            report.topology_score,
            report.journal_score,
            report.script_score,
            report.checksum,
            archive.dictionary.alias_score,
            report.frame_count as u64,
            archive.blobs.len() as u64,
            archive.sessions.len() as u64,
            archive.scripts.len() as u64,
            archive.journal.len() as u64,
        ];
        report.checksum ^= fastpath::fast_replay_cache(&mut cache, report.checksum);
        report.checksum ^= catalog::score_catalog(
            report.topology_score ^ archive.dictionary.alias_score,
            archive.header.yard_id,
        );
        let ctx = RuleContext {
            yard_id: archive.header.yard_id as u64,
            symbol_score: archive.dictionary.alias_score,
            topology_score: report.topology_score,
            journal_score: report.journal_score,
            session_score: report.checksum,
            script_score: report.script_score,
            frame_count: report.frame_count as u64,
            section_count: archive.header.section_count as u64,
        };
        for finding in rulebook::evaluate_all(&ctx) {
            report.findings.push(AnalysisFinding {
                severity: finding.severity,
                code: finding.code.to_string(),
                message: format!("rule detail {:#x}", finding.detail),
            });
        }
        if archive.dictionary.symbols.is_empty() {
            report.findings.push(AnalysisFinding {
                severity: 1,
                code: "EMPTY_DICTIONARY".to_string(),
                message: "archive has no symbol dictionary".to_string(),
            });
        }
        report
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn frame_score(frame: &Frame) -> u64 {
    match frame {
        Frame::Track {
            track_id,
            occupied,
            confidence,
            at,
        } => checksum::mix_u64(
            *track_id as u64 ^ *at ^ ((*confidence as u64) << 16) ^ (*occupied as u64),
        ),
        Frame::Switch {
            switch_id,
            target,
            locked,
            at,
        } => {
            checksum::mix_u64(*switch_id as u64 ^ ((*target as u64) << 8) ^ *at ^ (*locked as u64))
        }
        Frame::Route {
            route_id,
            start,
            end,
            authority,
            at,
        } => checksum::mix_u64(
            *route_id as u64
                ^ ((*start as u64) << 7)
                ^ ((*end as u64) << 23)
                ^ ((*authority as u64) << 41)
                ^ *at,
        ),
        Frame::Signal {
            signal_id,
            aspect,
            cause,
            at,
        } => checksum::mix_u64(
            *signal_id as u64 ^ ((*aspect as u64) << 13) ^ ((*cause as u64) << 29) ^ *at,
        ),
        Frame::WorkOrder {
            order_id,
            priority,
            text,
            at,
        } => {
            checksum::rolling64(text.as_bytes())
                ^ *order_id as u64
                ^ ((*priority as u64) << 33)
                ^ *at
        }
        Frame::Sensor {
            track_id,
            readings,
            at,
        } => readings.iter().fold(*track_id as u64 ^ *at, |acc, r| {
            acc.wrapping_add(*r as i64 as u64).rotate_left(3)
        }),
        Frame::Heartbeat { status, at } => *status as u64 ^ *at,
        Frame::Unknown { kind, payload, at } => {
            checksum::rolling64(payload) ^ ((*kind as u64) << 56) ^ *at
        }
    }
}
