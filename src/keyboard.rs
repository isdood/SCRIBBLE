use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;  // Add this import
use crate::vga_buffer::WRITER;  // Add this import
use crate::println;
use crate::vga_buffer::{Color, set_color};

const QUEUE_SIZE: usize = 100;

struct KeyboardBuffer {
    buffer: [u8; QUEUE_SIZE],
    write_pos: usize,
    read_pos: usize,
}

impl KeyboardBuffer {
    const fn new() -> Self {
        Self {
            buffer: [0; QUEUE_SIZE],
            write_pos: 0,
            read_pos: 0,
        }
    }

    fn push(&mut self, scancode: u8) -> bool {
        if (self.write_pos + 1) % QUEUE_SIZE == self.read_pos {
            return false;
        }
        self.buffer[self.write_pos] = scancode;
        self.write_pos = (self.write_pos + 1) % QUEUE_SIZE;
        true
    }

    fn pop(&mut self) -> Option<u8> {
        if self.read_pos == self.write_pos {
            return None;
        }
        let scancode = self.buffer[self.read_pos];
        self.read_pos = (self.read_pos + 1) % QUEUE_SIZE;
        Some(scancode)
    }
}

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );

    static ref SCANCODE_QUEUE: Mutex<KeyboardBuffer> = Mutex::new(KeyboardBuffer::new());
}

pub fn add_scancode(scancode: u8) {
    if !SCANCODE_QUEUE.lock().push(scancode) {
        println!("WARNING: scancode queue full; dropping keyboard input");
    }
    process_keyboard();
}

fn process_keyboard() {
    while let Some(scancode) = SCANCODE_QUEUE.lock().pop() {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                drop(keyboard);
                handle_keyevent(key);
            }
        }
    }
}

// Key Handler
fn handle_keyevent(key: DecodedKey) {
    interrupts::without_interrupts(|| {
        match key {
            DecodedKey::Unicode(character) => {
                set_color(Color::White, Color::Black);
                print!("{}", character);
            },
            DecodedKey::RawKey(_key) => {
                set_color(Color::LightCyan, Color::Black);
                print!("#");
            },
        }
    });
}

// Add the init function
pub fn initialize() {
    println!("Initializing keyboard...");
    // No need for explicit port initialization in QEMU
    println!("Keyboard initialization complete");
}
