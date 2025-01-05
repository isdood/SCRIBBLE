use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use crate::{print, vga_buffer};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

pub fn handle_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if character == '\n' {
                        print!("\n> ");
                    } else {
                        print!("{}", character);
                    }
                }
                DecodedKey::RawKey(key) => {
                    match key {
                        KeyCode::Backspace => {
                            // Try printing a debug message to verify the key is detected
                            serial_println!("Backspace pressed!");
                            vga_buffer::backspace();
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}
