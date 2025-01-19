//! Input handling for Garnet

use crate::{Error, Result};

/// Input handler for terminal
pub struct InputHandler {
    modifier_state: ModifierState,
    buffer: Vec<u8>,
}

#[derive(Default)]
struct ModifierState {
    shift: bool,
    ctrl: bool,
    alt: bool,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            modifier_state: ModifierState::default(),
            buffer: Vec::with_capacity(16),
        }
    }

    /// Process a keyboard scan code
    pub fn process_scancode(&mut self, scancode: u8) -> Result<Option<char>> {
        // Implementation based on Scribble's keyboard driver
        match scancode {
            0x2A | 0x36 => {
                self.modifier_state.shift = true;
                Ok(None)
            },
            0xAA | 0xB6 => {
                self.modifier_state.shift = false;
                Ok(None)
            },
            0x1D => {
                self.modifier_state.ctrl = true;
                Ok(None)
            },
            0x9D => {
                self.modifier_state.ctrl = false;
                Ok(None)
            },
            0x38 => {
                self.modifier_state.alt = true;
                Ok(None)
            },
            0xB8 => {
                self.modifier_state.alt = false;
                Ok(None)
            },
            _ => self.translate_scancode(scancode),
        }
    }

    fn translate_scancode(&self, scancode: u8) -> Result<Option<char>> {
        // Basic scancode to ASCII translation
        // This should be expanded based on Scribble's keyboard mapping
        let c = match scancode {
            0x1E..=0x26 => {
                let base = if self.modifier_state.shift { b'A' } else { b'a' };
                Some((base + (scancode - 0x1E)) as char)
            },
            0x2C..=0x32 => {
                let base = if self.modifier_state.shift { b'Z' } else { b'z' };
                Some((base - (0x32 - scancode)) as char)
            },
            _ => None,
        };
        Ok(c)
    }
}
