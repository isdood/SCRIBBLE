use crate::print;
use crate::vga_buffer::WRITER;
use pc_keyboard::{DecodedKey, KeyCode};
use spin::Mutex;

pub fn handle_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if character == '\n' {
                        let mut writer = WRITER.lock();
                        let current_row = writer.get_row_position();
                        writer.set_prompt_row(current_row + 1);
                        drop(writer); // Release the lock before printing
                        print!("\n> ");
                    } else if character as u8 == 8 {
                        crate::vga_buffer::backspace();
                    } else {
                        print!("{}", character);
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
