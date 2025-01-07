//  IMPORTS  \\
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use crate::interrupts::InterruptIndex;
use crate::interrupts::PICS;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::debug::DebugLevel;
use alloc::format;

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
    use crate::{debug_info, debug_error};

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    debug_info!("Keyboard interrupt received");

    let scancode: u8 = unsafe { port.read() };
    debug_info!("Scancode: {}", scancode);

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    debug_info!("Unicode key pressed: {}", character);
                    // ... rest of the handler ...
                },
                DecodedKey::RawKey(key) => {
                    debug_info!("Raw key pressed: {:?}", key);
                }
            }
        }
    } else {
        debug_error!("Failed to process keyboard input");
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
