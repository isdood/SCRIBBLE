// src/keyboard.rs
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts, KeyCode};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::vga_buffer::{WRITER, get_current_color};

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
    let current_color = get_current_color();
    let mut writer = WRITER.lock();

    match key {
        DecodedKey::Unicode(character) => {
            writer.write_byte_colored(character as u8, current_color);
        }
        DecodedKey::RawKey(key) => {
            match key {
                KeyCode::ArrowUp => {
                    writer.write_byte_colored(b'↑', current_color);
                }
                KeyCode::ArrowDown => {
                    writer.write_byte_colored(b'↓', current_color);
                }
                KeyCode::ArrowLeft => {
                    writer.write_byte_colored(b'←', current_color);
                }
                KeyCode::ArrowRight => {
                    writer.write_byte_colored(b'→', current_color);
                }
                _ => (), // Ignore other special keys
            }
        }
    }
}

pub fn initialize() {
    crate::println!("Keyboard initialized");
}
