use volatile::Volatile;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;
use core::sync::atomic::{AtomicU8, Ordering};

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

impl Color {
    fn from_u8(value: u8) -> Color {
        match value & 0x0F {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            _ => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    pub fn get_foreground(&self) -> Color {
        Color::from_u8(self.0 & 0x0F)
    }

    pub fn get_background(&self) -> Color {
        Color::from_u8((self.0 >> 4) & 0x0F)
    }
}

struct ColorState {
    system_color: AtomicU8,
    input_color: AtomicU8,
    current_color: AtomicU8,
}

impl ColorState {
    const fn new() -> Self {
        Self {
            system_color: AtomicU8::new(Color::White as u8),
            input_color: AtomicU8::new(Color::Yellow as u8),
            current_color: AtomicU8::new(Color::White as u8),
        }
    }
}

lazy_static! {
    static ref COLOR_STATE: ColorState = ColorState::new();
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
    pub(crate) color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn change_color(&mut self, foreground: Color, background: Color) {
        let old_color = self.color_code.get_foreground();
        self.color_code = ColorCode::new(foreground, background);
        COLOR_STATE.current_color.store(foreground as u8, Ordering::SeqCst);

        use crate::serial_println;
        serial_println!("Color changed from {:?} to {:?}", old_color, foreground);
    }

    pub fn set_system_color(&mut self) {
        let fg = Color::from_u8(COLOR_STATE.system_color.load(Ordering::SeqCst));
        self.change_color(fg, Color::Black);
    }

    pub fn set_input_color(&mut self) {
        let fg = Color::from_u8(COLOR_STATE.input_color.load(Ordering::SeqCst));
        self.change_color(fg, Color::Black);
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

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
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
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
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

pub fn _print_input(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.set_input_color();
        writer.write_fmt(args).unwrap();
    });
}

// src/vga_buffer.rs

// ... (previous code remains the same until the macros)

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
