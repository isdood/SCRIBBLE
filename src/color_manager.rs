use spin::Mutex;
use lazy_static::lazy_static;
use crate::vga_buffer::Color;

pub struct ColorManager {
    system_color: Color,
    input_color: Color,
}

impl ColorManager {
    pub const fn new() -> Self {
        Self {
            system_color: Color::White,
            input_color: Color::Yellow,
        }
    }

    pub fn get_system_color(&self) -> Color {
        self.system_color
    }

    pub fn get_input_color(&self) -> Color {
        self.input_color
    }
}

lazy_static! {
    pub static ref COLOR_MANAGER: Mutex<ColorManager> = Mutex::new(ColorManager::new());
}
