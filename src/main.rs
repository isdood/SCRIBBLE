#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Add debugging output here
    println!("Entering kernel_main");

    match init_system(boot_info) {
        Ok(_) => println!("System initialization complete"),
        Err(_) => {
            println!("Kernel initialization failed");
            kernel_panic("Failed to initialize system");
        }
    }

    // Add more debugging output
    println!("System initialized");

    // Initialize freezer system
    freezer::FreezerState::new();

    // Try initial system thaw
    match freezer::login("slug") {
        true => {
            println!("System activated");
        }
        false => {
            println!("Initial system thaw failed");
            kernel_panic("Authentication failure during system initialization");
        }
    }

    // Main system loop
    let mut consecutive_anomalies = 0;
    loop {
        let current_stats = stat::SystemMetrics::current();

        // System health checks
        check_system_status(&current_stats, &mut consecutive_anomalies);

        if consecutive_anomalies > 5 {
            println!("System frozen due to anomalies");
        }

        // Deep sleep between checks
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_panic(msg: &str) -> ! {
    println!("KERNEL PANIC: {}", msg);
    loop {
        x86_64::instructions::hlt();
    }
}
