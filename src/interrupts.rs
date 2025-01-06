use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[InterruptIndex::Timer.as_usize()]
        .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
        .set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    println!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
    loop {}
}
