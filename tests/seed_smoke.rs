#[test]
fn archive_seed_decodes_and_analyzes() {
    let data = include_bytes!("../fuzz/corpus/archive_fuzzer/seed_archive.rlch");
    let report = raillatch::decode_and_analyze_archive(data).expect("archive seed should parse");
    assert!(report.frame_count > 0);
}

#[test]
fn entrypoint_seeds_are_wired_to_library_code() {
    let stream = include_bytes!("../fuzz/corpus/stream_fuzzer/seed_stream.rlst");
    let script = include_bytes!("../fuzz/corpus/script_fuzzer/seed_script.rlsc");
    let topology = include_bytes!("../fuzz/corpus/topology_fuzzer/seed_topology.rlto");
    let journal = include_bytes!("../fuzz/corpus/journal_fuzzer/seed_journal.rljr");

    let stream_report = raillatch::decode_stream_and_analyze(stream).expect("stream seed");
    let script_report = raillatch::run_script_bytes(script).expect("script seed");
    let topology_score = raillatch::analyze_topology_bytes(topology).expect("topology seed");
    let journal_score = raillatch::replay_journal_bytes(journal).expect("journal seed");

    assert!(stream_report.frame_count > 0);
    assert!(script_report.steps > 0);
    assert_ne!(topology_score, 0);
    assert_ne!(journal_score, 0);
}
