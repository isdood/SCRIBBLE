use volatile::Volatile;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

// ... (Color enum and ColorCode implementations remain the same)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        ScreenChar {
            ascii_character,
            color_code,
        }
    }
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
    pub fn get_row_position(&self) -> usize {
        self.row_position
    }

    pub fn set_prompt_row(&mut self, row: usize) {
        if row < BUFFER_HEIGHT {
            self.row_position = row;
            self.column_position = 0;
            self.update_cursor();
        }
    }

    pub fn backspace(&mut self) {
        if self.column_position <= 2 {
            return;
        }

        if self.column_position > 0 {
            self.column_position -= 1;
            let blank = ScreenChar::new(b' ', self.color_code);
            unsafe {
                (*(&mut self.buffer.chars[self.row_position][self.column_position]
                as *mut Volatile<ScreenChar>)) = Volatile::new(blank);
            }
            self.update_cursor();
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

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                unsafe {
                    let screen_char = ScreenChar::new(byte, self.color_code);
                    (*(&mut self.buffer.chars[row][col] as *mut Volatile<ScreenChar>)) =
                    Volatile::new(screen_char);
                }

                self.column_position += 1;
                self.update_cursor();
            }
        }
    }

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    unsafe {
                        // Use raw pointer operations instead of read/write methods
                        let character = (*(&self.buffer.chars[row][col] as *const Volatile<ScreenChar>)).read();
                        (*(&mut self.buffer.chars[row - 1][col] as *mut Volatile<ScreenChar>)) =
                        Volatile::new(character);
                    }
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;
        self.update_cursor();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for col in 0..BUFFER_WIDTH {
            unsafe {
                (*(&mut self.buffer.chars[row][col] as *mut Volatile<ScreenChar>)) =
                Volatile::new(blank);
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
}

// Single implementation of Write trait
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
        writer.update_cursor();
    });
}
