//! Terminal implementation for Garnet

use crate::{Error, GarnetConfig, Result};

/// Terminal state structure
pub struct Terminal {
    config: GarnetConfig,
    buffer: Vec<u8>,
    cursor_x: u16,
    cursor_y: u16,
    foreground: u8,
        background: u8,
}

impl Terminal {
    /// Create a new terminal instance with the given configuration
    pub fn new(config: GarnetConfig) -> Result<Self> {
        if config.width == 0 || config.height == 0 {
            return Err(Error::InvalidDimensions);
        }

        Ok(Self {
            buffer: vec![0; config.width as usize * config.height as usize * 2],
            cursor_x: 0,
            cursor_y: 0,
            foreground: 0x0F, // White
                background: 0x00, // Black
                config,
        })
    }

    /// Write a character to the terminal at the current cursor position
    pub fn write_char(&mut self, c: char) -> Result<()> {
        match c {
            '\n' => self.new_line(),
            '\r' => self.carriage_return(),
            '\t' => self.tab(),
            _ => self.put_char(c),
        }
    }

    /// Clear the terminal screen
    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    // Private helper methods
    fn new_line(&mut self) {
        self.cursor_y = (self.cursor_y + 1) % self.config.height;
        if self.cursor_y == 0 {
            self.scroll_up();
        }
    }

    fn carriage_return(&mut self) {
        self.cursor_x = 0;
    }

    fn tab(&mut self) {
        self.cursor_x = (self.cursor_x + 8) & !7;
        if self.cursor_x >= self.config.width {
            self.new_line();
        }
    }

    fn put_char(&mut self, c: char) -> Result<()> {
        if self.cursor_x >= self.config.width {
            self.new_line();
        }

        let idx = (self.cursor_y as usize * self.config.width as usize + self.cursor_x as usize) * 2;
        self.buffer[idx] = c as u8;
        self.buffer[idx + 1] = (self.background << 4) | self.foreground;

        self.cursor_x += 1;
        Ok(())
    }

    fn scroll_up(&mut self) {
        let line_size = self.config.width as usize * 2;
        let buffer_size = self.buffer.len();
        self.buffer.copy_within(line_size..buffer_size, 0);
        let start = buffer_size - line_size;
        self.buffer[start..].fill(0);
    }
}
