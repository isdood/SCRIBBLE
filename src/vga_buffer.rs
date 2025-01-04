use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

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
    Yellow = 14,    // We'll use this for light orange
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
pub struct ColorTheme {
    pub text: Color,
    pub heading: Color,
    pub background: Color,
}

impl ColorTheme {
    pub fn get_text(&self) -> Color { self.text }
    pub fn get_background(&self) -> Color { self.background }
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// Define color themes
pub struct ColorTheme {
    text: Color,
    heading: Color,
    background: Color,
}

lazy_static! {
    static ref THEME: ColorTheme = ColorTheme {
        text: Color::Yellow,      // Light orange (using Yellow for a light orange look)
        heading: Color::LightGreen,
        background: Color::Black,
    };
}

impl Writer {
    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }

    pub fn write_styled(&mut self, s: &str, color: Color) {
        let original_color = self.color_code;
        self.set_color(color, THEME.background);
        self.write_string(s);
        self.color_code = original_color;
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
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Public interface for color manipulation
pub fn set_print_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().set_color(foreground, background);
    });
}

// Print macros with color support
#[macro_export]
macro_rules! print_colored {
    ($color:expr, $($arg:tt)*) => ({
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
        interrupts::without_interrupts(|| {
            let mut writer = $crate::vga_buffer::WRITER.lock();
            writer.set_color($color, $crate::vga_buffer::THEME.background);
            writer.write_fmt(format_args!($($arg)*)).unwrap();
            writer.set_color($crate::vga_buffer::THEME.text, $crate::vga_buffer::THEME.background);
        });
    });
}

#[macro_export]
macro_rules! print_colored {
    ($color:expr, $($arg:tt)*) => ({
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
        interrupts::without_interrupts(|| {
            let mut writer = $crate::vga_buffer::WRITER.lock();
            let theme = &$crate::vga_buffer::THEME;
            writer.set_color($color, theme.get_background());
            writer.write_fmt(format_args!($($arg)*)).unwrap();
            writer.set_color(theme.get_text(), theme.get_background());
        });
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
