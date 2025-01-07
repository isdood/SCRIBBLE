use crate::serial_println;
use alloc::format;  // Move format import here

#[derive(Debug, Clone, Copy)]
pub enum DebugLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: DebugLevel, message: &str) {
    match level {
        DebugLevel::Info => serial_println!("[INFO] {}", message),
        DebugLevel::Warning => serial_println!("[WARN] {}", message),
        DebugLevel::Error => serial_println!("[ERROR] {}", message)
    }
}

#[macro_export]
macro_rules! debug_info {
    ($($arg:tt)*) => {
        $crate::debug::log($crate::debug::DebugLevel::Info, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug_warn {
    ($($arg:tt)*) => {
        $crate::debug::log($crate::debug::DebugLevel::Warning, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug_error {
    ($($arg:tt)*) => {
        $crate::debug::log($crate::debug::DebugLevel::Error, &format!($($arg)*))
    };
}
