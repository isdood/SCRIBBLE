// src/interrupts.rs
use unstable_matter::arch::x86_64::structures::idt::InterruptStackFrame;
use unstable_matter::arch::x86_64::instructions::interrupts;
use crate::pic8259::{PICS, PIC_1_OFFSET};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    if let Some(mut writer) = crate::vga_buffer::WRITER.try_lock() {
        writer.blink_cursor();  // Call directly on MutexGuard
    }
    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}
