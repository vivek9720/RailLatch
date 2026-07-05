#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = raillatch::analyze_topology_bytes(data);
});
