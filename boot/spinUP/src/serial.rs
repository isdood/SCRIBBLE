// boot/spinUP/src/serial.rs
// Last Updated: 2025-01-13 05:37:02 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

use core::fmt;

const SERIAL_PORT: u16 = 0x3F8;

pub fn init_serial() {
    // Disable interrupts
    outb(SERIAL_PORT + 1, 0x00);

    // Enable DLAB
    outb(SERIAL_PORT + 3, 0x80);

    // Set divisor (115200 baud)
    outb(SERIAL_PORT + 0, 0x01);
    outb(SERIAL_PORT + 1, 0x00);

    // 8 bits, no parity, one stop bit
    outb(SERIAL_PORT + 3, 0x03);

    // Enable FIFO, clear with 14-byte threshold
    outb(SERIAL_PORT + 2, 0xC7);

    // IRQs enabled, RTS/DSR set
    outb(SERIAL_PORT + 4, 0x0B);
}

#[inline]
fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
                         in("al") value,
                         options(nomem, nostack, preserves_flags)
        );
    }
}

fn serial_write_byte(byte: u8) {
    while (inb(SERIAL_PORT + 5) & 0x20) == 0 {}
    outb(SERIAL_PORT, byte);
}

#[inline]
fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            out("al") value,
                         in("dx") port,
                         options(nomem, nostack, preserves_flags)
        );
    }
    value
}

pub fn serial_write_str(s: &str) {
    for byte in s.bytes() {
        serial_write_byte(byte);
    }
}

pub struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        serial_write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::serial::SerialWriter {}, $($arg)*);
    });
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
