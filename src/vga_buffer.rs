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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
    // ... existing methods ...

    pub fn get_column_position(&self) -> usize {
        self.column_position
    }

    pub fn set_column_position(&mut self, position: usize) {
        self.column_position = position;
    }

    pub fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            let color_code = self.color_code;
            let col = self.column_position;

            // Get the current row (last row)
            let row = BUFFER_HEIGHT - 1;

            // Write space character at current position
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code,
            });
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {  // Make public
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        self.update_cursor();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            let row = BUFFER_HEIGHT - 1;
            self.buffer.chars[row][self.column_position].write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
            self.update_cursor();
        }
    }

    fn update_cursor(&mut self) {
        use x86_64::instructions::port::Port;

        let pos = (BUFFER_HEIGHT - 1) * BUFFER_WIDTH + self.column_position;

        let mut port_3d4 = Port::new(0x3D4);
        let mut port_3d5 = Port::new(0x3D5);

        unsafe {
            port_3d4.write(0x0Fu8);
            port_3d5.write((pos & 0xFF) as u8);
            port_3d4.write(0x0Eu8);
            port_3d5.write(((pos >> 8) & 0xFF) as u8);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
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
