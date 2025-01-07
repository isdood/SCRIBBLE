#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::time::Duration;
use x86_64::instructions::{interrupts, hlt};
use scribble::splat::{self, SplatLevel};
use scribble::stat;

entry_point!(kernel_main);

// System constants
const STATS_UPDATE_INTERVAL: u64 = 1000;      // Update stats every 1000 ticks
const MAX_TICK_DEVIATION: u64 = 100;          // Maximum acceptable tick deviation
const CRITICAL_MEMORY_THRESHOLD: f32 = 90.0;   // Critical memory usage percentage
const PERFORMANCE_CHECK_INTERVAL: u64 = 5000;  // Performance check interval

#[derive(Debug)]
enum KernelState {
    Initializing,
    Running,
    Error,
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let mut kernel_state = KernelState::Initializing;

    // Initialize kernel with proper error handling
    if let Err(e) = initialize_kernel(boot_info) {
        kernel_state = KernelState::Error;
        splat::log(SplatLevel::Critical, &format!("Kernel initialization failed: {:?}", e));
        panic!("Failed to initialize kernel");
    }

    kernel_state = KernelState::Running;

    // Main kernel loop state
    let mut last_stats_check = 0;
    let mut last_performance_check = 0;
    let mut consecutive_anomalies = 0;

    splat::log(SplatLevel::Info, "Entering main kernel loop");

    loop {
        match kernel_state {
            KernelState::Running => {
                hlt(); // CPU sleep until next interrupt

                if !interrupts::are_enabled() {
                    splat::log(SplatLevel::Critical, "Interrupts disabled unexpectedly!");
                    kernel_state = KernelState::Error;
                    continue;
                }

                let current_stats = stat::SystemStats::current();

                // Regular stats update
                if current_stats.uptime_ticks >= last_stats_check + STATS_UPDATE_INTERVAL {
                    check_system_status(&current_stats, &mut consecutive_anomalies);
                    last_stats_check = current_stats.uptime_ticks;
                }

                // Detailed performance check
                if current_stats.uptime_ticks >= last_performance_check + PERFORMANCE_CHECK_INTERVAL {
                    perform_detailed_check(&current_stats);
                    last_performance_check = current_stats.uptime_ticks;
                }
            }
            KernelState::Error => {
                splat::log(SplatLevel::Critical, "Kernel in error state, attempting recovery...");
                if let Ok(()) = attempt_recovery() {
                    kernel_state = KernelState::Running;
                    splat::log(SplatLevel::Info, "Recovery successful, resuming normal operation");
                }
            }
            KernelState::Initializing => {
                splat::log(SplatLevel::Critical, "Invalid kernel state detected!");
                kernel_state = KernelState::Error;
            }
        }
    }
}

fn initialize_kernel(boot_info: &'static BootInfo) -> Result<(), &'static str> {
    splat::log(SplatLevel::Info, "Starting kernel initialization");

    // Disable interrupts during initialization
    interrupts::disable();
    splat::log(SplatLevel::BitsNBytes, "Interrupts disabled for initialization");

    // Initialize the system
    scribble::init(boot_info).map_err(|_| "System initialization failed")?;

    // Enable interrupts
    interrupts::enable();
    splat::log(SplatLevel::BitsNBytes, "Interrupts enabled");

    Ok(())
}

fn check_system_status(stats: &stat::SystemStats, consecutive_anomalies: &mut u32) {
    // Log basic system status
    splat::log(
        SplatLevel::Info,
        &format!(
            "System Status:\n\
└─ Uptime: {} ticks\n\
└─ Interrupts: {} keyboard, {} critical\n\
└─ Memory: {}KB used / {}KB total",
stats.uptime_ticks,
stats.keyboard_interrupts,
stats.critical_events,
stats.used_memory / 1024,
stats.total_memory / 1024
        )
    );

    // Check memory usage
    let memory_usage = (stats.used_memory as f32 / stats.total_memory as f32) * 100.0;
    if memory_usage > CRITICAL_MEMORY_THRESHOLD {
        splat::log(
            SplatLevel::Critical,
            &format!("Critical memory usage: {:.1}%", memory_usage)
        );
    }

    // Check for timing anomalies
    if stats.uptime_ticks > MAX_TICK_DEVIATION {
        *consecutive_anomalies += 1;
        splat::log(
            SplatLevel::Warning,
            &format!("Timing anomaly detected ({} consecutive)", consecutive_anomalies)
        );
    } else {
        *consecutive_anomalies = 0;
    }
}

fn perform_detailed_check(stats: &stat::SystemStats) {
    splat::log(
        SplatLevel::BitsNBytes,
        &format!(
            "Detailed System Analysis:\n\
└─ Memory:\n\
│  └─ Usage: {}KB / {}KB\n\
│  └─ Page Faults: {}\n\
│  └─ Heap Allocations: {}\n\
└─ Events:\n\
│  └─ Critical: {}\n\
│  └─ BitsNBytes: {}\n\
│  └─ Warnings: {}",
stats.used_memory / 1024,
stats.total_memory / 1024,
stats.page_faults,
stats.heap_allocations,
stats.critical_events,
stats.bitsnbytes_events,
stats.warning_events
        )
    );
}

fn attempt_recovery() -> Result<(), &'static str> {
    // Basic recovery attempts
    interrupts::enable();
    Ok(())
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    splat::log(SplatLevel::Critical, &format!("KERNEL PANIC: {}", info));

    // Dump detailed system state
    if let Ok(stats) = stat::get_memory_usage() {
        splat::log(
            SplatLevel::BitsNBytes,
            &format!("Final Memory State: {}KB / {}KB", stats.0 / 1024, stats.1 / 1024)
        );
    }

    splat::dump_log();

    loop {
        interrupts::disable();
        hlt();
    }
}
