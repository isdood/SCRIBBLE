// In lib.rs, update the init function
pub fn init(boot_info: &'static BootInfo) {
    use x86_64::instructions::interrupts;
    use crate::interrupts::{init_idt, PICS}; // Add these imports at the top of the function

    println!("=== Scribble OS ===");
    println!("Initializing system...");

    println!("Loading GDT...");
    gdt::init();

    println!("Setting up IDT...");
    init_idt(); // Use the imported function

    println!("Configuring PIC...");
    unsafe {
        PICS.lock().initialize(); // Use the imported PICS
    }

    // Rest of the initialization remains the same...
    println!("Setting up memory management...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    println!("Initializing heap...");
    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    println!("Initializing keyboard...");
    keyboard::initialize();

    // Enable interrupts after all initialization is complete
    println!("Enabling interrupts...");
    interrupts::enable();

    println!("System initialization complete!");
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
    println!("{}", info);
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
