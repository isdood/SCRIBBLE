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
    prompt_row: usize,  // Add this to track prompt line
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
            // Scroll the screen
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.prompt_row = self.prompt_row.saturating_sub(1); // Update prompt row when scrolling
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
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn update_cursor(&mut self) {
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

    pub fn backspace(&mut self) {
        // Only protect prompt character on prompt lines
        let is_at_prompt = self.column_position <= 2 && self.row_position == self.prompt_row;
        if is_at_prompt {
            return;
        }

        if self.column_position > 0 {
            self.column_position -= 1;
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            };
            self.buffer.chars[self.row_position][self.column_position].write(blank);
            self.update_cursor();
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
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
                                                      prompt_row: 3,  // Initialize prompt row
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });

}

pub fn enable_cursor() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4 = Port::new(0x3D4);
            let mut port_3d5 = Port::new(0x3D5);

            // Set cursor shape (block cursor)
            port_3d4.write(0x0A_u8);
            port_3d5.write(0x00_u8);  // Start scan line (top)
    port_3d4.write(0x0B_u8);
    port_3d5.write(0x0F_u8);  // End scan line (bottom)

    // Enable cursor and set intensity
    port_3d4.write(0x0A_u8);
    port_3d5.write(0x00_u8);  // Enable cursor and set to high intensity

    // Set cursor color to white using attribute controller
    let mut port_3c0 = Port::new(0x3C0);
    port_3c0.write(0x0D_u8);  // Select cursor color register
    port_3c0.write(0x0F_u8);  // Set to white (high intensity)
        }
    });
}

pub fn init() {
    enable_cursor();

    // Reset the screen state
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.row_position = 0;
        writer.column_position = 0;
        writer.prompt_row = 3;  // Set initial prompt row
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
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.row_position = 2;  // Start at line 3 (after boot messages)
    writer.column_position = 0;
    writer.write_string("> ");  // Write prompt
    });
}
