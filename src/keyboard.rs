use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1, DecodedKey};
use spin::Mutex;
use x86_64::instructions::port::Port;

pub struct KeyboardDriver {
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
}

impl KeyboardDriver {
    pub fn new() -> Self {
        KeyboardDriver {
            keyboard: Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore)
        }
    }
}

lazy_static::lazy_static! {
    pub static ref KEYBOARD: Mutex<KeyboardDriver> = Mutex::new(KeyboardDriver::new());
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
