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
    row_position: usize,
    color_code: ColorCode,
    cursor_color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
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

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            // Scroll everything up
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;

        // Write prompt at start of new line
        self.write_string("> ");
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

    fn update_cursor(&mut self) {
        let pos = self.row_position * BUFFER_WIDTH + self.column_position;

        // Change current character to white (cursor)
        let current_char = self.buffer.chars[self.row_position][self.column_position].read();
        self.buffer.chars[self.row_position][self.column_position].write(ScreenChar {
            ascii_character: current_char.ascii_character,
            color_code: self.cursor_color,
        });

        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4: Port<u8> = Port::new(0x3D4);
            let mut port_3d5: Port<u8> = Port::new(0x3D5);

            // Update cursor position
            port_3d4.write(0x0F_u8);
            port_3d5.write((pos & 0xFF) as u8);
            port_3d4.write(0x0E_u8);
            port_3d5.write(((pos >> 8) & 0xFF) as u8);
        }
    }

    pub fn backspace(&mut self) {
        // Prevent backspacing over prompt ("> ")
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
            // Move to end of previous line
            self.row_position -= 1;
            self.column_position = BUFFER_WIDTH - 1;

            // Find the last non-space character
            while self.column_position > 0 {
                let char = self.buffer.chars[self.row_position][self.column_position - 1].read();
                if char.ascii_character != b' ' {
                    break;
                }
                self.column_position -= 1;
            }

            self.buffer.chars[self.row_position][self.column_position].write(blank);
        }
        self.update_cursor();
    }

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        self.buffer.chars[self.row_position][self.column_position].write(blank);
        self.update_cursor();

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
                                                      cursor_color: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn backspace() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().backspace();
    });
}

pub fn enable_cursor() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4: Port<u8> = Port::new(0x3D4);
            let mut port_3d5: Port<u8> = Port::new(0x3D5);

            // Set cursor shape to underscore (lines 14-15)
            port_3d4.write(0x0A_u8);
            port_3d5.write(0x0E_u8);  // Start scan line
            port_3d4.write(0x0B_u8);
            port_3d5.write(0x0F_u8);  // End scan line

            // Enable cursor
            port_3d4.write(0x0A_u8);
            let current = port_3d5.read();
            port_3d5.write(current & !0x20);
        }
    });
}

pub fn set_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().color_code = ColorCode::new(foreground, background);
    });
}

pub fn clear_screen() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: writer.color_code,
        };

        // Clear entire screen
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                writer.buffer.chars[row][col].write(blank);
            }
        }

        // Reset position
        writer.row_position = 0;
        writer.column_position = 0;

        // Enable cursor and write prompt
        enable_cursor();
        writer.write_string("> ");
    });
}

pub fn init() {
    enable_cursor();
    clear_screen();
}
