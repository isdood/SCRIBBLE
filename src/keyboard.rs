use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::interrupts::{InterruptIndex, PICS};
use x86_64::structures::idt::InterruptStackFrame;

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
                            crate::println!();
                            crate::show_datetime();
                            crate::print!("> ");
                        }
                        '\u{0008}' => { // Backspace character
                            crate::vga_buffer::backspace();
                        }
                        _ => {
                            crate::print!("{}", character);
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

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::DecodedKey;

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            let mut writer = WRITER.lock();
            match key {
                DecodedKey::Unicode(character) => {
                    match character {
                        '\n' => {
                            println!();
                        },
                        '\u{8}' => { // Backspace
                            writer.backspace();
                        },
                        _ => {
                            print!("{}", character);
                        }
                    }
                },
                DecodedKey::RawKey(_key) => {
                    // Handle raw keys if needed
                }
            }
        }
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
