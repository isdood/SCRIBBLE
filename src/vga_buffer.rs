use volatile::Volatile;
use core::fmt::{self, Write};
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

            // First completely disable cursor
            port_3d4.write(0x0A_u8);
            port_3d5.write(0x20_u8);

            // Set cursor shape - just the bottom line for underscore
            port_3d4.write(0x0A_u8);
            port_3d5.write(0x0E_u8);  // Start scan line
            port_3d4.write(0x0B_u8);
            port_3d5.write(0x0F_u8);  // End scan line

            // Enable cursor and force high intensity
            port_3d4.write(0x0A_u8);
            port_3d5.write(0x00_u8);  // Force high intensity white cursor
        }
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }

    fn backspace(&mut self) {
        if self.row_position == 0 && self.column_position <= 2 {
            return;
        }

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        if self.column_position > 0 {
            self.column_position -= 1;
            self.buffer.chars[self.row_position][self.column_position].write(blank);
        } else if self.row_position > 0 {
            // Save current row for clearing
            let old_row = self.row_position;

            // Move to end of previous row
            self.row_position -= 1;
            self.column_position = BUFFER_WIDTH - 1;

            // Find actual last character
            while self.column_position > 0 {
                let char = self.buffer.chars[self.row_position][self.column_position - 1].read();
                if char.ascii_character != b' ' {
                    break;
                }
                self.column_position -= 1;
            }

            // Clear entire old row
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[old_row][col].write(blank);
            }

            // Clear any leftover characters in current position
            self.buffer.chars[self.row_position][self.column_position].write(blank);
        }

        self.update_cursor();
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

    fn write_byte(&mut self, byte: u8) {
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

                self.buffer.chars[self.row_position][self.column_position].write(colored_char);
                self.column_position += 1;
                self.update_cursor();
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn initialize(&mut self) {
        // Clear screen first
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.row_position = 0;
        self.column_position = 0;

        // Write prompt ONCE and enable cursor
        self.write_string("> ");
        self.enable_cursor();
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

pub fn set_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().set_color(foreground, background);
    });
}

pub fn enable_cursor() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().enable_cursor();
    });
}

pub fn backspace() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().backspace();
    });
}

pub fn initialize() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().initialize();
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
