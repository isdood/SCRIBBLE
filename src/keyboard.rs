//  IMPORTS  \\
use pc_keyboard::{HandleControl, Keyboard, ScancodeSet1, layouts};
use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::InterruptIndex;
use crate::interrupts::PICS;
use crate::stats::SYSTEM_STATS;
use crate::{debug_info, debug_warn};
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
        // ... rest of the handler
    } else {
        debug_warn!("Keyboard locked, skipping interrupt");
    }
}
