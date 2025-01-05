//
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use crate::{print, vga_buffer::WRITER};  // Import WRITER directly
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
                    match character {
                        '\n' => {
                            let mut writer = WRITER.lock();
                            let current_row = writer.get_row_position();
                            writer.set_prompt_row(current_row + 1);
                            drop(writer);  // Explicitly drop the lock before calling print!
                            print!("\n> ");
                        }
                        '\u{0008}' => { // Backspace character
                            crate::vga_buffer::backspace();
                        }
                        _ => {
                            print!("{}", character);
                        }
                    }
                }
                DecodedKey::RawKey(KeyCode::Backspace) => {
                    crate::vga_buffer::backspace();
                }
                _ => {}
            }
        }
    }
}
