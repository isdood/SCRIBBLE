// src/debug.rs
use crate::serial_println;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum DebugLevel {
    Info,
    Warning,
    Error,
    Critical,
}

lazy_static! {
    static ref DEBUG_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref MESSAGE_COUNT: AtomicU64 = AtomicU64::new(0);
}

pub fn log(level: DebugLevel, message: &str) {
    let count = MESSAGE_COUNT.fetch_add(1, Ordering::SeqCst);
    let timestamp = crate::stats::SYSTEM_STATS.lock().get_timer_ticks();

    let log_message = format!(
        "[{:04}][{:08}][{:?}] {}",
        count,
        timestamp,
        level,
        message
    );

    // Store in circular buffer
    if let Some(mut log) = DEBUG_LOG.try_lock() {
        if log.len() >= 1000 { // Keep last 1000 messages
            log.remove(0);
        }
        log.push(log_message.clone());
    }

    // Always output to serial
    serial_println!("{}", log_message);
}

pub fn dump_debug_log() {
    if let Some(log) = DEBUG_LOG.try_lock() {
        serial_println!("=== DEBUG LOG DUMP ===");
        for message in log.iter() {
            serial_println!("{}", message);
        }
        serial_println!("=== END DEBUG LOG ===");
    }
}

// Helper macros
#[macro_export]
macro_rules! debug_critical {
    ($($arg:tt)*) => {{
        let message = {
            use alloc::format;
            format!($($arg)*)
        };
        $crate::debug::log($crate::debug::DebugLevel::Critical, &message)
    }};
}
