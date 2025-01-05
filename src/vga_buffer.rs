use volatile::Volatile;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::print;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

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
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
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
    input_color: ColorCode,
    buffer: &'static mut Buffer,
}

trait VolatileAccess<T> {
    unsafe fn read_volatile(&self) -> T;
    unsafe fn write_volatile(&mut self, value: T);
}

impl VolatileAccess<ScreenChar> for Volatile<ScreenChar> {
    unsafe fn read_volatile(&self) -> ScreenChar {
        core::ptr::read_volatile(self as *const Volatile<ScreenChar> as *const ScreenChar)
    }

    unsafe fn write_volatile(&mut self, value: ScreenChar) {
        core::ptr::write_volatile(self as *mut Volatile<ScreenChar> as *mut ScreenChar, value)
    }
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

                let color = if self.is_prompt_position() {
                    self.color_code
                } else {
                    self.input_color
                };

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: color,
                };

                unsafe {
                    (&mut self.buffer.chars[row][col]).write_volatile(colored_char);
                }

                self.column_position += 1;
                self.move_cursor();
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn is_prompt_position(&self) -> bool {
        self.column_position < 2 && unsafe {
            self.buffer.chars[self.row_position][0].read_volatile().ascii_character == b'>'
        }
    }

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    unsafe {
                        let char_val = (&self.buffer.chars[row][col]).read_volatile();
                        (&mut self.buffer.chars[row - 1][col]).write_volatile(char_val);
                    }
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;
        self.move_cursor();
    }

    fn move_cursor(&mut self) {
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

    pub fn get_row_position(&self) -> usize {
        self.row_position
    }

    pub fn set_prompt_row(&mut self, row: usize) {
        if row < BUFFER_HEIGHT {
            self.row_position = row;
            self.column_position = 0;
            self.move_cursor();
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            unsafe {
                (&mut self.buffer.chars[row][col]).write_volatile(blank);
            }
        }
    }

    pub fn backspace(&mut self) {
        // Don't allow backspace over prompt
        if self.column_position <= 2 && self.is_prompt_position() {
            return;
        }

        if self.column_position > 0 {
            self.column_position -= 1;
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.input_color,
            };
            unsafe {
                (&mut self.buffer.chars[self.row_position][self.column_position])
                .write_volatile(blank);
            }
            self.move_cursor();
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
                                                      input_color: ColorCode::new(Color::White, Color::Black),
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

pub fn clear_screen() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.row_position = 0;
        writer.column_position = 0;
        writer.move_cursor();
        drop(writer);
        print!("> ");
    });
}

pub fn set_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().color_code = ColorCode::new(foreground, background);
    });
}

pub fn set_input_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().input_color = ColorCode::new(foreground, background);
    });
}

pub fn enable_cursor() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4 = Port::new(0x3D4);
            let mut port_3d5 = Port::new(0x3D5);

            port_3d4.write(0x0A_u8);
            port_3d5.write(0x0E_u8);
            port_3d4.write(0x0B_u8);
            port_3d5.write(0x0F_u8);

            port_3d4.write(0x0A_u8);
            let current = port_3d5.read();
            port_3d5.write(current & !0x20);
        }
    });
}

pub fn init() {
    enable_cursor();
}
