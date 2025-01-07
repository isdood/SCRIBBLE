use crate::serial_println;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

#[derive(Debug, Clone, Copy)]
pub enum SplatLevel {
    Info,
    Warning,
    Error,
    Critical,
}

lazy_static! {
    static ref SPLAT_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref MESSAGE_COUNT: AtomicU64 = AtomicU64::new(0);
}

pub fn log(level: SplatLevel, message: &str) {
    let count = MESSAGE_COUNT.fetch_add(1, Ordering::SeqCst);
    let timestamp = crate::stats::SYSTEM_STATS.lock().get_timer_ticks();

    let log_message = format!(
        "[{:04}][{:08}][{:?}] {}",
        count,
        timestamp,
        level,
        message
    );

    if let Some(mut log) = SPLAT_LOG.try_lock() {
        if log.len() >= 1000 {
            log.remove(0);
        }
        log.push(log_message.clone());
    }

    serial_println!("{}", log_message);
}

pub fn dump_splat_log() {
    if let Some(log) = SPLAT_LOG.try_lock() {
        serial_println!("=== SPLAT LOG DUMP ===");
        for message in log.iter() {
            serial_println!("{}", message);
        }
        serial_println!("=== END SPLAT LOG ===");
    }
}

#[macro_export]
macro_rules! splat_info {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::splat::log($crate::splat::SplatLevel::Info, &message)
    }};
}

#[macro_export]
macro_rules! splat_warn {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::splat::log($crate::splat::SplatLevel::Warning, &message)
    }};
}

#[macro_export]
macro_rules! splat_error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::splat::log($crate::splat::SplatLevel::Error, &message)
    }};
}

#[macro_export]
macro_rules! splat_critical {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        $crate::splat::log($crate::splat::SplatLevel::Critical, &message)
    }};
}
