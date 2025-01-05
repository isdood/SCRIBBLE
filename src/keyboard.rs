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
    static mut COMMAND_BUFFER: [u8; 256] = [0; 256];
    static mut BUFFER_POS: usize = 0;

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    match character {
                        '\n' => {
                            unsafe {
                                COMMAND_BUFFER[BUFFER_POS] = 0;
                                let command = core::str::from_utf8_unchecked(
                                    &COMMAND_BUFFER[..BUFFER_POS]
                                ).trim();

                                if command == "datetime" {
                                    crate::show_datetime();
                                }

                                BUFFER_POS = 0;
                            }
                            let mut writer = WRITER.lock();
                            let current_row = writer.get_row_position();
                            writer.set_prompt_row(current_row + 1);
                            drop(writer);
                            print!("\n> ");
                        }
                        '\u{0008}' => {
                            unsafe {
                                if BUFFER_POS > 0 {
                                    BUFFER_POS -= 1;
                                }
                            }
                            crate::vga_buffer::backspace();
                        }
                        _ => {
                            unsafe {
                                if BUFFER_POS < 255 {
                                    COMMAND_BUFFER[BUFFER_POS] = character as u8;
                                    BUFFER_POS += 1;
                                }
                            }
                            print!("{}", character);
                        }
                    }
                }
                DecodedKey::RawKey(KeyCode::Backspace) => {
                    unsafe {
                        if BUFFER_POS > 0 {
                            BUFFER_POS -= 1;
                        }
                    }
                    crate::vga_buffer::backspace();
                }
                _ => {}
            }
        }
    }
}
