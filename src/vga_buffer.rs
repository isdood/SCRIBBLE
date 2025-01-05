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

            // Set cursor shape to underscore
            port_3d4.write(0x09_u8);  // Set maximum scan line
            port_3d5.write(0x0F_u8);  // Set to 15 for normal text size

            port_3d4.write(0x0A_u8);
            port_3d5.write(0x0E_u8);  // Start scan line 14
            port_3d4.write(0x0B_u8);
            port_3d5.write(0x0F_u8);  // End scan line 15

            // Set high-intensity cursor
            port_3d4.write(0x0A_u8);
            port_3d5.write(port_3d5.read() & 0xDF);  // Clear bit 5 for high intensity
        }
        self.update_cursor();
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

    // ... (other Writer methods remain the same) ...
}

// Remove prompt from WRITER initialization
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub fn clear_screen() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        // First set text size
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4 = Port::new(0x3D4);
            let mut port_3d5 = Port::new(0x3D5);

            // Set normal text size
            port_3d4.write(0x09_u8);
            port_3d5.write(0x0F_u8);
        }

        // Clear screen
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.row_position = 0;
        writer.column_position = 0;

        // Write single prompt
        writer.write_string("> ");

        // Setup cursor properly
        writer.enable_cursor();
    });
}
