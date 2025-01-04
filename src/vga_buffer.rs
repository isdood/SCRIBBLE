// In vga_buffer.rs
pub struct Writer {
    column_position: usize,
    pub(crate) color_code: ColorCode,  // Make color_code accessible to crate
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn change_color(&mut self, foreground: Color, background: Color) {
        let old_color = self.color_code;
        self.color_code = ColorCode::new(foreground, background);

        // Debug color changes
        use crate::serial_println;
        serial_println!("Color changed from {:?} to {:?}",
                        (old_color.0 & 0x0F), foreground as u8);
    }

    pub fn get_current_color(&self) -> Color {
        match self.color_code.0 & 0x0F {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            _ => Color::White,
        }
    }

    // ... rest of the implementation stays the same
}
