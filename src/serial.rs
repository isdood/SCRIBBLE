use x86_64::instructions::port::Port;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERIAL_PORT: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3F8));
}

pub struct SerialPort {
    data: Port<u8>,
    int_en: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_sts: Port<u8>,
}

impl SerialPort {
    pub fn new(base: u16) -> SerialPort {
        let mut port = SerialPort {
            data: Port::new(base),
            int_en: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_sts: Port::new(base + 5),
        };
        unsafe {
            // Disable interrupts
            port.int_en.write(0x00);

            // Enable DLAB
            port.line_ctrl.write(0x80);

            // Set divisor (115200 baud)
            port.data.write(0x01);
            port.int_en.write(0x00);

            // 8 bits, no parity, one stop bit
            port.line_ctrl.write(0x03);

            // Enable FIFO, clear them, with 14-byte threshold
            port.fifo_ctrl.write(0xC7);

            // Mark data terminal ready, request to send
            port.modem_ctrl.write(0x0B);
        }
        port
    }

    fn line_sts(&mut self) -> u8 {
        unsafe { self.line_sts.read() }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            while (self.line_sts() & 0x20) == 0 {}
            self.data.write(byte);
        }
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_PORT.lock().write_fmt(args).expect("Printing to serial failed");
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
