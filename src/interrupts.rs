// src/interrupts.rs (top of file)
use alloc::string::String;
use x86_64::instructions::hlt;
use crate::alloc::string::ToString;
use core::sync::atomic::{AtomicUsize, Ordering};

// ... (in the timer handler)
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    if let Some(mut writer) = crate::vga_buffer::WRITER.try_lock() {
        writer.as_mut().blink_cursor();
    }
    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// ... (in any method using hlt)
pub fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}
