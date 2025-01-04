// src/keyboard.rs
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;
use crate::vga_buffer::{Color, set_color};
use crate::{print, println};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

pub fn initialize() {
    // We can remove the unused imports and empty the function body
    // as the initialization is now handled in the interrupts module
}

pub fn add_scancode(scancode: u8) {
    if let Ok(Some(key_event)) = KEYBOARD.lock().add_byte(scancode) {
        if let Some(key) = KEYBOARD.lock().process_keyevent(key_event) {
            handle_keyevent(key);
        }
    }
}

fn handle_keyevent(key: DecodedKey) {
    interrupts::without_interrupts(|| {
        match key {
            DecodedKey::Unicode(character) => {
                match character {
                    '\n' => {
                        println!();
                    },
                    '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' => {
                        set_color(Color::LightCyan, Color::Black);
                        print!("{}", character);
                    },
                    '0'..='9' => {
                        set_color(Color::Yellow, Color::Black);
                        print!("{}", character);
                    },
                    'a'..='z' | 'A'..='Z' => {
                        set_color(Color::LightGreen, Color::Black);
                        print!("{}", character);
                    },
                    _ => {
                        set_color(Color::White, Color::Black);
                        print!("{}", character);
                    }
                }
            },
            DecodedKey::RawKey(key) => {
                set_color(Color::LightRed, Color::Black);
                print!("<{:?}>", key);
            }
        }
    });
}
