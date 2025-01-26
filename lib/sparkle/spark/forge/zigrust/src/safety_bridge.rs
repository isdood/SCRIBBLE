use std::slice;
use std::ffi::c_void;
use spark_safety::{SafetyChecker, SafetyLevel};

#[no_mangle]
pub extern "C" fn init_safety_bridge() -> i32 {
    // Initialize any required safety systems
    0
}

#[no_mangle]
pub extern "C" fn check_safety(
    code_ptr: *const u8,
    code_len: usize,
    safety_level: i32,
    enable_optimizations: bool,
    check_ownership: bool,
) -> i32 {
    let code = unsafe {
        slice::from_raw_parts(code_ptr, code_len)
    };

    let level = match safety_level {
        0 => SafetyLevel::Calm,
        1 => SafetyLevel::Balanced,
        2 => SafetyLevel::Wild,
        _ => return -1,
    };

    let mut checker = SafetyChecker::new(level);

    // Convert code to string and check safety
    match std::str::from_utf8(code) {
        Ok(code_str) => {
            match checker.check_spell(code_str) {
                Ok(_) => 0,
                Err(_) => -1,
            }
        },
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn get_safety_stats(
    enchantments_count: *mut usize,
    wild_magic_detected: *mut bool,
) -> i32 {
    let checker = SafetyChecker::new(SafetyLevel::Balanced);
    let stats = checker.get_stats();

    unsafe {
        *enchantments_count = stats.enchantments_count;
        *wild_magic_detected = stats.wild_magic_detected;
    }

    0
}
