// lib/unstable_matter/src/devices/keyboard.rs
/// Keyboard Device Implementation
/// Last Updated: 2025-01-12 20:55:14 UTC
/// Author: Caleb J.D. Terkovics (isdood)

use crate::arch::x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy)]
pub enum DecodedKey {
    Unicode(char),
    RawKey(KeyCode),
}

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Special keys
    Escape,
    PrintScreen,
    ScrollLock,
    Pause,
    // Navigation
    Insert, Home, PageUp, Delete, End, PageDown,
    // Cursor
    ArrowRight, ArrowLeft, ArrowDown, ArrowUp,
    // Numeric pad
    NumpadDivide, NumpadMultiply, NumpadMinus, NumpadPlus, NumpadEnter,
    Numpad7, Numpad8, Numpad9,
    Numpad4, Numpad5, Numpad6,
    Numpad1, Numpad2, Numpad3,
    Numpad0, NumpadDot,
    // Other
    Backspace, Tab, Enter,
    LShift, RShift,
    LCtrl, RCtrl,
    LAlt, RAlt,
    Apps,
}

pub struct KeyboardLayout {
    normal_map: &'static [DecodedKey],
    shift_map: &'static [DecodedKey],
}

pub struct KeyboardDriver {
    port: Port<u8>,
    layout: KeyboardLayout,
    shift_pressed: bool,
    alt_pressed: bool,
    ctrl_pressed: bool,
}

impl KeyboardDriver {
    pub const fn new(layout: KeyboardLayout) -> Self {
        Self {
            port: Port::new(0x60),
            layout,
            shift_pressed: false,
            alt_pressed: false,
            ctrl_pressed: false,
        }
    }

    pub unsafe fn read_scancode(&mut self) -> Option<u8> {
        // Check if there's data to read from keyboard
        let status = Port::<u8>::new(0x64).read();
        if status & 1 == 0 {
            return None;
        }
        Some(self.port.read())
    }

    pub fn process_scancode(&mut self, scancode: u8) -> Option<DecodedKey> {
        let release = scancode & 0x80 != 0;
        let code = scancode & !0x80;

        if release {
            self.handle_key_release(code)
        } else {
            self.handle_key_press(code)
        }
    }

    fn handle_key_press(&mut self, code: u8) -> Option<DecodedKey> {
        match code {
            0x2A | 0x36 => { // Left or Right Shift
                self.shift_pressed = true;
                None
            }
            0x1D => { // Left Ctrl
                self.ctrl_pressed = true;
                None
            }
            0x38 => { // Left Alt
                self.alt_pressed = true;
                None
            }
            _ => {
                if code as usize >= self.layout.normal_map.len() {
                    return None;
                }
                Some(if self.shift_pressed {
                    self.layout.shift_map[code as usize]
                } else {
                    self.layout.normal_map[code as usize]
                })
            }
        }
    }

    fn handle_key_release(&mut self, code: u8) -> Option<DecodedKey> {
        match code {
            0x2A | 0x36 => { // Left or Right Shift
                self.shift_pressed = false;
                None
            }
            0x1D => { // Left Ctrl
                self.ctrl_pressed = false;
                None
            }
            0x38 => { // Left Alt
                self.alt_pressed = false;
                None
            }
            _ => None
        }
    }
}

// US 104-key layout implementation
pub const US_104_KEY: KeyboardLayout = KeyboardLayout {
    normal_map: &[
        // 0x00
        DecodedKey::RawKey(KeyCode::F1),
        DecodedKey::Unicode('1'),
        DecodedKey::Unicode('2'),
        DecodedKey::Unicode('3'),
        // ... more mappings
    ],
    shift_map: &[
        // 0x00
        DecodedKey::RawKey(KeyCode::F1),
        DecodedKey::Unicode('!'),
        DecodedKey::Unicode('@'),
        DecodedKey::Unicode('#'),
        // ... more mappings
    ],
};
