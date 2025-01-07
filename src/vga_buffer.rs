// IMPORTS //
use core::fmt::{self, Write};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;

// CONSTANTS //

// Software / Harware cursor switch constants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorMode {
    Hardware,
    Software,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

// VGA cursor control registers and values
const CURSOR_PORT_CTRL: u16 = 0x3D4;
const CURSOR_PORT_DATA: u16 = 0x3D5;

// Register numbers
//const CURSOR_START_REG: u8 = 0x0A;
const CURSOR_START_SCANLINE: u8 = 14;  // Determines cursor appearance
const CURSOR_END_SCANLINE: u8 = 15;    // Determines cursor size
const CURSOR_MODE_REGISTER: u8 = 0x0A;
const CURSOR_START_REGISTER: u8 = 0x0B;
const CURSOR_LOCATION_HIGH_REG: u8 = 0x0E;
const CURSOR_LOCATION_LOW_REG: u8 = 0x0F;

// VGA mode cursor colour
const NORMAL_CURSOR: (Color, Color) = (Color::Yellow, Color::Black);
const INSERT_CURSOR: (Color, Color) = (Color::Green, Color::Black);
const SELECT_CURSOR: (Color, Color) = (Color::White, Color::Blue);

//  //

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

#[derive(Debug, Clone, Copy)]
pub struct ProtectedRegion {
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl ProtectedRegion {
    pub fn new(row: usize, start_col: usize, length: usize) -> Self {
        Self {
            row,
            start_col,
            end_col: start_col + length,
        }
    }

    pub fn contains(&self, row: usize, col: usize) -> bool {
        row == self.row && (col >= self.start_col && col < self.end_col)
    }

    pub fn is_before(&self, row: usize, col: usize) -> bool {
        row == self.row && col <= self.start_col
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CursorStyle {
    Block,
    Underscore,
    Line
}

// Writer struct
pub struct Writer {
    pub row_position: usize,
    pub column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    prompt_length: usize,
    is_wrapped: bool,
    pub protected_region: ProtectedRegion,
    previous_cursor_pos: (usize, usize),
    previous_char_color: ColorCode,
    cursor_visible: bool,
    cursor_blink_counter: u8,
    cursor_style: CursorStyle,
    cursor_color: (Color, Color),
    cursor_mode: CursorMode,
    hardware_cursor_enabled: bool,
}

impl Writer {

        // Wrapper helper method
    pub fn needs_wrap(&self) -> bool {
        if self.column_position >= BUFFER_WIDTH {
            if self.row_position >= BUFFER_HEIGHT - 1 {
                // Need to scroll
                true
            } else {
                !self.is_wrapped
            }
        } else {
            false
        }
    }

        // Write byte
        pub fn write_byte(&mut self, byte: u8) {
            use x86_64::instructions::interrupts;

            interrupts::without_interrupts(|| {
                // Force write a character to the first position of the buffer to test
                unsafe {
                    let test_char = ScreenChar {
                        ascii_character: b'T',
                        color_code: ColorCode::new(Color::White, Color::Black),
                    };
                    (*self.buffer).chars[0][0].write_char(test_char);
                }

                match byte {
                    0x08 => {
                        let next_pos = if self.column_position == 0 {
                            if self.row_position > 0 {
                                (self.row_position - 1, BUFFER_WIDTH - 1)
                            } else {
                                (0, self.column_position)
                            }
                        } else {
                            (self.row_position, self.column_position - 1)
                        };

                        if !self.protected_region.contains(next_pos.0, next_pos.1) {
                            self.backspace();
                        }
                    },
                    b'\n' => {
                        self.restore_previous_cursor();
                        self.new_line();
                        self.is_wrapped = false;
                    },
                    byte => {

                        if self.needs_wrap() {
                            self.restore_previous_cursor();
                            self.is_wrapped = true;
                            self.new_line();
                        }

                        // Only write if we're not in protected region
                        if !self.protected_region.contains(self.row_position, self.column_position) {
                            self.buffer.chars[self.row_position][self.column_position].write_char(ScreenChar {
                                ascii_character: byte,
                                color_code: self.color_code,
                            });
                            self.column_position += 1;
                        }
                    }
                }

                self.update_cursor();
            });
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
        // Save current cursor state
        let current_cursor_visible = self.cursor_visible;
        self.cursor_visible = false;

        // Clean up any stray cursors
        self.clean_stray_cursors();
        self.restore_previous_cursor();

        self.column_position = 0;
        self.is_wrapped = false;

        // Set up protected region for new prompt
        self.protected_region = ProtectedRegion::new(self.row_position, 0, self.prompt_length);

        // Write prompt characters in yellow
        let prompt_chars = [b'>', b' '];
        let prompt_color = ColorCode::new(Color::Yellow, Color::Black);

        for (i, &ch) in prompt_chars.iter().enumerate() {
            self.buffer.chars[self.row_position][i].write_char(ScreenChar {
                ascii_character: ch,
                color_code: prompt_color,
            });
        }

        self.column_position = self.prompt_length;

        // Reset color to white for user input
        self.color_code = ColorCode::new(Color::White, Color::Black);

        // Restore cursor visibility and update
        self.cursor_visible = current_cursor_visible;
        self.update_cursor();
    }

    pub fn backspace(&mut self) {

        // Clear current cursor first
        self.restore_previous_cursor();

        // Clean any stray cursors
        self.clean_stray_cursors();

        // Check if we would enter protected region
        let next_pos = if self.column_position == 0 {
            if self.row_position > 0 {
                (self.row_position - 1, BUFFER_WIDTH - 1)
            } else {
                return; // Can't backspace at start of first line
            }
        } else {
            (self.row_position, self.column_position - 1)
        };

        if self.protected_region.contains(next_pos.0, next_pos.1) ||
            self.protected_region.is_before(next_pos.0, next_pos.1) {
                return;
            }

            // Perform backspace
            if self.column_position == 0 && self.row_position > 0 {
                let blank = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
                self.buffer.chars[self.row_position][0].write_char(blank);

                self.row_position -= 1;
                self.column_position = BUFFER_WIDTH - 1;
                self.buffer.chars[self.row_position][self.column_position].write_char(blank);
                self.is_wrapped = true;
            } else if self.column_position > 0 {
                self.column_position -= 1;
                let blank = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
                self.buffer.chars[self.row_position][self.column_position].write_char(blank);
            }

            self.update_cursor();
    }

    fn new_line(&mut self) {
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            self.restore_previous_cursor();

            if self.row_position < BUFFER_HEIGHT - 1 {
                self.row_position += 1;
            } else {
                // Scroll content up efficiently
                for row in 1..BUFFER_HEIGHT {
                    for col in 0..BUFFER_WIDTH {
                        let character = self.buffer.chars[row][col].read_char();
                        self.buffer.chars[row - 1][col].write_char(character);
                    }
                }
                // Clear last row
                self.clear_row(BUFFER_HEIGHT - 1);
            }
            self.column_position = 0;
            self.update_cursor();
        });
    }

    pub fn switch_cursor_mode(&mut self, mode: CursorMode) {
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            match mode {
                CursorMode::Hardware => {
                    // Clean up any software cursor
                    if self.cursor_mode == CursorMode::Software {
                        self.restore_previous_cursor();
                    }

                    // Enable hardware cursor
                    unsafe {
                        let mut control_port: Port<u8> = Port::new(CURSOR_PORT_CTRL);
                        let mut data_port: Port<u8> = Port::new(CURSOR_PORT_DATA);

                        // Set cursor appearance
                        control_port.write(CURSOR_MODE_REGISTER);
                        data_port.write(CURSOR_START_SCANLINE);

                        control_port.write(CURSOR_START_REGISTER);
                        data_port.write(CURSOR_END_SCANLINE);

                        // Set cursor color (yellow on black)
                        let mut attr_port: Port<u8> = Port::new(0x3C0);
                        attr_port.write(0x0B);
                        attr_port.write(ColorCode::new(Color::Yellow, Color::Black).0);
                    }
                    self.hardware_cursor_enabled = true;
                },
                CursorMode::Software => {
                    // Disable hardware cursor
                    unsafe {
                        let mut control_port: Port<u8> = Port::new(CURSOR_PORT_CTRL);
                        let mut data_port: Port<u8> = Port::new(CURSOR_PORT_DATA);

                        control_port.write(CURSOR_MODE_REGISTER);
                        data_port.write(0x20); // Bit 5 set to disable hardware cursor
                    }
                    self.hardware_cursor_enabled = false;
                }
            }

            self.cursor_mode = mode;
            self.update_cursor();
        });
    }

    pub fn update_cursor(&mut self) {
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            match self.cursor_mode {
                CursorMode::Hardware => {
                    if self.hardware_cursor_enabled {
                        let pos = (self.row_position * BUFFER_WIDTH + self.column_position) as u16;
                        unsafe {
                            let mut control_port: Port<u8> = Port::new(CURSOR_PORT_CTRL);
                            let mut data_port: Port<u8> = Port::new(CURSOR_PORT_DATA);

                            // Make sure cursor is enabled
                            control_port.write(CURSOR_MODE_REGISTER);
                            data_port.write(0x0E); // Enable cursor, normal size

                            // Update position
                            control_port.write(CURSOR_LOCATION_HIGH_REG);
                            data_port.write(((pos >> 8) & 0xFF) as u8);
                            control_port.write(CURSOR_LOCATION_LOW_REG);
                            data_port.write((pos & 0xFF) as u8);
                        }
                    }
                },
                CursorMode::Software => {
                    // ... existing software cursor code ...
                }
            }
        });
    }

    pub fn enable_cursor(&mut self) {
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            unsafe {
                let mut control_port: Port<u8> = Port::new(CURSOR_PORT_CTRL);
                let mut data_port: Port<u8> = Port::new(CURSOR_PORT_DATA);

                // Set cursor start scanline
                control_port.write(CURSOR_MODE_REGISTER);
                data_port.write(CURSOR_START_SCANLINE);

                // Set cursor end scanline
                control_port.write(CURSOR_START_REGISTER);
                data_port.write(CURSOR_END_SCANLINE);
            }

            self.cursor_visible = true;
            self.cursor_style = CursorStyle::Underscore;
            self.cursor_color = NORMAL_CURSOR;
            self.previous_cursor_pos = (0, 0);  // Start at a known position
            self.previous_char_color = ColorCode::new(Color::White, Color::Black);

            // Force cursor update
            self.update_cursor();
        });
    }

    pub fn blink_cursor(&mut self) {
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            self.cursor_blink_counter = self.cursor_blink_counter.wrapping_add(1);
            if self.cursor_blink_counter % 30 == 0 {
                self.cursor_visible = !self.cursor_visible;
                if !self.protected_region.contains(self.row_position, self.column_position) {
                    self.update_cursor();
                }
            }
        });
    }

    pub fn set_cursor_color(&mut self, foreground: Color, background: Color) {
        self.cursor_color = (foreground, background);
        self.update_cursor();
    }

    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        self.cursor_style = style;
        self.cursor_color = match style {
            CursorStyle::Block => NORMAL_CURSOR,
            CursorStyle::Underscore => INSERT_CURSOR,
            CursorStyle::Line => SELECT_CURSOR,
        };
        self.update_cursor();
    }

    pub fn restore_previous_cursor(&mut self) {
        let (prev_row, prev_col) = self.previous_cursor_pos;
        if prev_row < BUFFER_HEIGHT && prev_col < BUFFER_WIDTH {
            let prev_char = self.buffer.chars[prev_row][prev_col].read_char();
            // Only restore if it was a cursor character
            if prev_char.ascii_character == b'_' || prev_char.ascii_character == b'|' {
                let original_char = b' ';  // Default to space if we're unsure
                self.buffer.chars[prev_row][prev_col].write_char(ScreenChar {
                    ascii_character: original_char,
                    color_code: self.previous_char_color,
                });
            }
        }
    }

    pub fn clean_stray_cursors(&mut self) {
        // Save current cursor state
        let current_visible = self.cursor_visible;
        self.cursor_visible = false;

        // Clean up current row and the row above
        for row in self.row_position.saturating_sub(1)..=self.row_position {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read_char();
                if char.ascii_character == b'_' || char.ascii_character == b'|' {
                    self.buffer.chars[row][col].write_char(ScreenChar {
                        ascii_character: b' ',
                        color_code: self.color_code,
                    });
                }
            }
        }

        // Restore cursor state
        self.cursor_visible = current_visible;
        self.update_cursor();
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write_char(blank);
        }
    }

}

// Write trait implementation
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// WRITER initialization
// In src/vga_buffer.rs
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        use x86_64::instructions::interrupts;

        // Create writer with safe defaults
        let mut writer = Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::LightGray, Color::Black), // More visible initial color
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            prompt_length: 2,
            is_wrapped: false,
            protected_region: ProtectedRegion::new(0, 0, 2),
                previous_cursor_pos: (0, 0),
                previous_char_color: ColorCode::new(Color::White, Color::Black),
                cursor_visible: true,
                cursor_blink_counter: 0,
                cursor_style: CursorStyle::Underscore,
                cursor_color: NORMAL_CURSOR,
                cursor_mode: CursorMode::Hardware,
                hardware_cursor_enabled: false,
        };

        // Force a clean initialization of the screen
        interrupts::without_interrupts(|| {
            // Write a test character to verify buffer is working
            writer.write_byte(b'X');

            // Clear the screen
            for row in 0..BUFFER_HEIGHT {
                writer.clear_row(row);
            }

            // Reset position
            writer.column_position = 0;
            writer.row_position = 0;
        });

        Mutex::new(writer)
    };
}

// Public function to switch cursor modes
pub fn switch_cursor_mode(mode: CursorMode) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().switch_cursor_mode(mode);
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

// Public function to allow setting cursor style from outside
pub fn set_cursor_style(style: CursorStyle) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().set_cursor_style(style);
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
