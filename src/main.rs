#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use scribble::println;
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting...");

    // Initialize core system components with boot_info
    println!("Initializing GDT...");
    scribble::gdt::init();

    println!("Initializing IDT...");
    scribble::interrupts::init_idt();

    println!("Initializing PIC...");
    unsafe {
        scribble::interrupts::PICS.lock().initialize();
    }

    println!("Initializing memory management...");
    let phys_mem_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { scribble::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        scribble::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    scribble::allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    println!("Initializing keyboard...");
    scribble::keyboard::init();

    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();

    println!("System ready!");

    scribble::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    scribble::hlt_loop();
}
