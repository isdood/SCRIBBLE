#![no_main]
use libfuzzer_sys::fuzz_target;
use lazuline::prelude::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(vec) = std::str::from_utf8(data) {
        let _ = Lazuline::new()
            .unwrap()
            .process_raw(vec)
            .map(|r| r.is_ok());
    }
});
