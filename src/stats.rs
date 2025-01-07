use spin::Mutex;


pub struct SystemStats {
    timer_ticks: u64,
    keyboard_interrupts: u64,
}

impl SystemStats {
    const fn new() -> Self {
        SystemStats {
            timer_ticks: 0,
            keyboard_interrupts: 0,
        }
    }

    pub fn increment_timer(&mut self) {
        self.timer_ticks += 1;
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
}

lazy_static! {
    pub static ref SYSTEM_STATS: Mutex<SystemStats> = Mutex::new(SystemStats::new());
}
