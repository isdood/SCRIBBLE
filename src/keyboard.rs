use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use pc_keyboard::{
    layouts, HandleControl, Keyboard, ScancodeSet1,
    DecodedKey, KeyCode, KeyState, Error as KeyboardError
};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::interrupts::{PICS, InterruptIndex};
use crate::splat::{self, SplatLevel};
use core::sync::atomic::{AtomicU64, Ordering};

// Constants
const KEYBOARD_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;
const MAX_BUFFER_SIZE: usize = 16;

// Statistics tracking
static KEYSTROKES: AtomicU64 = AtomicU64::new(0);
static ERRORS: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub enum KeyboardStatus {
    Ready,
    BufferFull,
    Error(KeyboardError),
    Locked,
}

lazy_static! {
    pub(crate) static ref KEYBOARD: Mutex<KeyboardController> = Mutex::new(
        KeyboardController::new()
    );
}

pub struct KeyboardController {
    device: Keyboard<layouts::Us104Key, ScancodeSet1>,
    last_keycode: Option<KeyCode>,
    buffer_count: usize,
}

impl KeyboardController {
    pub fn new() -> Self {
        splat::log(SplatLevel::BitsNBytes, "Initializing keyboard controller");
        KeyboardController {
            device: Keyboard::new(
                ScancodeSet1::new(),
                                  layouts::Us104Key,
                                  HandleControl::Ignore
            ),
            last_keycode: None,
            buffer_count: 0,
        }
    }

    fn process_scancode(&mut self, scancode: u8) -> Result<Option<DecodedKey>, KeyboardError> {
        if self.buffer_count >= MAX_BUFFER_SIZE {
            splat::log(SplatLevel::Warning, "Keyboard buffer full");
            return Ok(None);
        }

        match self.device.add_byte(scancode) {
            Ok(Some(key_event)) => {
                self.buffer_count += 1;
                match self.device.process_keyevent(key_event) {
                    Some(key) => {
                        self.handle_key_event(&key, key_event.state);
                        Ok(Some(key))
                    },
                    None => Ok(None),
                }
            },
            Ok(None) => Ok(None),
            Err(e) => {
                ERRORS.fetch_add(1, Ordering::Relaxed);
                Err(e)
            }
        }
    }

    fn handle_key_event(&mut self, key: &DecodedKey, state: KeyState) {
        if state == KeyState::Down {
            KEYSTROKES.fetch_add(1, Ordering::Relaxed);
            match key {
                DecodedKey::Unicode(char) => {
                    splat::log(
                        SplatLevel::BitsNBytes,
                        &alloc::format!("Keypress: '{}' (Unicode: {:#x})", char, *char as u32)
                    );
                },
                DecodedKey::RawKey(code) => {
                    self.last_keycode = Some(*code);
                    splat::log(
                        SplatLevel::BitsNBytes,
                        &alloc::format!("Special key: {:?}", code)
                    );
                }
            }
        }
    }

    fn reset_buffer(&mut self) {
        self.buffer_count = 0;
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Record interrupt in statistics
    crate::stat::keyboard_interrupt();

    // Read keyboard status and data
    let status = unsafe { Port::new(KEYBOARD_STATUS_PORT).read() };
    if status & 1 == 0 {
        // No data available
        return;
    }

    let scancode: u8 = unsafe { Port::new(KEYBOARD_PORT).read() };

    // Process keyboard input with error handling
    match KEYBOARD.try_lock() {
        Some(mut controller) => {
            match controller.process_scancode(scancode) {
                Ok(_) => controller.reset_buffer(),
                Err(e) => {
                    splat::log(
                        SplatLevel::Warning,
                        &alloc::format!("Keyboard error: {:?}", e)
                    );
                }
            }
        }
        None => {
            splat::log(
                SplatLevel::Warning,
                "Keyboard controller locked, dropping input"
            );
        }
    }

    // End of interrupt
    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

// Public interface
pub fn get_keyboard_stats() -> (u64, u64) {
    (
        KEYSTROKES.load(Ordering::Relaxed),
     ERRORS.load(Ordering::Relaxed)
    )
}

pub fn get_keyboard_status() -> KeyboardStatus {
    match KEYBOARD.try_lock() {
        Some(controller) => {
            if controller.buffer_count >= MAX_BUFFER_SIZE {
                KeyboardStatus::BufferFull
            } else {
                KeyboardStatus::Ready
            }
        }
        None => KeyboardStatus::Locked
    }
}

pub fn log_keyboard_stats() {
    let (keystrokes, errors) = get_keyboard_stats();
    let reliability = if keystrokes > 0 {
        100.0 * (1.0 - (errors as f64 / keystrokes as f64))
    } else {
        100.0
    };

    splat::log(
        SplatLevel::BitsNBytes,
        &alloc::format!(
            "Keyboard Statistics:\n\
└─ Total Keystrokes: {}\n\
└─ Errors: {}\n\
└─ Reliability: {:.2}%\n\
└─ Status: {:?}",
keystrokes,
errors,
reliability,
get_keyboard_status()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_buffer_limits() {
        let mut controller = KeyboardController::new();
        for _ in 0..MAX_BUFFER_SIZE + 1 {
            controller.process_scancode(0x1E).unwrap(); // 'A' key
        }
        assert_eq!(controller.buffer_count, MAX_BUFFER_SIZE);
    }
}
