#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod memory;

use bootloader::BootInfo;

pub fn init_heap(boot_info: &'static BootInfo) {
    // Your heap initialization code
}

pub fn init_kernel(boot_info: &'static BootInfo) {
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // Initialize GDT
    gdt::init();

    // Initialize IDT
    interrupts::init_idt();

    // Initialize PICS
    unsafe { interrupts::PICS.lock().initialize() };

    // Enable interrupts
    x86_64::instructions::interrupts::enable();
}

// Add VGA initialization function
pub fn init_vga() {
    vga_buffer::clear_screen();
    vga_buffer::set_color(vga_buffer::Color::Green, vga_buffer::Color::Black);
    print!("> ");
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
