//  IMPORTS  \\
///////////////////////////////

use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use crate::interrupts::InterruptIndex;
use crate::interrupts::PICS;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::vga_buffer::CursorMode;
use crate::{print, println};

//////////// END //////////////

// Add this lazy_static initialization back
lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
                             layouts::Us104Key,
                             HandleControl::Ignore
    ));
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = port.read(); // Port read is safe in this context as it's the keyboard data port

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    // Handle cursor mode switching
                    match character {
                        // Ctrl+H for Hardware cursor
                        'H' => {
                            crate::vga_buffer::switch_cursor_mode(CursorMode::Hardware);
                            println!("Switched to hardware cursor");
                        },
                        // Ctrl+S for Software cursor
                        'S' => {
                            crate::vga_buffer::switch_cursor_mode(CursorMode::Software);
                            println!("Switched to software cursor");
                        },
                        // Normal character handling
                        _ => {
                            let should_handle = {
                                let writer = crate::vga_buffer::WRITER.lock();
                                let next_pos = if character == '\u{8}' && writer.column_position > 0 {
                                    writer.column_position - 1
                                } else if writer.needs_wrap() {
                                    0 // Allow wrapping to next line
                                } else {
                                    writer.column_position
                                };
                                !writer.protected_region.contains(
                                    if writer.needs_wrap() { writer.row_position + 1 } else { writer.row_position },
                                        next_pos
                                )
                            };

                            if should_handle {
                                // Set color to white for user input
                                crate::vga_buffer::set_color(crate::vga_buffer::Color::White, crate::vga_buffer::Color::Black);

                                match character {
                                    '\u{8}' => crate::vga_buffer::backspace(),
                                    '\n' => {
                                        print!("{}", character);
                                        crate::vga_buffer::write_prompt();
                                    },
                                    _ => print!("{}", character),
                                }
                            }
                        }
                    }
                },
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    PICS.lock()
    .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
}
