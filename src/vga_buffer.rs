// Keep all the same struct definitions, but modify the Writer implementation:

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

                // Determine color based on context
                let current_color = if self.prompt_active && col <= 1 {
                    ColorCode::new(Color::Green, Color::Black)  // Green prompt
                } else {
                    self.color_code  // Normal color (white)
                };

                let colored_char = ScreenChar {
                    ascii_character: byte,
                    color_code: current_color,
                };

                // Direct assignment instead of volatile operations
                self.buffer.chars[row][col] = Volatile::new(colored_char);

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
                    let character = self.buffer.chars[row][col];
                    self.buffer.chars[row - 1][col] = character;
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
            self.buffer.chars[row][col] = Volatile::new(blank);
        }
    }

    pub fn backspace(&mut self) {
        // Check if we're at the prompt position
        if self.column_position <= 2 &&
            self.buffer.chars[self.row_position][0].read().ascii_character == b'>' {
                return;
            }

            if self.column_position > 0 {
                self.column_position -= 1;
                let blank = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
                self.buffer.chars[self.row_position][self.column_position] = Volatile::new(blank);
                self.update_cursor();
            }
    }
}
