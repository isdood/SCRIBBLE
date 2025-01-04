// src/keyboard.rs
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts, KeyCode};
use crate::{print, println};
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

pub fn add_scancode(scancode: u8) {
    if let Ok(Some(key_event)) = KEYBOARD.lock().add_byte(scancode) {
        if let Some(key) = KEYBOARD.lock().process_keyevent(key_event) {
            handle_keyevent(key);
        }
    }
}

fn handle_keyevent(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(character) => {
            match character {
                '\n' => println!(),
                '\t' => print!("    "), // 4 spaces for tab
                '\x08' => print!("\x08 \x08"), // Backspace: move back, print space, move back
                character => print!("{}", character),
            }
        },
        DecodedKey::RawKey(key) => {
            match key {
                KeyCode::ArrowUp => print!("↑"),
                KeyCode::ArrowDown => print!("↓"),
                KeyCode::ArrowLeft => print!("←"),
                KeyCode::ArrowRight => print!("→"),
                _ => (), // Ignore other special keys
            }
        }
    }
}

pub fn initialize() {
    println!("Initializing keyboard...");
}
