// src/keyboard.rs
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;
use crate::vga_buffer::{Color, set_color, WRITER};
use crate::{print, println};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

pub fn initialize() {
    use x86_64::instructions::port::Port;
    use pic8259::ChainedPics;
    use spin::Mutex;
    use crate::interrupts::PICS;

    unsafe {
        PICS.lock()
        .initialize();
    }
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(Some(key_event)) = KEYBOARD.lock().add_byte(scancode) {
        if let Some(key) = KEYBOARD.lock().process_keyevent(key_event) {
            handle_keyevent(key);
        }
    }
}

fn handle_keyevent(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(character) => {
            interrupts::without_interrupts(|| {
                // Set color before printing each character
                match character {
                    '\n' => {
                        println!();
                    },
                    // Special characters in cyan
                    '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' => {
                        set_color(Color::LightCyan, Color::Black);
                        print!("{}", character);
                    },
                    // Numbers in yellow
                    '0'..='9' => {
                        set_color(Color::Yellow, Color::Black);
                        print!("{}", character);
                    },
                    // Letters in light green
                    'a'..='z' | 'A'..='Z' => {
                        set_color(Color::LightGreen, Color::Black);
                        print!("{}", character);
                    },
                    // Spaces and other characters in white
                    _ => {
                        set_color(Color::White, Color::Black);
                        print!("{}", character);
                    }
                }
            });
        }
        DecodedKey::RawKey(key) => {
            interrupts::without_interrupts(|| {
                set_color(Color::LightRed, Color::Black);
                print!("<{:?}>", key);
            });
        }
    }
}
