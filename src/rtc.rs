// src/rtc.rs
use x86_64::instructions::port::Port;
use spin::Mutex;
use lazy_static::lazy_static;
use alloc::string::String;
use alloc::format;

// RTC port constants
const CMOS_COMMAND: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

// RTC register indices
const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8
}

impl DateTime {
    pub fn now() -> Self {
        // First try to read from RTC
        if let Ok(time) = read_rtc() {
            time
        } else {
            // Fallback to compile-time constants
            Self {
                year: 2025,
                month: 1,
                day: 7,
                hour: 6,
                minute: 36,
                second: 38
            }
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day,
            self.hour, self.minute, self.second
        )
    }
}

fn read_rtc() -> Result<DateTime, &'static str> {
    // Disable NMI
    let mut command_port = Port::new(CMOS_COMMAND);
    let mut data_port = Port::new(CMOS_DATA);

    unsafe {
        // Read each component
        let second = read_register(&mut command_port, &mut data_port, RTC_SECONDS)?;
        let minute = read_register(&mut command_port, &mut data_port, RTC_MINUTES)?;
        let hour = read_register(&mut command_port, &mut data_port, RTC_HOURS)?;
        let day = read_register(&mut command_port, &mut data_port, RTC_DAY)?;
        let month = read_register(&mut command_port, &mut data_port, RTC_MONTH)?;
        let year = read_register(&mut command_port, &mut data_port, RTC_YEAR)? as u16 + 2000;

        Ok(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second
        })
    }
}

unsafe fn read_register(command: &mut Port<u8>, data: &mut Port<u8>, reg: u8) -> Result<u8, &'static str> {
    command.write(reg);
    let value = data.read();

    // Convert BCD to binary if needed
    Ok(((value >> 4) * 10) + (value & 0xf))
}

lazy_static! {
    static ref LAST_UPDATE: Mutex<DateTime> = Mutex::new(DateTime::now());
}

// Add this function to track system uptime
pub fn system_uptime() -> u64 {
    let current = DateTime::now();
    let start = *LAST_UPDATE.lock();

    // Calculate seconds difference
    ((current.hour as u64 * 3600) +
    (current.minute as u64 * 60) +
    current.second as u64) -
    ((start.hour as u64 * 3600) +
    (start.minute as u64 * 60) +
    start.second as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_format() {
        let dt = DateTime {
            year: 2025,
            month: 1,
            day: 7,
            hour: 6,
            minute: 36,
            second: 38
        };
        assert_eq!(dt.to_string(), "2025-01-07 06:36:38");
    }
}
