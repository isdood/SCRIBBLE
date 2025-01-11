use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1, DecodedKey};
use spin::Mutex;
use x86_64::instructions::port::Port;

pub struct KeyboardController {
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
    last_keycode: Option<DecodedKey>,
}

impl KeyboardController {
    pub fn new() -> Self {
        KeyboardController {
            keyboard: Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore),
            last_keycode: None,
        }
    }

    pub fn add_byte(&mut self, scancode: u8) -> Option<DecodedKey> {
        if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
            if let Some(decoded_key) = self.keyboard.process_keyevent(key_event) {
                self.last_keycode = Some(decoded_key);
                return Some(decoded_key);
            }
        }
        None
    }

    pub fn last_key(&self) -> Option<DecodedKey> {
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
