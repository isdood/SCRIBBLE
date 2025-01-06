use x86_64::instructions::port::Port;
use lazy_static::lazy_static;

pub struct RTC {
    address: Port<u8>,
    data: Port<u8>,
}

impl RTC {
    pub const fn new() -> RTC {
        RTC {
            address: Port::new(0x70),
            data: Port::new(0x71),
        }
    }

    unsafe fn read_register(&mut self, reg: u8) -> u8 {
        self.address.write(reg);
        self.data.read()
    }

    pub fn get_time(&mut self) -> (u8, u8, u8) {
        unsafe {
            // Read hours, minutes, seconds from RTC
            let seconds = self.read_register(0x00);
            let minutes = self.read_register(0x02);
            let hours = self.read_register(0x04);

            // Convert from BCD to binary
            let seconds = ((seconds & 0xF0) >> 4) * 10 + (seconds & 0x0F);
            let minutes = ((minutes & 0xF0) >> 4) * 10 + (minutes & 0x0F);
            let hours = ((hours & 0xF0) >> 4) * 10 + (hours & 0x0F);

            (hours, minutes, seconds)
        }
    }

    pub fn get_date(&mut self) -> (u16, u8, u8) {
        unsafe {
            // Read year, month, day from RTC
            let year = self.read_register(0x09) as u16;
            let month = self.read_register(0x08);
            let day = self.read_register(0x07);

            // Convert from BCD to binary
            let year = 2000 + ((year & 0xF0) >> 4) * 10 + (year & 0x0F) as u16;
            let month = ((month & 0xF0) >> 4) * 10 + (month & 0x0F);
            let day = ((day & 0xF0) >> 4) * 10 + (day & 0x0F);

            (year, month, day)
        }
    }
}

lazy_static! {
    pub static ref RTC_DEVICE: Mutex<RTC> = Mutex::new(RTC::new());
}
