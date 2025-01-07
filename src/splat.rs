use crate::serial_println;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use crate::stats::SYSTEM_STATS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplatLevel {
    Info,
    Warning,
    Error,
    Critical,
}

pub struct SplatEntry {
    timestamp: u64,
    level: SplatLevel,
    message: String,
    timer_ticks: u64,
    keyboard_interrupts: u64,
}

lazy_static! {
    static ref SPLAT_LOG: Mutex<Vec<SplatEntry>> = Mutex::new(Vec::with_capacity(1000));
    static ref MESSAGE_COUNT: AtomicU64 = AtomicU64::new(0);
}

pub fn log(level: SplatLevel, message: &str) {
    let count = MESSAGE_COUNT.fetch_add(1, Ordering::SeqCst);
    let stats = SYSTEM_STATS.lock();
    let timer_ticks = stats.get_timer_ticks();
    let keyboard_interrupts = stats.get_keyboard_interrupts();
    drop(stats); // Release the lock early

    let entry = SplatEntry {
        timestamp: count,
        level,
        message: String::from(message),
        timer_ticks,
        keyboard_interrupts,
    };

    let log_message = format!(
        "[{:04}][Tick:{:08}][KbInt:{:04}][{:?}] {}",
        entry.timestamp,
        entry.timer_ticks,
        entry.keyboard_interrupts,
        entry.level,
        entry.message
    );

    // Store in circular buffer
    if let Some(mut log) = SPLAT_LOG.try_lock() {
        if log.len() >= 1000 {
            log.remove(0);
        }
        log.push(entry);
    }

    // Always output to serial
    serial_println!("{}", log_message);
}

pub fn dump_log() {
    if let Some(log) = SPLAT_LOG.try_lock() {
        serial_println!("=== SPLAT LOG DUMP ===");
        for entry in log.iter() {
            serial_println!(
                "[{:04}][Tick:{:08}][KbInt:{:04}][{:?}] {}",
                entry.timestamp,
                entry.timer_ticks,
                entry.keyboard_interrupts,
                entry.level,
                entry.message
            );
        }
        serial_println!("=== END SPLAT LOG ===");
    }
}

// Helper functions to format system stats
pub fn format_stats() -> String {
    let stats = SYSTEM_STATS.lock();
    format!(
        "Timer:{}, Keyboard:{}",
        stats.get_timer_ticks(),
            stats.get_keyboard_interrupts()
    )
}

// Macros
#[macro_export]
macro_rules! splat {
    ($level:expr, $($arg:tt)*) => {{
        use alloc::format;
        $crate::splat::log($level, &format!($($arg)*))
    }};
}

// Convenience macros
#[macro_export]
macro_rules! splat_info {
    ($($arg:tt)*) => {
        $crate::splat!($crate::splat::SplatLevel::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! splat_warn {
    ($($arg:tt)*) => {
        $crate::splat!($crate::splat::SplatLevel::Warning, $($arg)*)
    };
}

#[macro_export]
macro_rules! splat_error {
    ($($arg:tt)*) => {
        $crate::splat!($crate::splat::SplatLevel::Error, $($arg)*)
    };
}

#[macro_export]
macro_rules! splat_critical {
    ($($arg:tt)*) => {
        $crate::splat!($crate::splat::SplatLevel::Critical, $($arg)*)
    };
}
