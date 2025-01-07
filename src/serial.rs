//  IMPORTS  \\
///////////////////////////////
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt::Write;
use x86_64::instructions::interrupts;
use crate::splat::{self, SplatLevel};
//////////// END //////////////

// Serial Port Constants
const SERIAL_PORT_ADDRESS: u16 = 0x3F8;  // COM1
const SERIAL_TIMEOUT: u16 = 1000;

#[derive(Debug)]
pub enum SerialError {
    InitFailed,
    WriteTimeout,
    PortLocked,
}

pub struct SerialStats {
    bytes_written: usize,
    write_failures: usize,
    last_status: u8,
}

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialController> = {
        match SerialController::new(SERIAL_PORT_ADDRESS) {
            Ok(controller) => {
                splat::log(SplatLevel::BitsNBytes, "Serial port initialized successfully");
                Mutex::new(controller)
            }
            Err(e) => {
                panic!("Failed to initialize serial port: {:?}", e);
            }
        }
    };
}

pub struct SerialController {
    port: SerialPort,
    stats: SerialStats,
}

impl SerialController {

    pub fn is_transmit_empty(&self) -> bool {
        unsafe {
            let mut port = Port::<u8>::new(0x3F8 + 5);
            (port.read() & 0x20) != 0
        }
    }

    /// Creates a new SerialController with the specified port address
    pub fn new(address: u16) -> Result<Self, SerialError> {
        let mut port = unsafe { SerialPort::new(address) };

        // Initialize the port with error checking
        match Self::init_port(&mut port) {
            Ok(_) => Ok(SerialController {
                port,
                stats: SerialStats {
                    bytes_written: 0,
                    write_failures: 0,
                    last_status: 0,
                }
            }),
            Err(e) => {
                splat::log(
                    SplatLevel::Critical,
                    &alloc::format!("Serial port initialization failed: {:?}", e)
                );
                Err(e)
            }
        }
    }

    fn init_port(port: &mut SerialPort) -> Result<(), SerialError> {
        port.init();

        // Verify initialization
        if !Self::is_port_ready(port) {
            return Err(SerialError::InitFailed);
        }

        Ok(())
    }

    fn is_port_ready(&self) -> bool {
        unsafe {
            let mut status_port = Port::new(0x3FD);
            status_port.read() & 0x20 != 0
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), SerialError> {
        let mut timeout = SERIAL_TIMEOUT;

        for byte in bytes {
            while timeout > 0 && !self.port.is_transmit_empty() {
                timeout -= 1;
            }

            if timeout == 0 {
                self.stats.write_failures += 1;
                return Err(SerialError::WriteTimeout);
            }

            unsafe { self.port.send(*byte); }
            self.stats.bytes_written += 1;
        }

        Ok(())
    }

    pub fn get_stats(&self) -> &SerialStats {
        &self.stats
    }
}

impl Write for SerialController {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_bytes(s.as_bytes())
        .map_err(|_| core::fmt::Error)
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    interrupts::without_interrupts(|| {
        if let Some(mut serial) = SERIAL1.try_lock() {
            if let Err(e) = serial.write_fmt(args) {
                serial.stats.write_failures += 1;
                splat::log(
                    SplatLevel::Warning,
                    &alloc::format!("Serial write failed: {:?}", e)
                );
            }
        } else {
            // Log serial port lock failure using direct VGA buffer if available
            if let Some(vga) = crate::vga_buffer::WRITER.try_lock() {
                let _ = vga.write_str("[SERIAL LOCKED]");
            }
        }
    });
}

/// Logs the current serial port statistics
pub fn log_serial_stats() {
    if let Some(serial) = SERIAL1.try_lock() {
        let stats = serial.get_stats();
        splat::log(
            SplatLevel::BitsNBytes,
            &alloc::format!(
                "Serial Port Statistics:\n\
└─ Bytes Written: {}\n\
└─ Write Failures: {}\n\
└─ Success Rate: {:.2}%",
stats.bytes_written,
stats.write_failures,
if stats.bytes_written > 0 {
    100.0 * (1.0 - (stats.write_failures as f64 / stats.bytes_written as f64))
} else {
    100.0
}
            )
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_write() {
        if let Ok(mut controller) = SerialController::new(0x3F8) {
            assert!(controller.write_bytes(b"test").is_ok());
        }
    }
}
