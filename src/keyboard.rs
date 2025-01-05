use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use crate::{print, vga_buffer};
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
                        print!("\n> ");  // New prompt on new line
                        // Update prompt row using the public methods
                        if let Some(mut writer) = vga_buffer::WRITER.try_lock() {
                            let current_row = writer.get_row_position();
                            writer.set_prompt_row(current_row);
                        }
                    } else if character as u8 == 8 {
                        vga_buffer::backspace();
                    } else {
                        print!("{}", character);
                    }
                }
                DecodedKey::RawKey(KeyCode::Backspace) => {
                    vga_buffer::backspace();
                }
                _ => {}
            }
        }
    }
}
