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
    ($($arg:tt)*) => ({
        $crate::debug::log(DebugLevel::Info, &format!($($arg)*));
    })
}

#[macro_export]
macro_rules! debug_warn {
    ($($arg:tt)*) => ({
        $crate::debug::log(DebugLevel::Warning, &format!($($arg)*));
    })
}

#[macro_export]
macro_rules! debug_error {
    ($($arg:tt)*) => ({
        $crate::debug::log(DebugLevel::Error, &format!($($arg)*));
    })
}
