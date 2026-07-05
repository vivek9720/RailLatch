//! RailLatch decodes disconnected railway interlocking replay archives.

pub mod analysis;
pub mod catalog;
pub mod checksum;
pub mod cursor;
pub mod error;
pub mod fastpath;
pub mod model;
pub mod parser;
pub mod rulebook;

pub use analysis::Analyzer;
pub use error::{RailError, Result};
pub use model::{AnalysisFinding, AnalysisReport, Archive, Header, Session, Topology};

pub fn parse_archive(data: &[u8]) -> Result<Archive> {
    parser::parse_archive(data)
}

pub fn decode_and_analyze_archive(data: &[u8]) -> Result<AnalysisReport> {
    let archive = parse_archive(data)?;
    Ok(Analyzer::new().analyze(&archive))
}

pub fn decode_stream(data: &[u8]) -> Result<Archive> {
    parser::decode_stream(data)
}

pub fn decode_stream_and_analyze(data: &[u8]) -> Result<AnalysisReport> {
    let archive = decode_stream(data)?;
    Ok(Analyzer::new().analyze(&archive))
}

pub fn run_script_bytes(data: &[u8]) -> Result<parser::ExecutionReport> {
    let program = parser::compile_script(data)?;
    parser::run_script(&program)
}

pub fn replay_journal_bytes(data: &[u8]) -> Result<u64> {
    let mut journal = parser::parse_journal(data)?;
    Ok(parser::replay_journal(&mut journal))
}

pub fn analyze_topology_bytes(data: &[u8]) -> Result<u64> {
    let dict = model::Dictionary::default();
    let mut topology = parser::parse_topology(data, &dict)?;
    Ok(parser::score_topology(&mut topology, dict.alias_score))
}
