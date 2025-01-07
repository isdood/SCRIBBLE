#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod allocator;
pub mod freezer;
pub mod gdt;
pub mod interrupts;
pub mod keyboard;
pub mod memory;
pub mod pic8259;
pub mod rtc;
pub mod vga_buffer;
pub mod serial;
pub mod splat;
pub mod stat;
pub mod unstable_matter;


use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use alloc::format;
use crate::stat::{SystemMetrics, increment_freeze_count};
use crate::splat::SplatLevel;


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!("Entering kernel_main");

    match init_system(boot_info) {
        Ok(_) => println!("System initialization complete"),
        Err(_) => {
            println!("Kernel initialization failed");
            kernel_panic("Failed to initialize system");
        }
    }

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

fn init_system(boot_info: &'static mut BootInfo) -> Result<(), &'static str> {
    gdt::init();
    unsafe {
        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.unwrap_or(0) as u64);
        let mapper = memory::init(phys_mem_offset);

        // Initialize heap with proper error handling
        allocator::init_heap(0x4000_0000_0000, 100 * 1024 * 1024);
    }
    Ok(())
}

fn check_system_status(stats: &SystemMetrics, consecutive_anomalies: &mut u32) {
    // Check memory usage
    let (total, used) = stat::get_memory_stats();
    let memory_usage = (used as f32 / total as f32) * 100.0;

    if memory_usage > 90.0 {
        let msg = format!(
            "Critical memory usage: {:.1}%", memory_usage
        );
        splat::log(SplatLevel::Critical, &msg);
        *consecutive_anomalies += 1;
    }

    // Check system metrics
    perform_detailed_check(stats);
}

pub fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError> {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset as u64);
    let mapper = unsafe { memory::init(physical_memory_offset) };
    let frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    Ok((mapper, frame_allocator))
}

fn perform_detailed_check(stats: &SystemMetrics) {
    splat::log(SplatLevel::Info, &stats.display());
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    increment_freeze_count();
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_panic(msg: &str) -> ! {
    println!("KERNEL PANIC: {}", msg);
    increment_freeze_count();
    loop {
        x86_64::instructions::hlt();
    }
}
