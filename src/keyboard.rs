// In keyboard.rs
use crate::serial_println;

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
                drop(keyboard);

                x86_64::instructions::interrupts::without_interrupts(|| {
                    let mut writer = WRITER.lock();

                    // Debug current color
                    let current_color = writer.get_current_color();
                    serial_println!("Current color before input: {:?}", current_color);

                    // Set color to yellow for keyboard input
                    writer.change_color(Color::Yellow, Color::Black);
                    serial_println!("Setting color to Yellow for keyboard input");

                    match key {
                        DecodedKey::Unicode(character) => {
                            if character == '\n' {
                                writer.write_byte(b'\n');
                                // Reset color after newline
                                writer.change_color(Color::White, Color::Black);
                            } else {
                                writer.write_byte(character as u8);
                            }
                            serial_println!("Wrote character: {:?}", character);
                        },
                        DecodedKey::RawKey(_key) => {
                            writer.write_byte(b'#');
                            serial_println!("Wrote raw key marker '#'");
                        },
                    }
                });
            }
        }
    }
}

pub fn init() {
    println!("Initializing keyboard...");

    // Test color changes with debug output
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        // Start with white
        writer.change_color(Color::White, Color::Black);
        serial_println!("Initial color set to White");

        // Color test sequence
        for (color, text) in &[
            (Color::Red, "Red Test\n"),
                                                         (Color::Green, "Green Test\n"),
                                                         (Color::Blue, "Blue Test\n"),
                                                         (Color::White, "Back to White\n")
        ] {
            writer.change_color(*color, Color::Black);
            serial_println!("Testing color: {:?}", color);
            writer.write_str(text).unwrap();
        }
    });

    // Rest of keyboard initialization...
    unsafe {
        // ... (keep existing initialization code)
    }
}
