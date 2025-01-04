use volatile::Volatile;
use core::fmt::{self, Write};
use spin::Mutex;
use lazy_static::lazy_static;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// ... (keep Color enum and other structs the same) ...

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.move_to_next_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.move_to_next_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                // Clear everything above the last row
                for r in 0..BUFFER_HEIGHT - 1 {
                    self.clear_current_row(r);
                }

                self.buffer.chars[row][col].write(colored_char);
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn clear_current_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn move_to_next_line(&mut self) {
        // Clear the row above the last row
        for row in 0..BUFFER_HEIGHT - 1 {
            self.clear_current_row(row);
        }
        // Clear the last row
        self.clear_current_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn change_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }

    pub fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            let color_code = self.color_code;
            let col = self.column_position;

            let row = BUFFER_HEIGHT - 1;
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code,
            });
        }
    }

    pub fn init_cursor_position(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_current_row(row);
        }
        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new({
        let mut writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
        writer.init_cursor_position();
        writer
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn set_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().change_color(foreground, background);
    });
}

pub fn clear_screen() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for row in 0..BUFFER_HEIGHT {
            writer.clear_current_row(row);
        }
        writer.column_position = 0;
    });
}
