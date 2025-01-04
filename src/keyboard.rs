// src/keyboard.rs
use x86_64::instructions::port::Port;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::vga_buffer::{Color, WRITER};
use crate::println;

const QUEUE_SIZE: usize = 100;

struct KeyboardBuffer {
    buffer: [u8; QUEUE_SIZE],
    write_pos: AtomicUsize,
    read_pos: AtomicUsize,
}

impl KeyboardBuffer {
    const fn new() -> Self {
        Self {
            buffer: [0; QUEUE_SIZE],
            write_pos: AtomicUsize::new(0),
            read_pos: AtomicUsize::new(0),
        }
    }

    fn push(&self, scancode: u8) -> bool {
        let write_pos = self.write_pos.load(Ordering::Relaxed);
        let next_write = (write_pos + 1) % QUEUE_SIZE;

        if next_write == self.read_pos.load(Ordering::Relaxed) {
            return false;
        }

        unsafe {
            let buffer_ptr = self.buffer.as_ptr() as *mut u8;
            buffer_ptr.add(write_pos).write(scancode);
        }

        self.write_pos.store(next_write, Ordering::Release);
        true
    }

    fn pop(&self) -> Option<u8> {
        let read_pos = self.read_pos.load(Ordering::Relaxed);
        if read_pos == self.write_pos.load(Ordering::Acquire) {
            return None;
        }

        let scancode = unsafe {
            let buffer_ptr = self.buffer.as_ptr();
            *buffer_ptr.add(read_pos)
        };

        self.read_pos.store((read_pos + 1) % QUEUE_SIZE, Ordering::Release);
        Some(scancode)
    }
}

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );

    static ref SCANCODE_QUEUE: KeyboardBuffer = KeyboardBuffer::new();
}

pub fn add_scancode(scancode: u8) {
    if !SCANCODE_QUEUE.push(scancode) {
        println!("WARNING: scancode queue full; dropping keyboard input");
    }
    process_keyboard();
}

fn process_keyboard() {
    while let Some(scancode) = SCANCODE_QUEUE.pop() {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                // Drop the keyboard lock before acquiring the writer lock
                drop(keyboard);

                // Use interrupts::without_interrupts to ensure atomic access
                x86_64::instructions::interrupts::without_interrupts(|| {
                    let mut writer = WRITER.lock();

                    // Save the current color
                    let current_color = writer.color_code;

                    // Set to magenta (or any bright color) and ensure it's set
                    writer.change_color(Color::Magenta, Color::Black);

                    match key {
                        DecodedKey::Unicode(character) => {
                            // Write directly using write_byte to maintain color
                            writer.write_byte(character as u8);
                        },
                        DecodedKey::RawKey(_key) => {
                            writer.write_byte(b'#');
                        },
                    }

                    // Add a space after each character
                    writer.write_byte(b' ');

                    // Don't restore the previous color - keep the new one
                });
            }
        }
    }
}

pub fn init() {
    println!("Initializing keyboard...");
        test_color_output();
    unsafe {
        let mut cmd_port: Port<u8> = Port::new(0x64);
        let mut data_port: Port<u8> = Port::new(0x60);

        // Disable PS/2 ports
        cmd_port.write(0xAD);
        cmd_port.write(0xA7);

        // Flush output buffer
        while (cmd_port.read() & 1) == 1 {
            data_port.read();
        }

        // Set configuration byte
        cmd_port.write(0x20);
        let mut config = data_port.read();
        config |= 1 << 0;      // Enable first PS/2 port interrupt
        config &= !(1 << 1);   // Disable second PS/2 port
        cmd_port.write(0x60);
        data_port.write(config);

        // Enable devices
        cmd_port.write(0xAE);  // Enable first PS/2 port
        data_port.write(0xFF); // Reset keyboard
        data_port.write(0xF4); // Enable scanning

        println!("Keyboard initialization complete");
    }
}

// Add this to keyboard.rs
pub fn test_color_output() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        // Test with different colors
        writer.change_color(Color::Red, Color::Black);
        writer.write_str("Red Text\n").unwrap();

        writer.change_color(Color::Green, Color::Black);
        writer.write_str("Green Text\n").unwrap();

        writer.change_color(Color::Blue, Color::Black);
        writer.write_str("Blue Text\n").unwrap();

        // Reset to default
        writer.change_color(Color::White, Color::Black);
    });
}
