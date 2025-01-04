use x86_64::instructions::port::Port;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::vga_buffer::{Color, WRITER};
use crate::println;
use crate::serial_println;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub fn add_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            drop(keyboard);

            x86_64::instructions::interrupts::without_interrupts(|| {
                let mut writer = WRITER.lock();

                // Set yellow color for keyboard input
                writer.change_color(Color::Yellow, Color::Black);

                match key {
                    DecodedKey::Unicode(character) => {
                        writer.write_byte(character as u8);
                        serial_println!("Wrote character '{}' in Yellow", character);
                    },
                    DecodedKey::RawKey(_key) => {
                        writer.write_byte(b'#');
                        serial_println!("Wrote '#' in Yellow");
                    },
                }
            });
        }
    }
}

pub fn init() {
    println!("Initializing keyboard...");

    // Test color changes
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        writer.change_color(Color::Red, Color::Black);
        for byte in b"Red Text Test\n".iter() {
            writer.write_byte(*byte);
        }

        writer.change_color(Color::Green, Color::Black);
        for byte in b"Green Text Test\n".iter() {
            writer.write_byte(*byte);
        }

        writer.change_color(Color::Blue, Color::Black);
        for byte in b"Blue Text Test\n".iter() {
            writer.write_byte(*byte);
        }

        // Reset to white for system text
        writer.change_color(Color::White, Color::Black);
    });

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

        // Configure controller
        cmd_port.write(0x20);
        let mut config = data_port.read();
        config |= 1 << 0;    // Enable first PS/2 port interrupt
        config &= !(1 << 1); // Disable second PS/2 port
        cmd_port.write(0x60);
        data_port.write(config);

        // Enable devices
        cmd_port.write(0xAE);  // Enable first PS/2 port
        data_port.write(0xFF); // Reset keyboard
        data_port.write(0xF4); // Enable scanning

        println!("Keyboard initialization complete");
    }
}
