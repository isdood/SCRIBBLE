use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;
use crate::splat::SplatLevel;
use crate::freezer;
use alloc::format;
use crate::String;

// System constants
const SYSTEM_CREATION: &str = "2025-01-07 06:12:41";
const SYSTEM_CREATOR: &str = "isdood";

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

// CryoSystem statistics
static THAW_ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
static FREEZE_COUNT: AtomicUsize = AtomicUsize::new(0);
static LAST_STATE_CHANGE: AtomicUsize = AtomicUsize::new(0);

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
    pub thaw_attempts: usize,
    pub freeze_count: usize,
    pub last_state_change: usize,
}

impl SystemStats {
    pub fn current() -> Self {
        SystemStats {
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

    pub fn memory_usage_kb(&self) -> (usize, usize) {
        (self.total_memory / 1024, self.used_memory / 1024)
    }

    pub fn get_cryo_metrics(&self) -> String {
        format!(
            "CryoSystem Metrics:\n\
└─ Total Thaw Attempts: {}\n\
└─ Freeze Count: {}\n\
└─ Last State Change: {} ticks ago",
self.thaw_attempts,
self.freeze_count,
self.uptime_ticks.saturating_sub(self.last_state_change)
        )
    }
}

// Event recording functions
pub fn timer_tick() {
    TIMER_TICKS.fetch_add(1, Ordering::SeqCst);
}

pub fn keyboard_interrupt() {
    KEYBOARD_INTERRUPTS.fetch_add(1, Ordering::SeqCst);
}

pub fn log_event(level: SplatLevel) -> usize {
    match level {
        SplatLevel::Critical => CRITICAL_EVENTS.fetch_add(1, Ordering::SeqCst),
        SplatLevel::BitsNBytes => BITSNBYTES_EVENTS.fetch_add(1, Ordering::SeqCst),
        SplatLevel::Warning => WARNING_EVENTS.fetch_add(1, Ordering::SeqCst),
        _ => BITSNBYTES_EVENTS.fetch_add(0, Ordering::SeqCst), // Return 0 for other levels
    }
}

// CryoSystem event recording
pub fn record_thaw_attempt() {
    THAW_ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    LAST_STATE_CHANGE.store(get_timer_ticks(), Ordering::SeqCst);
}

pub fn record_freeze() {
    FREEZE_COUNT.fetch_add(1, Ordering::SeqCst);
    LAST_STATE_CHANGE.store(get_timer_ticks(), Ordering::SeqCst);
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
    BITSNBYTES_EVENTS.load(Ordering::Relaxed)
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

pub fn display_status() {
    let cryo_state = if freezer::is_frozen() { "Frozen" } else { "Thawed" };
}

// System status reporting
pub fn report_system_status() {
    let stats = SystemStats::current();
    let (total_kb, used_kb) = stats.memory_usage_kb();
    let cryo_state = if crate::freezer::is_frozen() { "Frozen" } else { "Thawed" };
    let active_user = crate::freezer::get_active_user()
    .unwrap_or_else(|| String::from("None"));

    crate::splat::log(
        SplatLevel::Info,
        &format!(
            "System Status Report [{}]:\n\
└─ System Info:\n\
│  └─ Created: {}\n\
│  └─ Creator: {}\n\
│  └─ Active User: {}\n\
└─ CryoState: {}\n\
└─ Performance:\n\
│  └─ Uptime Ticks: {}\n\
│  └─ Keyboard Events: {}\n\
│  └─ Critical Events: {}\n\
│  └─ BitsNBytes Events: {}\n\
│  └─ Warnings: {}\n\
└─ Memory:\n\
│  └─ Usage: {}/{} KB\n\
│  └─ Page Faults: {}\n\
│  └─ Heap Allocs: {}\n\
└─ CryoMetrics:\n\
│  └─ Thaw Attempts: {}\n\
│  └─ Freeze Count: {}\n\
│  └─ Last State Change: {} ticks ago",
crate::rtc::DateTime::now().to_string(),
                 SYSTEM_CREATION,
                 SYSTEM_CREATOR,
                 active_user,
                 cryo_state,
                 stats.uptime_ticks,
                 stats.keyboard_interrupts,
                 stats.critical_events,
                 stats.bitsnbytes_events,
                 stats.warning_events,
                 used_kb,
                 total_kb,
                 stats.page_faults,
                 stats.heap_allocations,
                 stats.thaw_attempts,
                 stats.freeze_count,
                 stats.uptime_ticks.saturating_sub(stats.last_state_change)
        )
    );
}
