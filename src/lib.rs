#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::VirtAddr;

// Declare modules
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod keyboard;
pub mod serial;
pub mod memory;
pub mod allocator;

// Remove these re-exports since macros are already exported at crate root
// The macros are automatically available due to #[macro_export]

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init(boot_info: &'static BootInfo) {
    use x86_64::instructions::interrupts;
    use crate::interrupts::{init_idt, PICS};
    use crate::vga_buffer::{Color, set_color};

    // Title in Yellow on Blue
    set_color(Color::Yellow, Color::Blue);
    println!("\n=== Scribble OS ===");
    set_color(Color::LightCyan, Color::Black);
    println!("Starting initialization sequence...\n");

    // GDT initialization in Green
    set_color(Color::LightGreen, Color::Black);
    print!("Loading GDT... ");
    gdt::init();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // IDT initialization in Cyan
    set_color(Color::LightCyan, Color::Black);
    print!("Setting up IDT... ");
    init_idt();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // PIC initialization in Magenta
    set_color(Color::LightMagenta, Color::Black);
    print!("Configuring PIC... ");
    unsafe {
        PICS.lock().initialize();
    }
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Memory management in Blue
    set_color(Color::LightBlue, Color::Black);
    print!("Setting up memory management... ");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Heap initialization in Yellow
    set_color(Color::Yellow, Color::Black);
    print!("Initializing heap... ");
    if let Err(err) = allocator::init_heap(&mut mapper, &mut frame_allocator) {
        set_color(Color::Red, Color::Black);
        println!("FAILED");
        panic!("Heap initialization failed: {:?}", err);
    }
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Keyboard initialization in Pink
    set_color(Color::Pink, Color::Black);
    print!("Setting up keyboard handler... ");
    keyboard::initialize();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Interrupt enabling in Cyan
    set_color(Color::LightCyan, Color::Black);
    print!("Enabling interrupts... ");
    interrupts::enable();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Final message in bright colors
    set_color(Color::Yellow, Color::Blue);
    println!("\nSystem initialization complete!");

    // Reset to a comfortable default color for user interaction
    set_color(Color::LightGray, Color::Black);
    println!("\nReady for input...\n");
}

// ... (rest of the code remains the same)

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nPANIC: {}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
