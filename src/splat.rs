// src/splat.rs

use spin::Mutex;
use alloc::{string::String, vec::Vec};
use core::fmt::Write;
use lazy_static::lazy_static;
use x86_64::registers::rflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplatLevel {
    Critical,
    Warning,
    BitsNBytes,
    Info,
    Debug,
}

#[derive(Debug)]
struct LogEntry {
    level: SplatLevel,
    message: String,
    timestamp: u64,
}

const MAX_LOG_ENTRIES: usize = 1000;

lazy_static! {
    static ref LOG_BUFFER: Mutex<Vec<LogEntry>> = Mutex::new(Vec::with_capacity(MAX_LOG_ENTRIES));
}

pub fn log(level: SplatLevel, message: &str) {
    if let Some(mut buffer) = LOG_BUFFER.try_lock() {
        let entry = LogEntry {
            level,
            message: String::from(message),
            timestamp: get_timestamp(),
        };

        if buffer.len() >= MAX_LOG_ENTRIES {
            buffer.remove(0);
        }
        buffer.push(entry);

        // Print to serial and VGA buffer if available
        if let Some(mut serial) = crate::serial::SERIAL1.try_lock() {
            let _ = writeln!(serial, "[{:?}] {}", level, message);
        }
        if let Some(mut vga) = crate::vga_buffer::WRITER.try_lock() {
            let _ = writeln!(vga, "[{:?}] {}", level, message);
        }
    }
}

#[macro_export]
macro_rules! define_splat_macro {
    ($name:ident, $level:expr) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                $crate::splat::log($level, "")
            };
            ($format_str:expr) => {
                $crate::splat::log($level, $format_str)
            };
            ($format_str:expr, $($arg:tt)*) => {
                $crate::splat::log($level, &alloc::format!($format_str, $($arg)*))
            };
        }
    }
}

// Define the macros
define_splat_macro!(splat_critical, SplatLevel::Critical);
define_splat_macro!(splat_warn, SplatLevel::Warning);
define_splat_macro!(splat_bitsnbytes, SplatLevel::BitsNBytes);
define_splat_macro!(splat_info, SplatLevel::Info);
define_splat_macro!(splat_debug, SplatLevel::Debug);

fn get_timestamp() -> u64 {
    rflags::read_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels() {
        log(SplatLevel::Info, "Test message");
        let buffer = LOG_BUFFER.lock();
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0].level, SplatLevel::Info);
    }
}
