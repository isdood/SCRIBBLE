use volatile::Volatile;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

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
    buffer: &'static mut Buffer,
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

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                // Use unsafe to directly write to volatile memory
                unsafe {
                    (&mut self.buffer.chars[row][col] as *mut Volatile<ScreenChar>)
                    .write(Volatile::new(colored_char));
                }

                self.column_position += 1;
                self.update_cursor();
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
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
                    let character = unsafe {
                        (&self.buffer.chars[row][col] as *const Volatile<ScreenChar>)
                        .read()
                        .read()
                    };
                    unsafe {
                        (&mut self.buffer.chars[row - 1][col] as *mut Volatile<ScreenChar>)
                        .write(Volatile::new(character));
                    }
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
            unsafe {
                (&mut self.buffer.chars[row][col] as *mut Volatile<ScreenChar>)
                .write(Volatile::new(blank));
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.column_position <= 2 {
            return;
        }

        if self.column_position > 0 {
            self.column_position -= 1;
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            };
            unsafe {
                (&mut self.buffer.chars[self.row_position][self.column_position] as *mut Volatile<ScreenChar>)
                .write(Volatile::new(blank));
            }
            self.update_cursor();
        }
    }

    fn scroll_up(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.row_position = BUFFER_HEIGHT - 1;
    }

    fn update_cursor(&mut self) {
        let pos = self.row_position * BUFFER_WIDTH + self.column_position;
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port = Port::new(0x3D4);
            let mut data_port = Port::new(0x3D5);

            port.write(0x0Au8);
            data_port.write(0x00u8);  // Start scanline
            port.write(0x0Bu8);
            data_port.write(15u8);    // End scanline (full block cursor)

            port.write(0x0Fu8);
            data_port.write((pos & 0xFF) as u8);
            port.write(0x0Eu8);
            data_port.write(((pos >> 8) & 0xFF) as u8);
        }
    }

    pub fn get_row_position(&self) -> usize {
        self.row_position
    }

    pub fn set_prompt_row(&mut self, row: usize) {
        self.prompt_row = row;
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
        color_code: ColorCode::new(Color::White, Color::Black),
                                                      buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
                                                      prompt_row: 0,
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

pub fn print(args: fmt::Arguments) {
    _print(args);
}

pub fn enable_cursor() {
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::new(0x3D4);
        let mut data_port = Port::new(0x3D5);

        port.write(0x0Au8);
        data_port.write(0x00u8);
        port.write(0x0Bu8);
        data_port.write(15u8);
    }
}

pub fn disable_cursor() {
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::new(0x3D4);
        let mut data_port = Port::new(0x3D5);

        port.write(0x0Au8);
        data_port.write(0x20u8);
    }
}

pub fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 0..BUFFER_HEIGHT {
        writer.clear_row(row);
    }
    writer.column_position = 0;
    writer.row_position = 0;
    writer.prompt_row = 0;
    writer.update_cursor();
}

pub fn backspace() {
    WRITER.lock().backspace();
}

impl ScreenChar {
    fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        ScreenChar {
            ascii_character,
            color_code,
        }
    }
}
