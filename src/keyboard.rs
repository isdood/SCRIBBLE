use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, KeyCode, ScancodeSet1};
use crate::print;
use crate::vga_buffer::WRITER;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                             HandleControl::Ignore)
    );
}

pub fn initialize() {
    // Initialize keyboard-related functionality here if needed
    // For now, we just ensure the KEYBOARD static is initialized
    let _ = KEYBOARD.lock();
}

pub fn add_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if character == '\u{0008}' {  // Backspace character
                        WRITER.lock().backspace();
                    } else {
                        print!("{}", character);
                    }
                },
                DecodedKey::RawKey(key) => {
                    match key {
                        KeyCode::ArrowUp => print!("^"),
                        KeyCode::ArrowDown => print!("v"),
                        KeyCode::ArrowLeft => print!("<"),
                        KeyCode::ArrowRight => print!(">"),
                        _ => print!("{:?}", key),
                    }
                },
            }
        }
    }
}
