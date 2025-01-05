use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use crate::{print, vga_buffer, serial_println};
use spin::Mutex;
use lazy_static::lazy_static;

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
                        print!("\n");  // Just newline, no prompt
                    } else if character as u8 == 8 {  // ASCII backspace
                        vga_buffer::backspace();
                    } else {
                        print!("{}", character);
                    }
                }
                DecodedKey::RawKey(key) => {
                    match key {
                        KeyCode::Backspace => {
                            vga_buffer::backspace();
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}
