// src/stats.rs
use spin::Mutex;
use lazy_static::lazy_static;
use crate::allocator::HeapStats;

pub struct SystemStats {
    timer_ticks: u64,
    keyboard_interrupts: u64,
    heap_stats: Option<HeapStats>,
}

impl SystemStats {
    const fn new() -> Self {
        SystemStats {
            timer_ticks: 0,
            keyboard_interrupts: 0,
            heap_stats: None,
        }
    }

    pub fn increment_timer(&mut self) {
        self.timer_ticks += 1;
        // Update heap stats periodically
        if self.timer_ticks % 100 == 0 {
            self.update_heap_stats();
        }
    }

    pub fn increment_keyboard(&mut self) {
        self.keyboard_interrupts += 1;
    }

    pub fn get_timer_ticks(&self) -> u64 {
        self.timer_ticks
    }

    pub fn get_keyboard_interrupts(&self) -> u64 {
        self.keyboard_interrupts
    }

    pub fn update_heap_stats(&mut self) {
        self.heap_stats = Some(crate::allocator::get_heap_stats());
    }

    pub fn get_heap_stats(&self) -> Option<&HeapStats> {
        self.heap_stats.as_ref()
    }
}

lazy_static! {
    pub static ref SYSTEM_STATS: Mutex<SystemStats> = Mutex::new(SystemStats::new());
}
