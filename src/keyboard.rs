use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::interrupts::PICS;
use crate::interrupts::InterruptIndex;
use crate::stats::SYSTEM_STATS;
use crate::{splat_info, splat_warn};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(
        Keyboard::new(
            ScancodeSet1::new(),
                      layouts::Us104Key,
                      HandleControl::Ignore
        )
    );
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    // Quick increment without holding lock
    {
        SYSTEM_STATS.lock().increment_keyboard();
    }

    // Process keyboard input only if we can get the lock immediately
    if let Some(mut keyboard) = KEYBOARD.try_lock() {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(_key) = keyboard.process_keyevent(key_event) {
                // Handle key event if needed
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
