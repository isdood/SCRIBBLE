// src/interrupts.rs
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use spin;
use crate::println;  // Import from root instead of directly

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static::lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer.as_usize()]
        .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
        .set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
    {
        panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    }

    #[derive(Debug)]
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

    extern "x86-interrupt" fn timer_interrupt_handler(
        _stack_frame: InterruptStackFrame)
    {
        unsafe {
            PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
        }
    }

    extern "x86-interrupt" fn keyboard_interrupt_handler(
        _stack_frame: InterruptStackFrame)
    {
        use x86_64::instructions::port::Port;
        use pc_keyboard::{layouts, ScancodeSet1, HandleControl, Keyboard};
        use spin::Mutex;

        lazy_static::lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                                     HandleControl::Ignore)
            );
        }

        let mut port = Port::new(0x60);  // Keep the mut keyword

        let scancode: u8 = unsafe { port.read() };
        crate::keyboard::add_scancode(scancode);

        unsafe {
            PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
        }
    }

        let port = Port::new(0x60);  // Removed mut as it's not needed

        let scancode: u8 = unsafe { port.read() };
        crate::keyboard::add_scancode(scancode);

        unsafe {
            PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
        }
    }

    pub fn init_idt() {
        IDT.load();
    }
