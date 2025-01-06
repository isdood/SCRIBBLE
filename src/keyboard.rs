// In keyboard.rs
//IMPORTS\\
/////////////////////////////////
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::{PICS, InterruptIndex};
use crate::print;
use crate::println;
use crate::vga_buffer::CursorMode;  // Add this import

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    use x86_64::instructions::port::Port;

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    // Handle cursor mode switching
                    match character {
                        // Ctrl+H for Hardware cursor (you'll need to press H)
                        'H' => {
                            crate::vga_buffer::switch_cursor_mode(CursorMode::Hardware);
                            println!("Switched to hardware cursor");
                        },
                        // Ctrl+S for Software cursor (you'll need to press S)
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

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
