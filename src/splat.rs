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
        if let Some(mut writer) = crate::vga_buffer::WRITER.try_lock() {
            let _ = writeln!(writer, "[{:?}] {}", level, message);
        }
    }
}

#[macro_export]
macro_rules! splat_critical {
    () => ($crate::splat::log($crate::splat::SplatLevel::Critical, ""));
    ($s:expr) => ($crate::splat::log($crate::splat::SplatLevel::Critical, $s));
    ($s:expr, $($arg:tt)+) => ($crate::splat::log(
        $crate::splat::SplatLevel::Critical,
        &alloc::format!($s, $($arg)+)
    ));
}

#[macro_export]
macro_rules! splat_warn {
    () => ($crate::splat::log($crate::splat::SplatLevel::Warning, ""));
    ($s:expr) => ($crate::splat::log($crate::splat::SplatLevel::Warning, $s));
    ($s:expr, $($arg:tt)+) => ($crate::splat::log(
        $crate::splat::SplatLevel::Warning,
        &alloc::format!($s, $($arg)+)
    ));
}

#[macro_export]
macro_rules! splat_bitsnbytes {
    () => ($crate::splat::log($crate::splat::SplatLevel::BitsNBytes, ""));
    ($s:expr) => ($crate::splat::log($crate::splat::SplatLevel::BitsNBytes, $s));
    ($s:expr, $($arg:tt)+) => ($crate::splat::log(
        $crate::splat::SplatLevel::BitsNBytes,
        &alloc::format!($s, $($arg)+)
    ));
}

#[macro_export]
macro_rules! splat_info {
    () => ($crate::splat::log($crate::splat::SplatLevel::Info, ""));
    ($s:expr) => ($crate::splat::log($crate::splat::SplatLevel::Info, $s));
    ($s:expr, $($arg:tt)+) => ($crate::splat::log(
        $crate::splat::SplatLevel::Info,
        &alloc::format!($s, $($arg)+)
    ));
}

#[macro_export]
macro_rules! splat_debug {
    () => ($crate::splat::log($crate::splat::SplatLevel::Debug, ""));
    ($s:expr) => ($crate::splat::log($crate::splat::SplatLevel::Debug, $s));
    ($s:expr, $($arg:tt)+) => ($crate::splat::log(
        $crate::splat::SplatLevel::Debug,
        &alloc::format!($s, $($arg)+)
    ));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        if let Some(mut serial) = $crate::serial::SERIAL1.try_lock() {
            use core::fmt::Write;
            let _ = write!(serial, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

fn get_timestamp() -> u64 {
    rflags::read_raw()
}
