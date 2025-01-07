use crate::serial_println;

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
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let message = {
            use alloc::string::String;
            use alloc::format;
            format!($($arg)*)
        };
        $crate::debug::log($crate::debug::DebugLevel::Info, &message)
    }};
}

#[macro_export]
macro_rules! debug_warn {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let message = {
            use alloc::string::String;
            use alloc::format;
            format!($($arg)*)
        };
        $crate::debug::log($crate::debug::DebugLevel::Warning, &message)
    }};
}

#[macro_export]
macro_rules! debug_error {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let message = {
            use alloc::string::String;
            use alloc::format;
            format!($($arg)*)
        };
        $crate::debug::log($crate::debug::DebugLevel::Error, &message)
    }};
}
