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
    prompt_row: usize,  // Add this field
    color_code: ColorCode,
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
            let mut port_3d4: Port<u8> = Port::new(0x3D4);
            let mut port_3d5: Port<u8> = Port::new(0x3D5);

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
        self.prompt_row = row;
    }

    // Update backspace to use prompt_row
    pub fn backspace(&mut self) {
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
        prompt_row: 0,  // Add this field
        color_code: ColorCode::new(Color::Green, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub fn enable_cursor() {
    use x86_64::instructions::interrupts;
    use x86_64::instructions::port::Port;

    interrupts::without_interrupts(|| {
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port_3d4: Port<u8> = Port::new(0x3D4);
            let mut port_3d5: Port<u8> = Port::new(0x3D5);
            let mut port_3c0: Port<u8> = Port::new(0x3C0);
            let mut port_3da: Port<u8> = Port::new(0x3DA);

            // Reset attribute controller
            port_3da.read();  // Reset flip-flop
            port_3c0.write(0x20_u8);  // Enable video

            // Set cursor shape
            port_3d4.write(0x0A_u8);
            port_3d5.write(0x00_u8);  // Start scan line
            port_3d4.write(0x0B_u8);
            port_3d5.write(0x0F_u8);  // End scan line (full block)

    // Enable cursor with high intensity
    port_3d4.write(0x0A_u8);
    let current = port_3d5.read();
    port_3d5.write(current & !0x20);

    // Set cursor color to white
    port_3c0.write(0x0D_u8);  // Select cursor color register
    port_3c0.write(0x0F_u8);  // Set to white (intensity + RGB)
        }
    });
}

pub fn init() {
    use x86_64::instructions::interrupts;
    use x86_64::instructions::port::Port;

    interrupts::without_interrupts(|| {
        unsafe {
            // Initialize VGA ports
            let mut port_3d4: Port<u8> = Port::new(0x3D4);
            let mut port_3d5: Port<u8> = Port::new(0x3D5);
            let mut port_3c0: Port<u8> = Port::new(0x3C0);
            let mut port_3ce: Port<u8> = Port::new(0x3CE);
            let mut port_3cf: Port<u8> = Port::new(0x3CF);

            // Reset sequencer
            let mut port_3c4: Port<u8> = Port::new(0x3C4);
            let mut port_3c5: Port<u8> = Port::new(0x3C5);
            port_3c4.write(0x00);
            port_3c5.write(0x03);

            // Set up VGA registers for text mode
            // Misc Output Register (Read at 3CC, Write at 3C2)
            let mut port_3c2: Port<u8> = Port::new(0x3C2);
            port_3c2.write(0x67); // Enable video and set clock

            // Sequencer registers
            let sequencer_data: [(u8, u8); 5] = [
                (0x00, 0x03), // Reset
                                   (0x01, 0x01), // Clock Mode
                                   (0x02, 0x03), // Plane Enable
                                   (0x03, 0x00), // Character Map Select
                                   (0x04, 0x02), // Memory Mode
            ];

            for (idx, val) in sequencer_data.iter() {
                port_3c4.write(*idx);
                port_3c5.write(*val);
            }

            // Graphics registers
            let graphics_data: [(u8, u8); 9] = [
                (0x00, 0x00), // Set/Reset
                                   (0x01, 0x00), // Enable Set/Reset
                                   (0x02, 0x00), // Color Compare
                                   (0x03, 0x00), // Data Rotate
                                   (0x04, 0x00), // Read Map Select
                                   (0x05, 0x10), // Graphics Mode
                                   (0x06, 0x0E), // Misc
                                   (0x07, 0x00), // Color Don't Care
                                   (0x08, 0xFF), // Bit Mask
            ];

            for (idx, val) in graphics_data.iter() {
                port_3ce.write(*idx);
                port_3cf.write(*val);
            }

            // Attribute registers
            let attr_data: [(u8, u8); 21] = [
                (0x00, 0x00), // Palette 0
                                   (0x01, 0x01), // Palette 1
                                   (0x02, 0x02), // Palette 2
                                   (0x03, 0x03), // Palette 3
                                   (0x04, 0x04), // Palette 4
                                   (0x05, 0x05), // Palette 5
                                   (0x06, 0x06), // Palette 6
                                   (0x07, 0x07), // Palette 7
                                   (0x08, 0x08), // Palette 8
                                   (0x09, 0x09), // Palette 9
                                   (0x0A, 0x0A), // Palette A
                                   (0x0B, 0x0B), // Palette B
                                   (0x0C, 0x0C), // Palette C
                                   (0x0D, 0x0D), // Palette D
                                   (0x0E, 0x0E), // Palette E
                                   (0x0F, 0x0F), // Palette F
                                   (0x10, 0x0C), // Mode Control
                                   (0x11, 0x00), // Overscan Color
                                   (0x12, 0x0F), // Color Plane Enable
                                   (0x13, 0x08), // Horizontal Pixel Panning
                                   (0x14, 0x00), // Color Select
            ];

            for (idx, val) in attr_data.iter() {
                port_3c0.write(*idx);
                port_3c0.write(*val);
            }

            // Enable display
            port_3c0.write(0x20);

            // Clear the screen buffer
            let buffer = 0xb8000 as *mut u8;
            for i in 0..(BUFFER_HEIGHT * BUFFER_WIDTH * 2) {
                *buffer.offset(i as isize) = 0;
            }
        }
    });

    // Initialize the writer with default values
    let mut writer = WRITER.lock();
    writer.column_position = 0;
    writer.row_position = 0;
    writer.prompt_row = 0;
    writer.color_code = ColorCode::new(Color::Green, Color::Black);
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
        writer.row_position = 0;
        writer.column_position = 0;
    });
}
