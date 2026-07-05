#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = raillatch::run_script_bytes(data);
});
