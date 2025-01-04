// src/keyboard.rs
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts, KeyCode};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::vga_buffer::WRITER;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode));
}

pub fn add_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            handle_keyevent(key);
        }
    }
}

fn handle_keyevent(key: DecodedKey) {
    let mut writer = WRITER.lock();
    match key {
        DecodedKey::Unicode(character) => {
            writer.write_byte(character as u8);
        }
        DecodedKey::RawKey(key) => {
            match key {
                KeyCode::ArrowUp => writer.write_string("^"),
                KeyCode::ArrowDown => writer.write_string("v"),
                KeyCode::ArrowLeft => writer.write_string("<"),
                KeyCode::ArrowRight => writer.write_string(">"),
                _ => (), // Ignore other special keys
            }
        }
    }
}
