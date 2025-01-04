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
    match key {
        DecodedKey::Unicode(character) => {
            // Use the Writer directly to maintain the current color
            WRITER.lock().write_byte(character as u8);
        }
        DecodedKey::RawKey(key) => {
            match key {
                KeyCode::ArrowUp => WRITER.lock().write_string("↑"),
                KeyCode::ArrowDown => WRITER.lock().write_string("↓"),
                KeyCode::ArrowLeft => WRITER.lock().write_string("←"),
                KeyCode::ArrowRight => WRITER.lock().write_string("→"),
                _ => (), // Ignore other special keys
            }
        }
    }
}

pub fn initialize() {
    crate::println!("Keyboard initialized");
}
