// src/keyboard.rs
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts, KeyCode};
use spin::Mutex;
use lazy_static::lazy_static;

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
            crate::print!("{}", character);
        }
        DecodedKey::RawKey(key) => {
            match key {
                KeyCode::ArrowUp => crate::print!("↑"),
                KeyCode::ArrowDown => crate::print!("↓"),
                KeyCode::ArrowLeft => crate::print!("←"),
                KeyCode::ArrowRight => crate::print!("→"),
                _ => (), // Ignore other special keys
            }
        }
    }
}

pub fn initialize() {
    crate::println!("Keyboard initialized");
}
