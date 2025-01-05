use volatile::Volatile;
use core::fmt::{self, Write};
use spin::Mutex;
use lazy_static::lazy_static;
use core::str;

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
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn enable_cursor(&mut self) {
        unsafe {
            use x86_64::instructions::port::Port;

            let mut port_3d4 = Port::new(0x3D4);
            let mut port_3d5 = Port::new(0x3D5);

            // Enable cursor (clear bit 5)
            port_3d4.write(0x0A_u8);
            let cur_state = port_3d5.read() as u8;
            port_3d5.write((cur_state & !0x20) as u8);

            // Set cursor shape (make it white by using scanlines 14-15)
            port_3d4.write(0x0A_u8);
            port_3d5.write(14_u8);  // Start scanline
            port_3d4.write(0x0B_u8);
            port_3d5.write(15_u8);  // End scanline
        }
    }

    fn update_cursor(&mut self) {
        let pos = self.row_position * BUFFER_WIDTH + self.column_position;
        unsafe {
            use x86_64::instructions::port::Port;

            let mut port_3d4 = Port::new(0x3D4);
            let mut port_3d5 = Port::new(0x3D5);

            port_3d4.write(0x0F_u8);
            port_3d5.write((pos & 0xFF) as u8);
            port_3d4.write(0x0E_u8);
            port_3d5.write(((pos >> 8) & 0xFF) as u8);
        }
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

    pub fn backspace(&mut self) {
        // Check if we're at the prompt position ("> ")
        let is_at_prompt = self.column_position <= 2 &&
        self.buffer.chars[self.row_position][0].read().ascii_character == b'>' &&
        self.buffer.chars[self.row_position][1].read().ascii_character == b' ';

        if is_at_prompt {
            return;
        }

        // Don't backspace if we're at column 0 and row 0
        if self.row_position == 0 && self.column_position == 0 {
            return;
        }

        // Move cursor back
        if self.column_position > 0 {
            self.column_position -= 1;
            // Create a blank character
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            };
            // Clear the character at the current position
            self.buffer.chars[self.row_position][self.column_position].write(blank);
        } else if self.row_position > 0 {
            // Move to previous line
            self.row_position -= 1;
            // Find the last non-empty character in the previous line
            let mut last_col = BUFFER_WIDTH - 1;
            while last_col > 2 { // Start after prompt
                let char = self.buffer.chars[self.row_position][last_col].read();
                if char.ascii_character != b' ' {
                    break;
                }
                last_col -= 1;
            }
            self.column_position = last_col + 1;
            if self.column_position >= BUFFER_WIDTH {
                self.column_position = BUFFER_WIDTH - 1;
            }
        }

        self.update_cursor();
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            },
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                self.buffer.chars[self.row_position][self.column_position].write(colored_char);
                self.column_position += 1;
                self.update_cursor();
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

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;

        // Write prompt with proper colors
        let prompt = "> ";
        for byte in prompt.bytes() {
            let colored_char = ScreenChar {
                ascii_character: byte,
                color_code: self.color_code,
            };
            self.buffer.chars[self.row_position][self.column_position].write(colored_char);
            self.column_position += 1;
        }

        self.update_cursor();
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
        row_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Public interface functions
pub fn backspace() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().backspace();
    });
}

pub fn enable_cursor() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().enable_cursor();
    });
}

pub fn clear_screen() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.row_position = BUFFER_HEIGHT - 1;
        writer.column_position = 0;
        writer.update_cursor();
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
