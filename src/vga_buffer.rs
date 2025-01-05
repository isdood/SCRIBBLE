use core::fmt::{self, Write};
use volatile::Volatile;
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

    pub fn enable_cursor(&mut self) {
        unsafe {
        use x86_64::instructions::port::Port;
        let mut port_3d4 = Port::new(0x3D4);
        let mut port_3d5 = Port::new(0x3D5);

        // First disable cursor
        port_3d4.write(0x0A_u8);
        port_3d5.write(0x20_u8);

        // Set cursor shape to underscore
        port_3d4.write(0x0A_u8);
        port_3d5.write(0x0F_u8);  // Start at scan line 15 (bottom)
        port_3d4.write(0x0B_u8);
        port_3d5.write(0x0F_u8);  // End at scan line 15 (bottom)

        // Enable cursor
        port_3d4.write(0x0A_u8);
        let cur_state = port_3d5.read() as u8;
        port_3d5.write(cur_state & !0x20);
        }
    }
    pub fn update_cursor(&mut self) {
        let pos = self.row_position * BUFFER_WIDTH + self.column_position;

        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4 = Port::new(0x3D4);
            let mut port_3d5 = Port::new(0x3D5);

            port_3d4.write(0x0F_u8);
            port_3d5.write((pos & 0xFF) as u8);
            port_3d4.write(0x0E_u8);
            port_3d5.write(((pos >> 8) & 0xFF) as u8);

            // Set white background for cursor position (keeping this for the white color)
            let current_char = self.buffer.chars[self.row_position][self.column_position].read();
            let white_bg_char = ScreenChar {
                ascii_character: current_char.ascii_character,
                color_code: ColorCode::new(Color::Black, Color::White),
            };
            self.buffer.chars[self.row_position][self.column_position].write(white_bg_char);
        }
    }

    // Backspace functionality
    fn backspace(&mut self) {
        if self.row_position == 0 && self.column_position <= 2 {
            return;
        }

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        if self.column_position > 0 {
            // Simple backspace within current line
            self.column_position -= 1;
            self.buffer.chars[self.row_position][self.column_position].write(blank);
        } else if self.row_position > 0 {
            // Moving to previous line
            let current_row = self.row_position;
            self.row_position -= 1;

            // Save first line content if we're moving back to it
            let mut first_line_content = [blank; BUFFER_WIDTH];
            if self.row_position == 0 {
                for i in 0..BUFFER_WIDTH {
                    first_line_content[i] = self.buffer.chars[0][i].read();
                }
            }

            // Find the last non-space character in previous line
            self.column_position = BUFFER_WIDTH;
            while self.column_position > 0 {
                let char = self.buffer.chars[self.row_position][self.column_position - 1].read();
                if char.ascii_character != b' ' {
                    break;
                }
                self.column_position -= 1;
            }

            // Clear the current (old) row
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[current_row][col].write(blank);
            }

            // If moving back to first line, restore its content
            if self.row_position == 0 {
                for i in 0..BUFFER_WIDTH {
                    self.buffer.chars[0][i].write(first_line_content[i]);
                }
                self.column_position = 2;  // Start after prompt
                while self.column_position < BUFFER_WIDTH {
                    let char = self.buffer.chars[0][self.column_position].read();
                    if char.ascii_character == b' ' {
                        break;
                    }
                    self.column_position += 1;
                }
            }
        }
    }

        self.update_cursor();
    }

        self.update_cursor();
    }

    pub fn new_line(&mut self) {
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

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        // Set text color
        self.color_code = ColorCode::new(foreground, background);

        // Set cursor color to white
        let cursor_color = ColorCode::new(Color::White, Color::White);
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4 = Port::new(0x3D4);
            let mut port_3d5 = Port::new(0x3D5);

            // Select cursor color register
            port_3d4.write(0x0E_u8);
            port_3d5.write(cursor_color.0);
        }
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

pub fn clear_screen() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.row_position = 0;
        writer.column_position = 0;
        writer.update_cursor();
    });
}

pub fn set_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().set_color(foreground, background);
    });
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
