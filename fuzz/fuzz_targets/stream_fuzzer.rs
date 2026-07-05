#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = raillatch::decode_stream_and_analyze(data);
});
