#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod memory;
pub mod keyboard;

use bootloader::BootInfo;
use vga_buffer::{Color, clear_screen, set_color, enable_cursor};

pub fn init_heap(_boot_info: &'static BootInfo) {
    // Your heap initialization code here
    use x86_64::{
        structures::paging::{
            FrameAllocator,
            Mapper,
            Page,
            Size4KiB,
        },
        VirtAddr,
    };

    pub fn init_heap(boot_info: &'static BootInfo) {
        use memory::{self, BootInfoFrameAllocator};
        use x86_64::structures::paging::Page;

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

        // Initialize a new offset page table
        let mut mapper = unsafe { memory::init(phys_mem_offset) };

        // Initialize the frame allocator
        let mut frame_allocator = unsafe {
            BootInfoFrameAllocator::init(&boot_info.memory_map)
        };

        // Initialize the heap
        allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

        // Log heap initialization for debugging
        serial_println!("[DEBUG] Heap initialized at 0x{:x}", allocator::HEAP_START);
    }


}

pub fn init_kernel(_boot_info: &'static BootInfo) {
    // Initialize GDT
    gdt::init();

    // Initialize IDT
    interrupts::init_idt();

    // Initialize PICS
    unsafe { interrupts::PICS.lock().initialize() };

    // Enable interrupts
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn init_vga() {
    serial_println!("[DEBUG] Starting VGA initialization");

    // Basic VGA initialization
    vga_buffer::init();
    serial_println!("[DEBUG] VGA hardware initialized");

    // Set color and clear screen
    set_color(Color::White, Color::Black);  // Try white text for better visibility
    clear_screen();
    serial_println!("[DEBUG] Screen cleared");

    // Write test pattern to screen buffer directly
    unsafe {
        let buffer = 0xb8000 as *mut u8;
        // Write 'TEST' in white on black
        let test_str = b"TEST";
        for (i, &byte) in test_str.iter().enumerate() {
            *buffer.offset(i as isize * 2) = byte;
            *buffer.offset(i as isize * 2 + 1) = 0x0F; // White on black
        }
    }
    serial_println!("[DEBUG] Test pattern written");

    // Try to print something
    print!("Test Output");
    serial_println!("[DEBUG] Print attempted");

    enable_cursor();
    serial_println!("[DEBUG] Cursor enabled");
}
