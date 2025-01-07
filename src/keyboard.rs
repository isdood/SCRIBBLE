//  IMPORTS  \\
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use crate::interrupts::InterruptIndex;
use crate::interrupts::PICS;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::vga_buffer::CursorMode;
use crate::{print, println};
use crate::serial_println;
// END IMPORTS \\

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

    // Debug print to serial
    serial_println!("Keyboard interrupt received");

    let scancode: u8 = unsafe { port.read() };
    serial_println!("Scancode: {}", scancode);

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    // Debug print to serial
                    serial_println!("Key pressed: {}", character);

                    match character {
                        'H' => {
                            serial_println!("Switching to hardware cursor");
                            crate::vga_buffer::switch_cursor_mode(CursorMode::Hardware);
                        },
                        'S' => {
                            serial_println!("Switching to software cursor");
                            crate::vga_buffer::switch_cursor_mode(CursorMode::Software);
                        },
                        _ => {
                            let should_handle = {
                                let writer = crate::vga_buffer::WRITER.lock();
                                let next_pos = if character == '\u{8}' && writer.column_position > 0 {
                                    writer.column_position - 1
                                } else if writer.needs_wrap() {
                                    0
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
                                    _ => {
                                        // Directly write to VGA buffer for debugging
                                        let mut writer = crate::vga_buffer::WRITER.lock();
                                        writer.write_byte(character as u8);
                                    }
                                }
                            }
                        }
                    }
                },
                DecodedKey::RawKey(key) => {
                    serial_println!("Raw key: {:?}", key);
                }
            }
        }
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
