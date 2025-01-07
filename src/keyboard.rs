// src/keyboard.rs
use pc_keyboard::{layouts, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

#[derive(Debug)]
pub enum KeyboardError {
    BufferFull,
    NoData,
}

pub struct KeyboardController {
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
    last_keycode: Option<KeyCode>,
}

impl KeyboardController {
    pub fn new() -> Self {
        KeyboardController {
            keyboard: Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore),
            last_keycode: None,
        }
    }

    pub fn add_byte(&mut self, scancode: u8) -> Option<KeyCode> {
        if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
            self.last_keycode = Some(key_event.code);
            Some(key_event.code)
        } else {
            None
        }
    }

    pub fn last_key(&self) -> Option<KeyCode> {
        self.last_keycode
    }
}

lazy_static::lazy_static! {
    pub static ref KEYBOARD: Mutex<KeyboardController> = Mutex::new(KeyboardController::new());
}

pub fn handle_keyboard_interrupt() {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    if let Some(mut keyboard) = KEYBOARD.try_lock() {
        if let Some(key) = keyboard.add_byte(scancode) {
            use crate::splat::SplatLevel;
            crate::splat::log(SplatLevel::BitsNBytes, &alloc::format!("Key Event: {:?}", key));
        }
    }
}
