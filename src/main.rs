#![no_std]
#![no_main]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::time::Duration;
use x86_64::instructions::{interrupts, hlt};
use scribble::splat::{self, SplatLevel};
use scribble::stat;

mod freezer;

entry_point!(kernel_main);

// System constants
const STATS_UPDATE_INTERVAL: u64 = 1000;       // Update stats every 1000 ticks
const MAX_TICK_DEVIATION: u64 = 100;           // Maximum acceptable tick deviation
const CRITICAL_MEMORY_THRESHOLD: f32 = 90.0;   // Critical memory usage percentage
const PERFORMANCE_CHECK_INTERVAL: u64 = 5000;  // Performance check interval
const AUTO_FREEZE_TIMEOUT: u64 = 600;          // Auto-freeze after 10 minutes of inactivity

#[derive(Debug)]
enum KernelState {
    Initializing,
    Thawed,
    Frozen,
    Error,
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let mut kernel_state = KernelState::Initializing;
    let mut last_activity = 0u64;

    // Initialize kernel with proper error handling
    if let Err(e) = initialize_kernel(boot_info) {
        kernel_state = KernelState::Error;
        splat::log(SplatLevel::Critical, &format!("Kernel initialization failed: {:?}", e));
        panic!("Failed to initialize kernel");
    }

    // Initialize CryoSystem
    freezer::init();

    splat::log(
        SplatLevel::BitsNBytes,
        &format!(
            "SplatNStat CryoSystem v1.0\n\
└─ Boot Time: 2025-01-07 06:10:52\n\
└─ System Admin: isdood\n\
└─ Initial State: Frozen"
        )
    );

    // Attempt initial thaw with system credentials
    match freezer::thaw("slug", "123") {
        Ok(_) => {
            kernel_state = KernelState::Thawed;
            splat::log(SplatLevel::BitsNBytes, "System thawed successfully");
        }
        Err(e) => {
            splat::log(SplatLevel::Critical, &format!("Initial system thaw failed: {:?}", e));
            kernel_state = KernelState::Frozen;
        }
    }

    // Main kernel loop state
    let mut last_stats_check = 0;
    let mut last_performance_check = 0;
    let mut consecutive_anomalies = 0;

    splat::log(SplatLevel::Info, "Entering main kernel loop");

    loop {
        match kernel_state {
            KernelState::Thawed => {
                hlt(); // CPU sleep until next interrupt

                if !interrupts::are_enabled() {
                    splat::log(SplatLevel::Critical, "Interrupts disabled unexpectedly!");
                    kernel_state = KernelState::Error;
                    continue;
                }

                let current_stats = stat::SystemStats::current();
                last_activity = current_stats.uptime_ticks;

                // Check for auto-freeze timeout
                if current_stats.uptime_ticks - last_activity > AUTO_FREEZE_TIMEOUT {
                    splat::log(SplatLevel::Warning, "Auto-freeze initiated due to inactivity");
                    freezer::freeze();
                    kernel_state = KernelState::Frozen;
                    continue;
                }

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
            KernelState::Frozen => {
                hlt();

                // Only allow minimal system functions while frozen
                if let Some(user) = freezer::get_active_user() {
                    splat::log(
                        SplatLevel::BitsNBytes,
                        &format!("System thawed by user: {}", user)
                    );
                    kernel_state = KernelState::Thawed;
                }
            }
            KernelState::Error => {
                splat::log(SplatLevel::Critical, "Kernel in error state, attempting recovery...");
                if let Ok(()) = attempt_recovery() {
                    if !freezer::is_frozen() {
                        kernel_state = KernelState::Thawed;
                    } else {
                        kernel_state = KernelState::Frozen;
                    }
                    splat::log(SplatLevel::Info, "Recovery successful");
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

    interrupts::disable();
    splat::log(SplatLevel::BitsNBytes, "Interrupts disabled for initialization");

    // Initialize the system
    scribble::init(boot_info).map_err(|_| "System initialization failed")?;

    interrupts::enable();
    splat::log(SplatLevel::BitsNBytes, "Interrupts enabled");

    Ok(())
}

fn check_system_status(stats: &stat::SystemStats, consecutive_anomalies: &mut u32) {
    // Only log if system is thawed
    if !freezer::is_frozen() {
        splat::log(
            SplatLevel::Info,
            &format!(
                "System Status:\n\
└─ Uptime: {} ticks\n\
└─ Interrupts: {} keyboard, {} critical\n\
└─ Memory: {}KB used / {}KB total\n\
└─ CryoState: {}\n\
└─ Active User: {}",
stats.uptime_ticks,
stats.keyboard_interrupts,
stats.critical_events,
stats.used_memory / 1024,
stats.total_memory / 1024,
if freezer::is_frozen() { "Frozen" } else { "Thawed" },
    freezer::get_active_user().unwrap_or_else(|| String::from("None"))
            )
        );

        check_system_health(stats, consecutive_anomalies);
    }
}

fn check_system_health(stats: &stat::SystemStats, consecutive_anomalies: &mut u32) {
    let memory_usage = (stats.used_memory as f32 / stats.total_memory as f32) * 100.0;
    if memory_usage > CRITICAL_MEMORY_THRESHOLD {
        splat::log(
            SplatLevel::Critical,
            &format!("Critical memory usage: {:.1}%", memory_usage)
        );
    }

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
    if !freezer::is_frozen() {
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
│  └─ Warnings: {}\n\
└─ CryoStatus:\n\
│  └─ {}\n\
│  └─ {}",
stats.used_memory / 1024,
stats.total_memory / 1024,
stats.page_faults,
stats.heap_allocations,
stats.critical_events,
stats.bitsnbytes_events,
stats.warning_events,
if freezer::is_frozen() { "SYSTEM FROZEN" } else { "SYSTEM THAWED" },
    freezer::get_cryo_status()
            )
        );
    }
}

fn attempt_recovery() -> Result<(), &'static str> {
    interrupts::enable();

    // Attempt to restore system security
    if freezer::is_frozen() {
        freezer::thaw("slug", "123")
        .map_err(|_| "Failed to thaw system")?;
    }

    Ok(())
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    splat::log(SplatLevel::Critical, &format!("KERNEL PANIC: {}", info));

    // Freeze system on panic
    freezer::freeze();

    if let Ok(stats) = stat::get_memory_usage() {
        splat::log(
            SplatLevel::BitsNBytes,
            &format!(
                "Final System State:\n\
└─ Memory: {}KB / {}KB\n\
└─ CryoState: {}\n\
└─ Last Active User: {}\n\
└─ Time: 2025-01-07 06:10:52",
stats.0 / 1024,
stats.1 / 1024,
if freezer::is_frozen() { "Frozen" } else { "Thawed" },
    freezer::get_active_user().unwrap_or_else(|| String::from("None"))
            )
        );
    }

    splat::dump_log();

    loop {
        interrupts::disable();
        hlt();
    }
}
