// src/vga_buffer.rs

use crate::unstable_matter::UnstableMatter;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// debug macro
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        $crate::vga_buffer::WRITER.lock().debug_print(&format_args!($($arg)*).to_string());
    }};
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

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
    const fn new(foreground: Color, background: Color) -> ColorCode {
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
    chars: [[UnstableMatter<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        fn verify_text_mode() -> bool {
            unsafe {
                use x86_64::instructions::port::Port;

                let mut misc_port = Port::new(0x3CC);
                let misc_output = misc_port.read();

                // Bit 0 indicates text/graphics mode
                (misc_output & 1) == 0
            }
        }

        pub fn write_byte(&mut self, byte: u8) {
            if !Self::verify_text_mode() {
                // Try to force text mode
                unsafe {
                    use x86_64::instructions::port::Port;
                    let mut mode_port = Port::new(0x3D4);
                    mode_port.write(0x00_u8);
                    let mut data_port = Port::new(0x3D5);
                    data_port.write(0x03_u8); // Text mode 80x25
                }
            }
        }
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

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
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

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn blink_cursor(&mut self) {
        if self.column_position > 0 {
            let last_pos = self.column_position - 1;
            let row = BUFFER_HEIGHT - 1;
            let screen_char = self.buffer.chars[row][last_pos].read();
            self.buffer.chars[row][last_pos].write(screen_char);
        }
    }

    pub fn debug_print(&mut self, msg: &str) {
        let color_backup = self.color_code;
        self.color_code = ColorCode::new(Color::Yellow, Color::Black);
        self.write_string("[DEBUG] ");
        self.write_string(msg);
        self.write_byte(b'\n');
        self.color_code = color_backup;
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
                                                      buffer: unsafe {
                                                          let buf = &mut *(0xb8000 as *mut Buffer);
                                                          // Validate VGA memory is accessible
                                                          let mut test_char = ScreenChar {
                                                              ascii_character: b'T',
                                                              color_code: ColorCode::new(Color::White, Color::Black),
                                                          };
                                                          buf.chars[0][0].write(test_char);
                                                          test_char = buf.chars[0][0].read();
                                                          if test_char.ascii_character != b'T' {
                                                              // If we can't read/write, panic
                                                              panic!("VGA buffer not accessible");
                                                          }
                                                          buf
                                                      },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        use core::fmt::Write;
        WRITER.lock().write_fmt(args).unwrap();
    });
}
