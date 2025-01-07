use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;
use alloc::{string::String, vec::Vec, format};
use lazy_static::lazy_static;
use crate::rtc::DateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SplatLevel {
    Critical,
    BitsNBytes,
    Warning,
    Info,
    Debug,
}

impl SplatLevel {
    fn as_str(&self) -> &'static str {
        match self {
            SplatLevel::Critical => "CRITICAL",
            SplatLevel::BitsNBytes => "BITS&BYTES",
            SplatLevel::Warning => "WARNING",
            SplatLevel::Info => "INFO",
            SplatLevel::Debug => "DEBUG",
        }
    }

    fn as_color(&self) -> &'static str {
        match self {
            SplatLevel::Critical => "\x1b[1;31m",    // Bright Red
            SplatLevel::BitsNBytes => "\x1b[1;36m",  // Bright Cyan
            SplatLevel::Warning => "\x1b[1;33m",     // Bright Yellow
            SplatLevel::Info => "\x1b[1;32m",        // Bright Green
            SplatLevel::Debug => "\x1b[1;37m",       // Bright White
        }
    }
}

#[derive(Debug)]
pub struct LogEntry {
    timestamp: DateTime,
    level: SplatLevel,
    message: String,
    module: String,
    caller: String,
}

const MAX_LOG_ENTRIES: usize = 1000;
const MODULE_SEPARATOR: &str = "::";

lazy_static! {
    static ref LOG_BUFFER: Mutex<Vec<LogEntry>> = Mutex::new(Vec::with_capacity(MAX_LOG_ENTRIES));
}

static LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn log(level: SplatLevel, message: &str) {
    let timestamp = DateTime::now();
    let (module, caller) = get_caller_info();

    let entry = LogEntry {
        timestamp,
        level,
        message: String::from(message),
        module,
        caller,
    };

    // Print to serial/screen
    print_log_entry(&entry);

    // Store in buffer with rotation
    if let Some(mut buffer) = LOG_BUFFER.try_lock() {
        if buffer.len() >= MAX_LOG_ENTRIES {
            buffer.remove(0);

            // Log buffer rotation event
            if level != SplatLevel::Debug {
                let rotation_entry = LogEntry {
                    timestamp: DateTime::now(),
                    level: SplatLevel::Warning,
                    message: format!("Log buffer rotated, {} entries cleared", MAX_LOG_ENTRIES),
                    module: String::from("splat"),
                    caller: String::from("log_rotation"),
                };
                print_log_entry(&rotation_entry);
            }
        }
        buffer.push(entry);
        LOG_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    // Update statistics
    crate::stat::log_event(level);
}

fn get_caller_info() -> (String, String) {
    let mut frames = backtrace::Backtrace::new().frames().to_vec();
    frames.reverse();

    for frame in frames {
        if let Some(symbol) = frame.symbols().get(0) {
            if let Some(name) = symbol.name() {
                let name = name.to_string();
                if !name.contains("splat") && !name.contains("log") {
                    let parts: Vec<&str> = name.split(MODULE_SEPARATOR).collect();
                    let module = parts.first().unwrap_or(&"unknown").to_string();
                    let caller = parts.last().unwrap_or(&"unknown").to_string();
                    return (module, caller);
                }
            }
        }
    }

    (String::from("unknown"), String::from("unknown"))
}

fn print_log_entry(entry: &LogEntry) {
    use core::fmt::Write;
    if let Some(mut serial) = crate::serial::SERIAL1.try_lock() {
        let _ = writeln!(
            serial,
            "{}[{} {} {}::{} ] {}\x1b[0m",
            entry.level.as_color(),
                         entry.timestamp.to_string(),
                         entry.level.as_str(),
                         entry.module,
                         entry.caller,
                         entry.message
        );
    }
}

#[macro_export]
macro_rules! define_splat_macro {
    ($name:ident, $level:expr) => {
        macro_rules! $name {
            ($($arg:tt)*) => ({
                $crate::splat::log($level, &alloc::format!($($arg)*))
            })
        }
    }
}

define_splat_macro!(splat_critical, SplatLevel::Critical);
define_splat_macro!(splat_bitsnbytes, SplatLevel::BitsNBytes);
define_splat_macro!(splat_warn, SplatLevel::Warning);
define_splat_macro!(splat_info, SplatLevel::Info);
define_splat_macro!(splat_debug, SplatLevel::Debug);

pub fn dump_log() {
    if let Some(buffer) = LOG_BUFFER.try_lock() {
        println!("\n=== Log Dump ({} entries) ===", buffer.len());
        for entry in buffer.iter() {
            print_log_entry(entry);
        }
        println!("=== End Log Dump ===\n");
    }
}

pub fn get_log_stats() -> (usize, usize) {
    if let Some(buffer) = LOG_BUFFER.try_lock() {
        (LOG_COUNT.load(Ordering::Relaxed), buffer.len())
    } else {
        (LOG_COUNT.load(Ordering::Relaxed), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splat_levels_ordering() {
        assert!(SplatLevel::Critical > SplatLevel::BitsNBytes);
        assert!(SplatLevel::BitsNBytes > SplatLevel::Warning);
        assert!(SplatLevel::Warning > SplatLevel::Info);
        assert!(SplatLevel::Info > SplatLevel::Debug);
    }
}
