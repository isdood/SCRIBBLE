pub fn init(boot_info: &'static BootInfo) {
    use crate::interrupts::PICS;

    // Clear the screen first
    clear_screen();

    // Boot sequence with colors
    set_color(Color::Yellow, Color::Blue);
    crate::println!("\n=== Scribble OS ===");

    set_color(Color::LightCyan, Color::Black);
    crate::println!("Starting initialization sequence...\n");

    // GDT initialization
    set_color(Color::LightGreen, Color::Black);
    crate::print!("Loading GDT... ");
    gdt::init();
    set_color(Color::White, Color::Black);
    crate::println!("OK");

    // IDT initialization
    set_color(Color::LightCyan, Color::Black);
    crate::print!("Setting up IDT... ");
    interrupts::init_idt();
    set_color(Color::White, Color::Black);
    crate::println!("OK");

    // PIC initialization
    set_color(Color::Magenta, Color::Black);
    crate::print!("Configuring PIC... ");
    unsafe { PICS.lock().initialize() };
    set_color(Color::White, Color::Black);
    crate::println!("OK");

    // Memory management
    set_color(Color::LightBlue, Color::Black);
    crate::print!("Setting up memory management... ");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let _mapper = unsafe { memory::init(phys_mem_offset) };
    let _frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    set_color(Color::White, Color::Black);
    crate::println!("OK");

    // Keyboard initialization
    set_color(Color::LightBlue, Color::Black);
    crate::print!("Setting up keyboard handler... ");
    keyboard::initialize();
    set_color(Color::White, Color::Black);
    crate::println!("OK");

    // Enable interrupts
    set_color(Color::LightCyan, Color::Black);
    crate::print!("Enabling interrupts... ");
    x86_64::instructions::interrupts::enable();
    set_color(Color::White, Color::Black);
    crate::println!("OK");

    // Final messages in correct order
    set_color(Color::Yellow, Color::Blue);
    crate::println!("\nSystem initialization complete!");
    crate::println!("Welcome to Scribble OS!");

    set_color(Color::Green, Color::Black);
    crate::println!("\nType something to test the keyboard...");
    crate::print!("Ready for input > ");  // Added prompt character
}
