use volatile::Volatile;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

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

                // Always write to the last row
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                // Clear everything above the last row
                for r in 0..BUFFER_HEIGHT - 1 {
                    self.clear_row(r);
                }

                self.buffer.chars[row][col].write(colored_char);
                self.column_position += 1;
            }
        }
    }

    fn move_to_next_line(&mut self) {
        // Clear the row above the last row
        for row in 0..BUFFER_HEIGHT - 1 {
            self.clear_row(row);
        }
        // Clear the last row
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn init_cursor_position(&mut self) {
        // Clear everything
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        // Start at the bottom
        self.column_position = 0;
    }

    // ... (keep other methods the same) ...
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new({
        let mut writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
        // Initialize cursor at the bottom
        writer.init_cursor_position();
        writer
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn set_color(foreground: Color, background: Color) {
    WRITER.lock().change_color(foreground, background);
}

pub fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 0..BUFFER_HEIGHT {
        writer.clear_row(row);
    }
    writer.column_position = 0;
}
