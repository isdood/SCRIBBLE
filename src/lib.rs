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

// Remove these re-exports as the macros are already exported via #[macro_export]
// pub use crate::vga_buffer::{print, println};
// pub use crate::serial::{serial_print, serial_println};

// ... (rest of the code remains the same)

// Re-export macros
pub use crate::vga_buffer::{print, println};
pub use crate::serial::{serial_print, serial_println};

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init(boot_info: &'static BootInfo) {
    use x86_64::instructions::interrupts;
    use crate::interrupts::{init_idt, PICS};

    println!("\n=== Scribble OS ===");
    println!("Starting initialization sequence...\n");

    // Initialize GDT first as it's required for interrupt handling
    print!("Loading GDT... ");
    gdt::init();
    println!("OK");

    // Set up the Interrupt Descriptor Table
    print!("Setting up IDT... ");
    init_idt();
    println!("OK");

    // Configure the Programmable Interrupt Controller
    print!("Configuring PIC... ");
    unsafe {
        PICS.lock().initialize();
    }
    println!("OK");

    // Initialize memory management
    print!("Setting up memory management... ");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    println!("OK");

    // Initialize heap allocator
    print!("Initializing heap... ");
    if let Err(err) = allocator::init_heap(&mut mapper, &mut frame_allocator) {
        println!("FAILED");
        panic!("Heap initialization failed: {:?}", err);
    }
    println!("OK");

    // Initialize keyboard handler
    print!("Setting up keyboard handler... ");
    keyboard::initialize();
    println!("OK");

    // Enable interrupts last, after all initialization is complete
    print!("Enabling interrupts... ");
    interrupts::enable();
    println!("OK");

    println!("\nSystem initialization complete!\n");
}

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
