// src/keyboard.rs
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use spin::Mutex;
use crate::{print, vga_buffer};
use crate::vga_buffer::{Color, WRITER};

lazy_static::lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                             HandleControl::Ignore)
    );
}

pub fn add_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock(); // Lock the keyboard once

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            // Set color to green for user input
            vga_buffer::set_color(Color::Green, Color::Black);

            match key {
                DecodedKey::Unicode(character) => {
                    print!("{}", character);
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

pub fn initialize() {
    // No special initialization needed
}
