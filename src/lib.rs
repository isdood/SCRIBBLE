// src/lib.rs
pub fn init(boot_info: &'static BootInfo) {
    use x86_64::instructions::interrupts;
    use crate::interrupts::{init_idt, PICS};
    use crate::vga_buffer::{Color, set_color, clear_screen};
    use x86_64::VirtAddr;

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
    init_idt();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // PIC initialization
    set_color(Color::Magenta, Color::Black);
    print!("Configuring PIC... ");
    unsafe {
        PICS.lock().initialize();
    }
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Memory management
    set_color(Color::LightBlue, Color::Black);
    print!("Setting up memory management... ");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Heap initialization
    set_color(Color::Yellow, Color::Black);
    print!("Initializing heap... ");
    if let Err(err) = allocator::init_heap(&mut mapper, &mut frame_allocator) {
        set_color(Color::Red, Color::Black);
        println!("FAILED");
        panic!("Heap initialization failed: {:?}", err);
    }
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Keyboard initialization
    set_color(Color::Pink, Color::Black);
    print!("Setting up keyboard handler... ");
    keyboard::initialize();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Enable interrupts
    set_color(Color::LightCyan, Color::Black);
    print!("Enabling interrupts... ");
    interrupts::enable();
    set_color(Color::Green, Color::Black);
    println!("OK");

    // Final message
    set_color(Color::Yellow, Color::Blue);
    println!("\nSystem initialization complete!");

    // Reset to default color for user interaction
    set_color(Color::LightGray, Color::Black);
    println!("\nReady for input...\n");
}
