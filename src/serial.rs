//  IMPORTS  \\
///////////////////////////////
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt::Write;
use x86_64::instructions::{interrupts, port::Port};
use crate::splat::{self, SplatLevel};
use alloc::format;
//////////// END //////////////

// Serial Port Constants
const SERIAL_PORT_ADDRESS: u16 = 0x3F8;  // COM1
const SERIAL_TIMEOUT: u16 = 1000;

// Port offset constants
const LINE_STATUS_REG: u16 = 5;
const TRANSMIT_EMPTY_BIT: u8 = 0x20;

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
        let timestamp = "2025-01-07 06:42:01";
        match SerialController::new(SERIAL_PORT_ADDRESS) {
            Ok(controller) => {
                splat::log(
                    SplatLevel::BitsNBytes,
                    &format!(
                        "Serial port initialized successfully\n\
└─ Time: {}\n\
└─ Port: COM1\n\
└─ Address: {:#x}",
timestamp,
SERIAL_PORT_ADDRESS
                    )
                );
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
                    &format!("Serial port initialization failed: {:?}", e)
                );
                Err(e)
            }
        }
    }

    fn init_port(port: &mut SerialPort) -> Result<(), SerialError> {
        port.init();

        // Verify initialization by checking transmit empty bit
        unsafe {
            let mut status_port = Port::new(SERIAL_PORT_ADDRESS + LINE_STATUS_REG);
            if (status_port.read() & TRANSMIT_EMPTY_BIT) == 0 {
                return Err(SerialError::InitFailed);
            }
        }

        Ok(())
    }

    fn is_transmit_empty(&self) -> bool {
        unsafe {
            let mut status_port = Port::<u8>::new(SERIAL_PORT_ADDRESS + LINE_STATUS_REG);
            (status_port.read() & TRANSMIT_EMPTY_BIT) != 0
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), SerialError> {
        let mut timeout = SERIAL_TIMEOUT;

        for byte in bytes {
            while timeout > 0 && !self.is_transmit_empty() {
                timeout -= 1;
            }

            if timeout == 0 {
                self.stats.write_failures += 1;
                return Err(SerialError::WriteTimeout);
            }

            unsafe {
                self.port.send(*byte);
            }
            self.stats.bytes_written += 1;
        }

        Ok(())
    }

    pub fn get_stats(&self) -> &SerialStats {
        &self.stats
    }
}

// Rest of the implementation remains the same...
