use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::PIC_1_OFFSET;
use crate::print;

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
                    // Get current writer position to check protection
                    let should_handle = {
                        let writer = crate::vga_buffer::WRITER.lock();
                        !writer.protected_region.contains(writer.row_position,
                                                          if character == '\u{8}' && writer.column_position > 0 {
                                                              writer.column_position - 1
                                                          } else {
                                                              writer.column_position
                                                          })
                    };

                    if should_handle {
                        if character == '\u{8}' {
                            crate::vga_buffer::backspace();
                        } else {
                            print!("{}", character);
                            if character == '\n' {
                                crate::vga_buffer::write_prompt();
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
