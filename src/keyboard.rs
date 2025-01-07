use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use pc_keyboard::{
    layouts, HandleControl, Keyboard, ScancodeSet1,
    DecodedKey, KeyCode, KeyState, Error as KeyboardError,
    KeyEvent
};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::interrupts::{PICS, InterruptIndex};
use crate::splat::{self, SplatLevel};
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::format;

// Constants
const KEYBOARD_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;
const MAX_BUFFER_SIZE: usize = 16;
const BUFFER_SIZE: usize = 16;

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

pub struct KeyboardController {
    device: Keyboard<layouts::Us104Key, ScancodeSet1>,
    buffer: [u8; BUFFER_SIZE],
    buffer_count: usize,
    last_keycode: Option<KeyCode>,
}

impl KeyboardController {
    pub fn new() -> KeyboardController {
        KeyboardController {
            device: Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore),
            buffer: [0; BUFFER_SIZE],
            buffer_count: 0,
            last_keycode: None,
        }
    }

    pub fn add_byte(&mut self, scancode: u8) -> Result<Option<KeyEvent>, KeyboardError> {
        if self.buffer_count >= BUFFER_SIZE {
            ERRORS.fetch_add(1, Ordering::Relaxed);
            return Err(KeyboardError::BufferFull);
        }

        self.buffer[self.buffer_count] = scancode;
        self.buffer_count += 1;
        KEYSTROKES.fetch_add(1, Ordering::Relaxed);

        Ok(self.process_buffer())
    }

    fn process_buffer(&mut self) -> Option<KeyEvent> {
        if self.buffer_count == 0 {
            return None;
        }

        match self.device.add_byte(self.buffer[0]) {
            Ok(Some(event)) => {
                // Shift buffer left
                for i in 1..self.buffer_count {
                    self.buffer[i-1] = self.buffer[i];
                }
                self.buffer_count -= 1;
                Some(event)
            }
            Ok(None) => None,
            Err(e) => {
                ERRORS.fetch_add(1, Ordering::Relaxed);
                splat::log(
                    SplatLevel::Warning,
                    &format!("Keyboard buffer processing error: {:?}", e)
                );
                None
            }
        }
    }

    pub fn process_scancode(&mut self, scancode: u8) -> Result<(), KeyboardError> {
        match self.add_byte(scancode) {
            Ok(Some(event)) => {
                if let Some(key) = self.device.process_keyevent(event.clone()) {
                    self.handle_key_event(&key, event.state);
                }
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => {
                ERRORS.fetch_add(1, Ordering::Relaxed);
                Err(e)
            }
        }
    }

    fn handle_key_event(&mut self, key: &DecodedKey, state: KeyState) {
        match (key, state) {
            (DecodedKey::Unicode(character), KeyState::Down) => {
                splat::log(
                    SplatLevel::BitsNBytes,
                    &format!("Key pressed: {}", character)
                );
            }
            (DecodedKey::RawKey(key_code), state) => {
                self.last_keycode = Some(key_code);
                splat::log(
                    SplatLevel::BitsNBytes,
                    &format!("Raw key {:?}: {:?}", key_code, state)
                );
            }
        }
    }

    pub fn reset_buffer(&mut self) {
        self.buffer_count = 0;
    }
}

lazy_static! {
    pub static ref KEYBOARD: Mutex<KeyboardController> = Mutex::new(
        KeyboardController::new()
    );
}

// Rest of the implementation remains the same...
