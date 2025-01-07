// src/main.rs
#![no_std]
#![no_main]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use crate::stat::increment_freeze_count;
use scribble::{
    freezer,
    gdt,
    memory,
    splat::{self, SplatLevel},
    stat::{self, SystemMetrics},
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    match init_system(boot_info) {
        Ok(_) => splat::log(SplatLevel::Info, "System initialization complete"),
        Err(e) => {
            splat::log(SplatLevel::Critical, "Kernel initialization failed");
            kernel_panic("Failed to initialize system");
        }
    }

    // Initialize freezer system
    freezer::FreezerState::new();

    // Try initial system thaw
    match freezer::login("slug") {
        true => {
            let boot_message = alloc::format!(
                "System activated\n\
Kernel Version: {}\n\
Boot Time: {}\n\
Current User: {}",
env!("CARGO_PKG_VERSION"),
                                              "2025-01-07 07:45:42",
                                              "isdood"
            );
            splat::log(SplatLevel::Info, &boot_message);
        }
        false => {
            splat::log(SplatLevel::Critical, "Initial system thaw failed");
            increment_freeze_count();
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
            increment_freeze_count();
            splat::log(SplatLevel::Critical, "System frozen due to anomalies");
        }

        // Deep sleep between checks
        x86_64::instructions::hlt();
    }
}

fn init_system(boot_info: &'static BootInfo) -> Result<(), &'static str> {
    gdt::init();
    unsafe {
        memory::init(x86_64::VirtAddr::new(boot_info.physical_memory_offset as u64));
    }
    Ok(())
}

fn check_system_status(stats: &SystemMetrics, consecutive_anomalies: &mut u32) {
    // Check memory usage
    let (total, used) = stat::get_memory_stats();
    let memory_usage = (used as f32 / total as f32) * 100.0;

    if memory_usage > 90.0 {
        let msg = alloc::format!(
            "Critical memory usage: {:.1}%", memory_usage
        );
        splat::log(SplatLevel::Critical, &msg);
        *consecutive_anomalies += 1;
    }

    // Check system metrics
    perform_detailed_check(stats);
}

fn perform_detailed_check(stats: &SystemMetrics) {
    splat::log(SplatLevel::Info, &stats.display());
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let panic_msg = alloc::format!("KERNEL PANIC: {}", info);
    splat::log(SplatLevel::Critical, &panic_msg);
    increment_freeze_count();
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_panic(msg: &str) -> ! {
    let panic_msg = alloc::format!(
        "KERNEL PANIC: {}\n\
System state has been preserved.\n\
Please contact system administrator.",
msg
    );
    splat::log(SplatLevel::Critical, &panic_msg);
    increment_freeze_count();
    loop {
        x86_64::instructions::hlt();
    }
}
