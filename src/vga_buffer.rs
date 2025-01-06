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

// In src/vga_buffer.rs

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    prompt_length: usize,
    is_new_line: bool, // Add this new field to track if we're starting a new line
}

impl Writer {
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
        self.is_new_line = true; // Mark that we're at a new line
        self.update_cursor();
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            0x08 => self.backspace(),
            b'\n' => self.new_line(),
            byte => {
                // If we're at a new line, print the prompt first
                if self.is_new_line {
                    self.write_prompt();
                    self.is_new_line = false;
                }

                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                    // The new_line() call will have set is_new_line to true,
                    // so the next iteration will print the prompt
                    return;
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

    pub fn write_prompt(&mut self) {
        // Don't allow writing prompt if we're in the middle of a line
        if self.column_position == 0 {
            self.write_string("> ");
            self.column_position = self.prompt_length;
            self.update_cursor();
        }
    }

    pub fn backspace(&mut self) {
        // Check if we're at the start of a line with a prompt
        if self.column_position <= self.prompt_length &&
            (self.row_position == 0 || self.column_position == self.prompt_length) {
                return; // Prevent backspace at or before prompt
            }

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
                // When moving to the previous line, go to the last character
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

// Update the WRITER initialization
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let mut writer = Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            prompt_length: 2,
            is_new_line: true, // Initialize as true to print first prompt
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
