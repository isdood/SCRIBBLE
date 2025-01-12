// src/stat.rs
use alloc::format;
use alloc::string::String;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::splat::SplatLevel;

// System metrics
static TIMER_TICKS: AtomicUsize = AtomicUsize::new(0);
static KEYBOARD_INTERRUPTS: AtomicUsize = AtomicUsize::new(0);
static CRITICAL_EVENTS: AtomicUsize = AtomicUsize::new(0);
static BITSNBYTES_EVENTS: AtomicUsize = AtomicUsize::new(0);
static WARNING_EVENTS: AtomicUsize = AtomicUsize::new(0);

// Memory metrics
static TOTAL_MEMORY: AtomicUsize = AtomicUsize::new(0);
static USED_MEMORY: AtomicUsize = AtomicUsize::new(0);
static PAGE_FAULTS: AtomicUsize = AtomicUsize::new(0);
static HEAP_ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

// Freezer metrics
static THAW_ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
static FREEZE_COUNT: AtomicUsize = AtomicUsize::new(0);
static LAST_STATE_CHANGE: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct SystemMetrics {
    pub uptime_ticks: usize,
    pub keyboard_interrupts: usize,
    pub critical_events: usize,
    pub bitsnbytes_events: usize,
    pub warning_events: usize,
    pub total_memory: usize,
    pub used_memory: usize,
    pub page_faults: usize,
    pub heap_allocations: usize,
    pub thaw_attempts: usize,
    pub freeze_count: usize,
    pub last_state_change: usize,
}

impl SystemMetrics {
    pub fn current() -> Self {
        SystemMetrics {
            uptime_ticks: TIMER_TICKS.load(Ordering::Relaxed),
            keyboard_interrupts: KEYBOARD_INTERRUPTS.load(Ordering::Relaxed),
            critical_events: CRITICAL_EVENTS.load(Ordering::Relaxed),
            bitsnbytes_events: BITSNBYTES_EVENTS.load(Ordering::Relaxed),
            warning_events: WARNING_EVENTS.load(Ordering::Relaxed),
            total_memory: TOTAL_MEMORY.load(Ordering::Relaxed),
            used_memory: USED_MEMORY.load(Ordering::Relaxed),
            page_faults: PAGE_FAULTS.load(Ordering::Relaxed),
            heap_allocations: HEAP_ALLOCATIONS.load(Ordering::Relaxed),
            thaw_attempts: THAW_ATTEMPTS.load(Ordering::Relaxed),
            freeze_count: FREEZE_COUNT.load(Ordering::Relaxed),
            last_state_change: LAST_STATE_CHANGE.load(Ordering::Relaxed),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "System Metrics:\n\
Uptime Ticks: {}\n\
Keyboard Interrupts: {}\n\
Critical Events: {}\n\
Bits n' Bytes Events: {}\n\
Warning Events: {}\n\
Total Memory: {} bytes\n\
Used Memory: {} bytes\n\
Page Faults: {}\n\
Heap Allocations: {}\n\
Thaw Attempts: {}\n\
Freeze Count: {}\n\
Last State Change: {} ticks ago",
self.uptime_ticks,
self.keyboard_interrupts,
self.critical_events,
self.bitsnbytes_events,
self.warning_events,
self.total_memory,
self.used_memory,
self.page_faults,
self.heap_allocations,
self.thaw_attempts,
self.freeze_count,
self.last_state_change
        )
    }
}

// Statistics tracking functions
pub fn increment_timer() {
    TIMER_TICKS.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_keyboard() {
    KEYBOARD_INTERRUPTS.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_event_counter(level: SplatLevel) -> usize {
    match level {
        SplatLevel::Critical => CRITICAL_EVENTS.fetch_add(1, Ordering::SeqCst),
        SplatLevel::BitsNBytes => BITSNBYTES_EVENTS.fetch_add(1, Ordering::SeqCst),
        SplatLevel::Warning => WARNING_EVENTS.fetch_add(1, Ordering::SeqCst),
        _ => BITSNBYTES_EVENTS.fetch_add(0, Ordering::SeqCst), // Return 0 for other levels
    }
}

pub fn increment_thaw_attempts() {
    THAW_ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    LAST_STATE_CHANGE.store(get_timer_ticks(), Ordering::SeqCst);
}

pub fn increment_freeze_count() {
    FREEZE_COUNT.fetch_add(1, Ordering::SeqCst);
    LAST_STATE_CHANGE.store(get_timer_ticks(), Ordering::SeqCst);
}

pub fn increment_page_faults() {
    PAGE_FAULTS.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_heap_allocs() {
    HEAP_ALLOCATIONS.fetch_add(1, Ordering::SeqCst);
}

pub fn update_memory_stats(total: usize, used: usize) {
    TOTAL_MEMORY.store(total, Ordering::SeqCst);
    USED_MEMORY.store(used, Ordering::SeqCst);
}

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
    BITSNBYTES_EVENTS.load(Ordering::Relaxed)
}

pub fn get_warning_events() -> usize {
    WARNING_EVENTS.load(Ordering::Relaxed)
}

pub fn get_memory_stats() -> (usize, usize) {
    (
        TOTAL_MEMORY.load(Ordering::Relaxed),
     USED_MEMORY.load(Ordering::Relaxed)
    )
}
