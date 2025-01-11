// src/serial.rs
use core::fmt::Write;
use uart_16550::SerialPort;
use spin::Mutex;

pub struct SerialController(SerialPort);

impl SerialController {
    pub fn new(port: u16) -> Self {
        let mut serial_port = unsafe { SerialPort::new(port) };
        serial_port.init();
        SerialController(serial_port)
    }

    pub fn send(&mut self, byte: u8) {
        self.0.send(byte);
    }
}

impl Write for SerialController {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

lazy_static::lazy_static! {
    pub static ref SERIAL1: Mutex<SerialController> = {
        Mutex::new(SerialController::new(0x3F8))
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
    });
}
