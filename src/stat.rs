use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;
use crate::splat::SplatLevel;
use alloc::format;

// System statistics
static TIMER_TICKS: AtomicUsize = AtomicUsize::new(0);
static KEYBOARD_INTERRUPTS: AtomicUsize = AtomicUsize::new(0);
static CRITICAL_EVENTS: AtomicUsize = AtomicUsize::new(0);
static BITSNBYTES_EVENTS: AtomicUsize = AtomicUsize::new(0);
static WARNING_EVENTS: AtomicUsize = AtomicUsize::new(0);

// Memory statistics
static TOTAL_MEMORY: AtomicUsize = AtomicUsize::new(0);
static USED_MEMORY: AtomicUsize = AtomicUsize::new(0);
static PAGE_FAULTS: AtomicUsize = AtomicUsize::new(0);
static HEAP_ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy)]
pub struct SystemStats {
    pub uptime_ticks: usize,
    pub keyboard_interrupts: usize,
    pub critical_events: usize,
    pub bitsnbytes_events: usize,
    pub warning_events: usize,
    pub total_memory: usize,
    pub used_memory: usize,
    pub page_faults: usize,
    pub heap_allocations: usize,
}

impl SystemStats {
    pub fn current() -> Self {
        SystemStats {
            uptime_ticks: TIMER_TICKS.load(Ordering::Relaxed),
            keyboard_interrupts: KEYBOARD_INTERRUPTS.load(Ordering::Relaxed),
            critical_events: CRITICAL_EVENTS.load(Ordering::Relaxed),
            bitsnbytes_events: BITSNBYTES_EVENTS.load(Ordering::Relaxed), // Fixed: BITS_EVENTS -> BITSNBYTES_EVENTS
            warning_events: WARNING_EVENTS.load(Ordering::Relaxed),
            total_memory: TOTAL_MEMORY.load(Ordering::Relaxed),
            used_memory: USED_MEMORY.load(Ordering::Relaxed),
            page_faults: PAGE_FAULTS.load(Ordering::Relaxed),
            heap_allocations: HEAP_ALLOCATIONS.load(Ordering::Relaxed),
        }
    }

    pub fn memory_usage_kb(&self) -> (usize, usize) {
        (self.total_memory / 1024, self.used_memory / 1024)
    }
}

// Event recording functions
pub fn timer_tick() {
    TIMER_TICKS.fetch_add(1, Ordering::SeqCst);
}

pub fn keyboard_interrupt() {
    KEYBOARD_INTERRUPTS.fetch_add(1, Ordering::SeqCst);
}

pub fn log_event(level: SplatLevel) {
    match level {
        SplatLevel::Critical => CRITICAL_EVENTS.fetch_add(1, Ordering::SeqCst),
        SplatLevel::BitsNBytes => BITSNBYTES_EVENTS.fetch_add(1, Ordering::SeqCst), // Fixed: BITS_EVENTS -> BITSNBYTES_EVENTS
        SplatLevel::Warning => WARNING_EVENTS.fetch_add(1, Ordering::SeqCst),
        _ => {}
    };
}

pub fn page_fault() {
    PAGE_FAULTS.fetch_add(1, Ordering::SeqCst);
}

pub fn heap_allocation() {
    HEAP_ALLOCATIONS.fetch_add(1, Ordering::SeqCst);
}

pub fn update_memory_usage(total: usize, used: usize) {
    TOTAL_MEMORY.store(total, Ordering::SeqCst);
    USED_MEMORY.store(used, Ordering::SeqCst);
}

// Getters for individual statistics
pub fn get_timer_ticks() -> usize {
    TIMER_TICKS.load(Ordering::Relaxed)
}

pub fn get_keyboard_interrupts() -> usize {
    KEYBOARD_INTERRUPTS.load(Ordering::Relaxed)
}

pub fn get_critical_events() -> usize {
    CRITICAL_EVENTS.load(Ordering::Relaxed)
}

pub fn get_bitsnbytes_events() -> usize {
    BITSNBYTES_EVENTS.load(Ordering::Relaxed) // Fixed: BITS_EVENTS -> BITSNBYTES_EVENTS
}

pub fn get_warning_events() -> usize {
    WARNING_EVENTS.load(Ordering::Relaxed)
}

pub fn get_memory_usage() -> (usize, usize) {
    (
        TOTAL_MEMORY.load(Ordering::Relaxed),
     USED_MEMORY.load(Ordering::Relaxed)
    )
}

// System status reporting
pub fn report_system_status() {
    let stats = SystemStats::current();
    let (total_kb, used_kb) = stats.memory_usage_kb();

    crate::splat::log(
        SplatLevel::Info,
        &format!(
            "System Status Report:\n\
Uptime Ticks: {}\n\
Keyboard Interrupts: {}\n\
Critical Events: {}\n\
Bits & Bytes Events: {}\n\
Warnings: {}\n\
Memory Usage: {}/{} KB\n\
Page Faults: {}\n\
Heap Allocations: {}",
stats.uptime_ticks,
stats.keyboard_interrupts,
stats.critical_events,
stats.bitsnbytes_events,
stats.warning_events,
used_kb,
total_kb,
stats.page_faults,
stats.heap_allocations
        )
    );
}
