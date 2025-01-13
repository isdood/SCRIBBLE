// src/vga_buffer.rs
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use unstable_matter::{
    SpaceTime, Vector3D, VectorSpace, MeshCell,
    ufo::{UFO, Protected},
};
use x86_64::instructions::port::Port;

// Color enums remain the same...
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

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_ADDR: usize = 0xb8000;

/// VGA buffer using UnstableMatter's vector space
struct VGABuffer {
    space: SpaceTime<ScreenChar>,
    ufo: UFO<ScreenChar>,
}

impl VGABuffer {
    fn new() -> Self {
        Self {
            space: SpaceTime::new(VGA_BUFFER_ADDR, BUFFER_WIDTH * BUFFER_HEIGHT, 0),
            ufo: UFO::new(),
        }
    }

    unsafe fn write_char(&mut self, pos: Vector3D, char: ScreenChar) {
        let idx = self.pos_to_index(pos);
        self.space.write_at(idx, char);
    }

    unsafe fn read_char(&self, pos: Vector3D) -> ScreenChar {
        let idx = self.pos_to_index(pos);
        self.space.read_at(idx)
    }

    fn pos_to_index(&self, pos: Vector3D) -> usize {
        (pos.y() * BUFFER_WIDTH + pos.x()) as usize
    }
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: VGABuffer,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: VGABuffer::new(),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let pos = Vector3D::new(
                    self.column_position as isize,
                    (BUFFER_HEIGHT - 1) as isize,
                                        0,
                );

                let screen_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                unsafe {
                    self.buffer.write_char(pos, screen_char);
                }

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let src_pos = Vector3D::new(col as isize, row as isize, 0);
                let dst_pos = Vector3D::new(col as isize, (row - 1) as isize, 0);

                unsafe {
                    let character = self.buffer.read_char(src_pos);
                    self.buffer.write_char(dst_pos, character);
                }
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
            let pos = Vector3D::new(col as isize, row as isize, 0);
            unsafe {
                self.buffer.write_char(pos, blank);
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

    pub fn debug_print(&mut self, msg: &str) {
        let color_backup = self.color_code;
        self.color_code = ColorCode::new(Color::Yellow, Color::Black);
        self.write_string("[DEBUG] ");
        self.write_string(msg);
        self.write_byte(b'\n');
        self.color_code = color_backup;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let mut writer = Writer::new();

        // Validate VGA memory
        let test_char = ScreenChar {
            ascii_character: b'T',
            color_code: ColorCode::new(Color::White, Color::Black),
        };

        let pos = Vector3D::new(0, 0, 0);
        unsafe {
            writer.buffer.write_char(pos, test_char);
            let read_char = writer.buffer.read_char(pos);

            if read_char.ascii_character != b'T' {
                panic!("VGA buffer not accessible");
            }
        }

        Mutex::new(writer)
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        use core::fmt::Write;
        WRITER.lock().write_fmt(args).unwrap();
    });
}
