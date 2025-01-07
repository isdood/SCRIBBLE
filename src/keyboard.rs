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
    unsafe { KEYBOARD_INTERRUPTS += 1 };

    serial_println!("[DEBUG] Keyboard interrupt #{}", unsafe { KEYBOARD_INTERRUPTS });

    // Try to acquire keyboard lock without blocking
    if let Some(mut keyboard) = KEYBOARD.try_lock() {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };

        serial_println!("[DEBUG] Scancode: 0x{:02x}", scancode);

        // Process key event
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                serial_println!("[DEBUG] Processed key: {:?}", key);
                // ... rest of key handling
            }
        }
    } else {
        serial_println!("[WARNING] Keyboard locked, skipping interrupt");
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
