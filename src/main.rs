// src/main.rs

#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use scribble::{freezer, stat, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!("Entering kernel_main");

    match init_system(boot_info) {
        Ok(_) => println!("System initialization complete"),
        Err(_) => {
            println!("Kernel initialization failed");
            panic!();
        }
    }

    println!("System initialized");

    // Initialize the freezer state
    freezer::FreezerState::new();
    let login_result = freezer::login("slug");

    match login_result {
        Ok(_) => println!("System activated"),
        Err(_) => println!("Initial system thaw failed"),
    }

    let mut consecutive_anomalies = 0;

    loop {
        // Check system status periodically
        let current_stats = stat::SystemMetrics::current();
        check_system_status(&current_stats, &mut consecutive_anomalies);

        if consecutive_anomalies > 5 {
            println!("System frozen due to anomalies");
            panic!();
        }
    }
}

fn init_system(_boot_info: &'static mut BootInfo) -> Result<(), &'static str> {
    // Initialization logic here
    Ok(())
}

fn check_system_status(_current_stats: &stat::SystemMetrics, _consecutive_anomalies: &mut u32) {
    // Check system status logic here
}

// Define a panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    loop {}
}
