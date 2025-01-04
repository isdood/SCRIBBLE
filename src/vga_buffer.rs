impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // Move existing text up and write at bottom
                for row in 1..BUFFER_HEIGHT {
                    for col in 0..BUFFER_WIDTH {
                        let character = self.buffer.chars[row][col].read();
                        self.buffer.chars[row - 1][col].write(character);
                    }
                }

                // Always write at the bottom row
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                self.buffer.chars[row][col].write(colored_char);
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        // Move everything up one line
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        // Clear the bottom line
        self.clear_row(BUFFER_HEIGHT - 1);
        // Reset column position but stay on bottom row
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        // Move to bottom row first
        self.column_position = 0;
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        // Write the string
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

// Modify the WRITER initialization
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new({
        let mut writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
        // Clear screen and set position to bottom
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        // Ensure we're at the bottom row
        writer.column_position = 0;
        writer.write_fmt(args).unwrap();
    });
}
