#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

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

// Keep only one definition of hlt_loop
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init_vga() {
    let mut writer = vga_buffer::WRITER.lock();
    writer.enable_cursor(); // Now public
    drop(writer);

    vga_buffer::clear_screen();
    vga_buffer::set_color(vga_buffer::Color::Green, vga_buffer::Color::Black);
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
