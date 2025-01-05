// ... previous code remains the same until clear_row ...

fn clear_row(&mut self, row: usize) {
    let blank = ScreenChar {
        ascii_character: b' ',
        color_code: self.color_code,
    };
    for col in 0..BUFFER_WIDTH {
        unsafe {
            self.buffer.chars[row][col] = Volatile::new(blank);
        }
    }
}

pub fn backspace(&mut self) {
    // Check if we're at the prompt position
    if self.column_position <= 2 && unsafe {
        self.buffer.chars[self.row_position][0].read().ascii_character == b'>'
    } {
        return;
    }

    if self.column_position > 0 {
        self.column_position -= 1;
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        unsafe {
            self.buffer.chars[self.row_position][self.column_position] = Volatile::new(blank);
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

pub fn set_prompt_active(&mut self, active: bool) {
    self.prompt_active = active;
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
                                                      prompt_active: false,
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        // Set prompt_active based on first character being '>'
        let is_prompt = format_args!("{}", args).to_string().starts_with('>');
        writer.prompt_active = is_prompt;
        writer.write_fmt(args).unwrap();
        writer.prompt_active = false;
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
        writer.update_cursor();
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
    clear_screen();
    enable_cursor();
    crate::print!("> ");  // Initial prompt
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
