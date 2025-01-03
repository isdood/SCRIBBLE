use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use x86_64::instructions::interrupts;
use crate::keyboard;
use crate::println;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

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

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler)
            .set_stack_index(crate::gdt::KEYBOARD_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    interrupts::without_interrupts(|| {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };

        println!("Received scancode: {:x}", scancode);

        unsafe {
            PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
        }

        keyboard::add_scancode(scancode);
    });
}

pub fn init_idt() {  // Make sure this is public
    IDT.load();
}
