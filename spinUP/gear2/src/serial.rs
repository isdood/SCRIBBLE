pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub const unsafe fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn init(&mut self) {
        unsafe {
            // Disable interrupts
            outb(self.port + 1, 0x00);

            // Enable DLAB
            outb(self.port + 3, 0x80);

            // Set divisor (115200 baud)
            outb(self.port + 0, 0x01);
            outb(self.port + 1, 0x00);

            // 8 bits, no parity, one stop bit
            outb(self.port + 3, 0x03);

            // Enable FIFO, clear them, with 14-byte threshold
            outb(self.port + 2, 0xC7);

            // Mark data terminal ready, signal request to send
            outb(self.port + 4, 0x0B);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            while (inb(self.port + 5) & 0x20) == 0 {}
            outb(self.port, byte);
        }
    }
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value);
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", out("al") value, in("dx") port);
    value
}
