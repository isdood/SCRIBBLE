use core::fmt::{self, Write};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;

//\\        CONSTANTS        //\\
/////////////////////////////////
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const CURSOR_PORT_CTRL: u16 = 0x3D4;
const CURSOR_PORT_DATA: u16 = 0x3D5;

const CURSOR_START_LINE: u8 = 0;   // Start from top of character
const CURSOR_END_LINE: u8 = 15;    // End at bottom of character
////////////////////////////////

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
    fn write_char(&mut self, screen_char: ScreenChar) {
        let value = (u16::from(screen_char.color_code.0) << 8) | u16::from(screen_char.ascii_character);
        unsafe {
            core::ptr::write_volatile(&mut self.0, value);
        }
    }

    fn read_char(&self) -> ScreenChar {
        let value = unsafe {
            core::ptr::read_volatile(&self.0)
        };
        ScreenChar {
            ascii_character: (value & 0xFF) as u8,
            color_code: ColorCode((value >> 8) as u8),
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
    prompt_length: usize,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            0x08 => self.backspace(),
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

    pub fn backspace(&mut self) {
        let at_prompt_position = self.column_position <= self.prompt_length && self.row_position == 0;

        if !at_prompt_position {
            if self.column_position > 0 {
                self.column_position -= 1;
                let blank = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
                self.buffer.chars[self.row_position][self.column_position].write_char(blank);
                self.update_cursor();
            } else if self.row_position > 0 {
                self.row_position -= 1;
                self.column_position = BUFFER_WIDTH - 1;
                let blank = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
                self.buffer.chars[self.row_position][self.column_position].write_char(blank);
                self.update_cursor();
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' | 0x08 => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_prompt(&mut self) {
        self.write_string("> ");
        self.column_position = self.prompt_length;
        self.update_cursor();
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
        self.write_string("> ");
        self.column_position = self.prompt_length;
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
        let pos = (self.row_position * BUFFER_WIDTH + self.column_position) as u16;

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

            control_port.write(0x0A_u8);
            data_port.write(0x20_u8);

            for _ in 0..100000 {
                core::hint::spin_loop();
            }

            control_port.write(0x0A_u8);
            data_port.write(CURSOR_START_LINE);
            control_port.write(0x0B_u8);
            data_port.write(CURSOR_END_LINE);

            control_port.write(0x0A_u8);
            data_port.write(CURSOR_START_LINE);
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
            prompt_length: 2,
        };
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.enable_cursor();
        writer.update_cursor();
        Mutex::new(writer)
    };
}

pub fn backspace() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().backspace();
    });
}

pub fn set_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.color_code = ColorCode::new(foreground, background);
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

pub fn write_prompt() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_prompt();
    });
}

pub fn enable_cursor() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().enable_cursor();
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
