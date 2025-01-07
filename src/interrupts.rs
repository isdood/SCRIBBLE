// IMPORTS
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use pic8259::ChainedPics;
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::format;
use alloc::string::String;
use x86_64::instructions::hlt;
use crate::{
    println,
    serial_println,
    keyboard::{self, KEYBOARD},
    vga_buffer::WRITER,
    splat::{self, SplatLevel},
    stat,
};

// PIC Configuration
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// Interrupt Statistics
static INTERRUPT_COUNT: AtomicU64 = AtomicU64::new(0);
static LAST_INTERRUPT_TIME: AtomicU64 = AtomicU64::new(0);

pub static PICS: Mutex<ChainedPics> =
Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

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

    fn log_interrupt(&self) {
        INTERRUPT_COUNT.fetch_add(1, Ordering::Relaxed);
        LAST_INTERRUPT_TIME.store(
            crate::rtc::DateTime::now().to_string().parse().unwrap_or(0),
                                  Ordering::Relaxed
        );
    }
}

#[derive(Debug)]
pub struct InterruptStats {
    total_interrupts: u64,
    last_interrupt_time: u64,
    current_time: String,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        splat::log(SplatLevel::BitsNBytes, "Configuring interrupt handlers");

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::Keyboard.as_usize()]
        .set_handler_fn(keyboard_interrupt_handler)
        .set_present(true);

        idt[InterruptIndex::Timer.as_usize()]
        .set_handler_fn(timer_interrupt_handler);

        splat::log(SplatLevel::BitsNBytes, "Interrupt handlers configured");

        idt
    };
}

pub fn init_idt() {
    splat::log(SplatLevel::BitsNBytes, "Loading Interrupt Descriptor Table");
    IDT.load();
    splat::log(SplatLevel::BitsNBytes, "IDT loaded successfully");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    splat::log(
        SplatLevel::Warning,
        &format!(
            "BREAKPOINT EXCEPTION\n\
└─ Time: {}\n\
└─ Stack Frame: {:#?}",
crate::rtc::DateTime::now().to_string(),
                 stack_frame
        )
    );
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    let fault_address = Cr2::read();

    splat::log(
        SplatLevel::Critical,
        &format!(
            "PAGE FAULT DETECTED\n\
└─ Time: {}\n\
└─ Fault Address: {:#x}\n\
└─ Error Code: {:?}\n\
└─ Details:\n\
│  └─ Present: {}\n\
│  └─ Write: {}\n\
│  └─ User: {}\n\
│  └─ Reserved Write: {}\n\
│  └─ Instruction Fetch: {}\n\
└─ Stack Frame: {:#?}",
crate::rtc::DateTime::now().to_string(),
                 fault_address.as_u64(),
                 error_code,
                 error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION),
                 error_code.contains(PageFaultErrorCode::CAUSED_BY_WRITE),
                 error_code.contains(PageFaultErrorCode::USER_MODE),
                 error_code.contains(PageFaultErrorCode::MALFORMED_TABLE),
                 error_code.contains(PageFaultErrorCode::INSTRUCTION_FETCH),
                 stack_frame
        )
    );

    stat::page_fault();
    handle_fatal_error();
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    InterruptIndex::Timer.log_interrupt();
    stat::timer_tick();

    static mut TICK_COUNT: u64 = 0;
    unsafe {
        TICK_COUNT = TICK_COUNT.wrapping_add(1);

        // Cursor blinking every 100 ticks
        if TICK_COUNT % 100 == 0 {
            if let Some(mut writer) = WRITER.try_lock() {
                writer.blink_cursor();
            }
        }

        // Log system stats every 1000 ticks
        if TICK_COUNT % 1000 == 0 {
            log_system_stats();
        }
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    InterruptIndex::Keyboard.log_interrupt();
    stat::keyboard_interrupt();

    match KEYBOARD.try_lock() {
        Some(mut keyboard) => {
            let mut port = Port::new(0x60);
            let scancode: u8 = unsafe { port.read() };

            match keyboard.add_byte(scancode) {
                Ok(Some(key_event)) => {
                    let key_event = key_event.clone(); // Clone the key event
                    if let Some(key) = keyboard.process_keyevent(key_event) {
                        splat::log(SplatLevel::BitsNBytes, &format!("Key Event: {:?}", key));
                    }
                }
                Ok(None) => (), // No complete keypress yet
                Err(e) => splat::log(SplatLevel::Warning, &alloc::format!("Keyboard error: {:?}", e)),
            }
        }
        None => splat::log(SplatLevel::Warning, "Keyboard buffer locked"),
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

fn handle_fatal_error() -> ! {
    loop {
        interrupts::disable();
        interrupts::hlt();
    }
}

fn log_system_stats() {
    let stats = InterruptStats {
        total_interrupts: INTERRUPT_COUNT.load(Ordering::Relaxed),
        last_interrupt_time: LAST_INTERRUPT_TIME.load(Ordering::Relaxed),
        current_time: crate::rtc::DateTime::now().to_string(),
    };

    splat::log(
        SplatLevel::BitsNBytes,
        &format!(
            "System Statistics:\n\
└─ Total Interrupts: {}\n\
└─ Last Interrupt: {}\n\
└─ Current Time: {}",
stats.total_interrupts,
stats.last_interrupt_time,
stats.current_time
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupt_index() {
        assert_eq!(InterruptIndex::Timer.as_u8(), PIC_1_OFFSET);
        assert_eq!(InterruptIndex::Keyboard.as_u8(), PIC_1_OFFSET + 1);
    }
}
