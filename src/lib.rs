#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]  // Add this line
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod memory;
pub mod keyboard;

use bootloader::BootInfo;

pub fn init_heap(_boot_info: &'static BootInfo) {
    // Your heap initialization code here
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
    vga_buffer::init();  // Initialize VGA hardware first
    set_color(Color::Green, Color::Black);  // Set initial color
    clear_screen();  // Clear screen with the set color
    println!("Welcome to Scribble OS");
    println!("Kernel initialized");
    println!("");  // Blank line
    print!("> ");  // Print prompt
}
