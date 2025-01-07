// IMPORTS \\
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::{splat_info, serial_println};
use crate::keyboard;
use crate::vga_buffer::WRITER;
use crate::stats::SYSTEM_STATS;
use crate::println;
// END IMPORTS //

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Hardware interrupt indices
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        self as usize
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        // Make sure keyboard interrupt is properly registered
        {
            idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard::keyboard_interrupt_handler)
            .set_present(true);
        }

        idt[InterruptIndex::Timer.as_usize()]
        .set_handler_fn(timer_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    // Add debug print to verify IDT loading
    serial_println!("IDT loaded successfully");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    loop {}
}

// Timer interrupt handler
use crate::{stats::SYSTEM_STATS};

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::interrupts;

    // Quick increment without holding lock too long
    {
        let mut stats = SYSTEM_STATS.lock();
        stats.increment_timer();
    }

    // Only perform cursor blink and debug output every 100 ticks
    static mut TICK_COUNT: u64 = 0;
    unsafe {
        TICK_COUNT = TICK_COUNT.wrapping_add(1);
        if TICK_COUNT % 100 == 0 {
            interrupts::without_interrupts(|| {
                if let Some(mut writer) = WRITER.try_lock() {
                    writer.blink_cursor();
                }
            });
        }

        // End interrupt quickly
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
