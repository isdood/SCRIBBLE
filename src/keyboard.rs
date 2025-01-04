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
    if let Ok(Some(key_event)) = KEYBOARD.lock().add_byte(scancode) {
        if let Some(key) = KEYBOARD.lock().process_keyevent(key_event) {
            // Set color to green for user input
            vga_buffer::set_color(Color::Green, Color::Black);

            let mut writer = WRITER.lock();

            match key {
                DecodedKey::Unicode(character) => {
                    print!("{}", character);
                },
                DecodedKey::RawKey(key) => {
                    match key {
                        KeyCode::ArrowUp => writer.write_string("^"),
                        KeyCode::ArrowDown => writer.write_string("v"),
                        KeyCode::ArrowLeft => writer.write_string("<"),
                        KeyCode::ArrowRight => writer.write_string(">"),
                        _ => print!("{:?}", key),
                    }
                },
            }
        }
    }
}

pub fn initialize() {
    // Initialization code remains the same
}
