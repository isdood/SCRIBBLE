// src/vga_buffer.rs
use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

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

// src/vga_buffer.rs
// ... previous imports and Color enum remain the same ...

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);  // Make ColorCode public

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {  // Make new public
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// ... Buffer struct remains the same ...

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,  // Make color_code public
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte_colored(&mut self, byte: u8, color_code: ColorCode) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.column_position = 0,
            b'\t' => {
                for _ in 0..4 {
                    self.write_byte_colored(b' ', color_code);
                }
            },
            b'\x08' => self.backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
                self.update_cursor();
            }
        }
    }

    // Keep existing write_byte method but make it use write_byte_colored
    pub fn write_byte(&mut self, byte: u8) {
        self.write_byte_colored(byte, self.color_code);
    }

    // ... rest of the implementation remains the same ...
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Add a function to get current color
pub fn get_current_color() -> ColorCode {
    WRITER.lock().color_code
}


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub fn set_color(foreground: Color, background: Color) {
    WRITER.lock().color_code = ColorCode::new(foreground, background);
}

pub fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 0..BUFFER_HEIGHT {
        writer.clear_row(row);
    }
    writer.column_position = 0;
    writer.update_cursor();
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
