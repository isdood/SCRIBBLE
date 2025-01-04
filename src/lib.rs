// src/lib.rs
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader::BootInfo;
use core::panic::PanicInfo;
use vga_buffer::{Color, set_color, clear_screen};

pub mod vga_buffer;
mod gdt;
mod interrupts;
mod memory;
mod allocator;
mod keyboard;

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
    }

    #[macro_export]
    macro_rules! println {
        () => ($crate::print!("\n"));
        ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

pub fn init(boot_info: &'static BootInfo) {
    use crate::interrupts::PICS;

    // Clear the screen first
    clear_screen();

    // Boot sequence with colors
    set_color(Color::Yellow, Color::Blue);
    println!("\n=== Scribble OS ===");
    set_color(Color::LightCyan, Color::Black);
    println!("Starting initialization sequence...\n");

    // GDT initialization
    set_color(Color::LightGreen, Color::Black);
    print!("Loading GDT... ");
    gdt::init();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // IDT initialization
    set_color(Color::LightCyan, Color::Black);
    print!("Setting up IDT... ");
    interrupts::init_idt();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // PIC initialization
    set_color(Color::Magenta, Color::Black);
    print!("Configuring PIC... ");
    unsafe { PICS.lock().initialize() };
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Memory management
    set_color(Color::LightBlue, Color::Black);
    print!("Setting up memory management... ");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let _mapper = unsafe { memory::init(phys_mem_offset) };
    let _frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Keyboard initialization
    set_color(Color::LightBlue, Color::Black);
    print!("Setting up keyboard handler... ");
    keyboard::initialize();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Enable interrupts
    set_color(Color::LightCyan, Color::Black);
    print!("Enabling interrupts... ");
    x86_64::instructions::interrupts::enable();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Final messages
    set_color(Color::Yellow, Color::Blue);
    println!("\nSystem initialization complete!");
    set_color(Color::White, Color::Black);
    println!("\nReady for input...\n");
}
