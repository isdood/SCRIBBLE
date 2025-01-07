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
