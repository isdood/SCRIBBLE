#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::BootInfo;
use x86_64::VirtAddr;
use alloc::format;
use core::sync::atomic::{AtomicBool, Ordering};

// Module declarations
pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;
pub mod keyboard;
pub mod splat;
pub mod stat;

// System state tracking
static SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum InitError {
    GDTFailed,
    MemoryInitFailed,
    HeapInitFailed,
    PICInitFailed,
    LockError(&'static str),
}

pub fn init(boot_info: &'static BootInfo) -> Result<(), InitError> {
    if SYSTEM_INITIALIZED.load(Ordering::SeqCst) {
        splat::log(SplatLevel::Warning, "System initialization already completed");
        return Ok(());
    }

    use x86_64::instructions::interrupts;
    interrupts::disable();
    splat::log(SplatLevel::BitsNBytes, "Interrupts disabled for initialization");

    // Initialize subsystems with proper error handling
    init_subsystems(boot_info)?;

    // Enable interrupts and finalize initialization
    interrupts::enable();
    SYSTEM_INITIALIZED.store(true, Ordering::SeqCst);
    splat::log(SplatLevel::Info, "System initialization complete");

    // Log initial system state
    log_system_state();

    Ok(())
}

fn init_subsystems(boot_info: &'static BootInfo) -> Result<(), InitError> {
    // GDT Initialization
    splat::log(SplatLevel::BitsNBytes, "Initializing GDT...");
    gdt::init();

    // Memory Management Initialization
    let (mapper, frame_allocator) = init_memory_management(boot_info)?;

    // Heap Initialization
    init_heap_memory(mapper, frame_allocator)?;

    // Interrupt System Initialization
    init_interrupt_system()?;

    Ok(())
}

fn init_memory_management(boot_info: &'static BootInfo)
-> Result<(memory::OffsetPageTable<'static>, memory::BootInfoFrameAllocator), InitError> {

    splat::log(SplatLevel::BitsNBytes, "Setting up memory management");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    splat::log(
        SplatLevel::BitsNBytes,
        &format!("Physical memory offset: {:#x}", boot_info.physical_memory_offset)
    );

    let mapper = unsafe {
        memory::init(phys_mem_offset)
    };

    let frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Initialize memory tracking
    stat::update_memory_stats(boot_info);

    Ok((mapper, frame_allocator))
}

fn init_heap_memory(
    mut mapper: memory::OffsetPageTable,
    mut frame_allocator: memory::BootInfoFrameAllocator
) -> Result<(), InitError> {
    splat::log(
        SplatLevel::BitsNBytes,
        &format!("Initializing heap (size: {} KB)", allocator::HEAP_SIZE / 1024)
    );

    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .map_err(|_| InitError::HeapInitFailed)?;

    Ok(())
}

fn init_interrupt_system() -> Result<(), InitError> {
    splat::log(SplatLevel::BitsNBytes, "Initializing interrupt system");

    // Initialize IDT
    interrupts::init_idt();

    // Initialize PIC
    unsafe {
        interrupts::PICS.try_lock()
        .ok_or(InitError::LockError("Failed to acquire PIC lock"))?
        .initialize();
    }

    Ok(())
}

fn log_system_state() {
    let stats = stat::SystemStats::current();
    splat::log(
        SplatLevel::BitsNBytes,
        &format!(
            "Initial System State:\n\
└─ Memory: {}KB / {}KB used\n\
└─ Heap Allocations: {}\n\
└─ Page Tables: Initialized\n\
└─ Interrupts: Enabled\n\
└─ GDT: Loaded\n\
└─ IDT: Configured",
stats.used_memory / 1024,
stats.total_memory / 1024,
stats.heap_allocations
        )
    );
}

// Standard I/O macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

// Testing infrastructure
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
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

// Memory management utilities
pub fn dump_memory_info(start_addr: VirtAddr, size: usize) {
    if !SYSTEM_INITIALIZED.load(Ordering::Relaxed) {
        splat::log(SplatLevel::Warning, "Cannot dump memory info: System not initialized");
        return;
    }

    splat::log(SplatLevel::BitsNBytes, "=== Memory Information Dump ===");
    stat::report_system_status();
    splat::visualize_memory_map(start_addr, size);
}
