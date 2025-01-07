use crate::serial_println;

#[derive(Debug, Clone, Copy)]
pub enum DebugLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: DebugLevel, message: &str) {
    serial_println!("[{:?}] {}", level, message);
}

#[macro_export]
macro_rules! debug_info {
    ($($arg:tt)*) => {{
        use alloc::format;
        $crate::debug::log($crate::debug::DebugLevel::Info, &format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! debug_warn {
    ($($arg:tt)*) => {{
        use alloc::format;
        $crate::debug::log($crate::debug::DebugLevel::Warning, &format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! debug_error {
    ($($arg:tt)*) => {{
        use alloc::format;
        $crate::debug::log($crate::debug::DebugLevel::Error, &format!($($arg)*))
    }};
}
