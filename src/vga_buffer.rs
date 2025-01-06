use core::fmt::{self, Write};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const CURSOR_PORT_CTRL: u16 = 0x3D4;
const CURSOR_PORT_DATA: u16 = 0x3D5;

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

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
struct UnstableMatter(u16);

impl UnstableMatter {
    /// Write a character directly to VGA memory
    fn write_char(&mut self, screen_char: ScreenChar) {
        let value = (u16::from(screen_char.color_code.0) << 8) | u16::from(screen_char.ascii_character);
        self.write(value);
    }

    /// Read a character from VGA memory
    fn read_char(&self) -> ScreenChar {
        let value = self.read();
        ScreenChar {
            ascii_character: (value & 0xFF) as u8,
            color_code: ColorCode((value >> 8) as u8),
        }
    }

    /// Write a raw u16 value to VGA memory
    fn write(&mut self, value: u16) {
        unsafe {
            core::ptr::write_volatile(&mut self.0, value);
        }
    }

    /// Read a raw u16 value from VGA memory
    fn read(&self) -> u16 {
        unsafe {
            core::ptr::read_volatile(&self.0)
        }
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[UnstableMatter; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer.chars[row][col].write_char(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });

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
                    let character = self.buffer.chars[row][col].read_char();
                    self.buffer.chars[row - 1][col].write_char(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;
        self.update_cursor();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write_char(blank);
        }
    }

    pub fn update_cursor(&mut self) {
        let pos = self.row_position * BUFFER_WIDTH + self.column_position;
        unsafe {
            let mut control_port: Port<u8> = Port::new(CURSOR_PORT_CTRL);
            let mut data_port: Port<u8> = Port::new(CURSOR_PORT_DATA);

            control_port.write(0x0F_u8);
            data_port.write((pos & 0xFF) as u8);
            control_port.write(0x0E_u8);
            data_port.write(((pos >> 8) & 0xFF) as u8);
        }
    }

    pub fn enable_cursor(&mut self) {
        unsafe {
            let mut control_port: Port<u8> = Port::new(CURSOR_PORT_CTRL);
            let mut data_port: Port<u8> = Port::new(CURSOR_PORT_DATA);

            // Read current state and clear bit 5 to enable cursor
            control_port.write(0x0A_u8);
            let cur_state = data_port.read() as u8;
            data_port.write((cur_state & !0x20) as u8);

            // Set cursor shape
            control_port.write(0x0A_u8);
            data_port.write(0x0F_u8);  // Changed port_3d5 to data_port
            control_port.write(0x0B_u8);
            data_port.write(0x0F_u8);  // Changed port_3d5 to data_port
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
    pub static ref WRITER: Mutex<Writer> = {
        let mut writer = Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
        writer.enable_cursor();
        Mutex::new(writer)
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
