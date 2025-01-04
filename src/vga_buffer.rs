pub struct Writer {
    column_position: usize,
    row_position: usize,  // Add row position tracking
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    // ... other methods remain the same ...

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                // Always write at current row position
                self.buffer.chars[self.row_position][self.column_position].write(colored_char);
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        // If not at bottom, move down
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            // At bottom, scroll content up
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;
    }

    // Move cursor to bottom of screen
    fn move_to_bottom(&mut self) {
        self.row_position = BUFFER_HEIGHT - 1;
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        self.move_to_bottom();  // Ensure we're at bottom before writing
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            self.buffer.chars[self.row_position][self.column_position].write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
        }
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new({
        let mut writer = Writer {
            column_position: 0,
            row_position: BUFFER_HEIGHT - 1,  // Start at bottom
            color_code: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
        // Clear screen on initialization
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.move_to_bottom();  // Ensure we start at bottom
        writer
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_to_bottom();  // Move to bottom before printing
        writer.write_fmt(args).unwrap();
    });
}
