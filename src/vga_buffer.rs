use volatile::Volatile;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;
use core::ops::Deref;

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
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
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
    prompt_active: bool,
    input_mode: bool,
}

impl Writer {
    pub fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col] = Volatile::new(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;
        self.update_cursor();
    }

    pub fn write_prompt(&mut self) {
        self.write_byte(b'>');
        self.write_byte(b' ');
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn backspace(&mut self) {
        if !self.input_mode || (self.column_position <= 2 && self.row_position >= 0) {
            return;
        }

        if self.column_position > 0 {
            self.column_position -= 1;
        } else if self.row_position > 0 {
            self.row_position -= 1;
            self.column_position = BUFFER_WIDTH - 1;
        }

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        self.buffer.chars[self.row_position][self.column_position] = Volatile::new(blank);
        self.update_cursor();
    }

    pub fn set_input_mode(&mut self, active: bool) {
        self.input_mode = active;
        if active {
            self.write_prompt();
        }
    }
}

// Public interface functions
pub fn write_string(s: &str) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_string(s);
    });
}

pub fn backspace() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().backspace();
    });
}

pub fn set_input_mode(active: bool) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().set_input_mode(active);
    });
}

// Update the keyboard handler in interrupts.rs
pub fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::DecodedKey;

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            crate::vga_buffer::set_input_mode(true);
            match key {
                DecodedKey::Unicode(character) => {
                    match character {
                        '\n' => {
                            println!();
                        },
                        '\u{8}' => { // Backspace
                            crate::vga_buffer::backspace();
                        },
                        _ => {
                            print!("{}", character);
                        }
                    }
                },
                DecodedKey::RawKey(_key) => {
                    // Handle raw keys if needed
                }
            }
        }
    }

    unsafe {
        PICS.lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
