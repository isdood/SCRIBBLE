//  IMPORTS  \\
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use crate::interrupts::InterruptIndex;
use crate::interrupts::PICS;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::{debug_info, debug_warn, debug_error};
use crate::{debug_info, debug_warn, stats::SYSTEM_STATS};
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
    SYSTEM_STATS.lock().increment_keyboard();

    debug_info!("Keyboard interrupt #{}", SYSTEM_STATS.lock().get_keyboard_interrupts());

    if let Some(mut keyboard) = KEYBOARD.try_lock() {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };

        debug_info!("Scancode: 0x{:02x}", scancode);

        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                debug_info!("Processed key: {:?}", key);
                // ... rest of key handling
            }
        }
    } else {
        debug_warn!("Keyboard locked, skipping interrupt");
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
