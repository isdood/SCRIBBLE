// src/interrupts.rs
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use spin;
use crate::println;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, // This will be PIC_1_OFFSET + 1
}

pub static PICS: spin::Mutex<ChainedPics> =
spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static::lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // Fix the double fault handler setup
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler);
        }

        idt[InterruptIndex::Timer as usize]
        .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as usize]
        .set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}
