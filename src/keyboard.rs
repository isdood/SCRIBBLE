use x86_64::structures::idt::InterruptStackFrame;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::{print, println};

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = x86_64::instructions::port::Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    match character {
                        '\n' => {
                            println!();
                            crate::vga_buffer::set_input_mode(true);
                        },
                        '\u{8}' => { // Backspace
                            crate::vga_buffer::backspace();
                        },
                        _ => {
                            print!("{}", character);
                        }
                    }
                },
                DecodedKey::RawKey(_key) => {}
            }
        }
    }

    unsafe {
        crate::interrupts::PICS.lock()
        .notify_end_of_interrupt(PIC_1_OFFSET + 1); // Use direct offset instead of as_u8()
    }
}
