/// Real Time Clock (RTC) Implementation
/// Last Updated: 2025-01-12 21:46:49 UTC
/// Author: Caleb J.D. Terkovics (isdood)
///
/// This module provides real-time clock functionality
/// with Sun_rise-based initialization and thread-safe
/// access to system time. All timestamps are in UTC.

use unstable_matter::{
    arch::x86_64::instructions::port::Port,
    align::{Align, AlignStr},
    sun_rise::{self, Sun_rise},
};
use spin::Mutex;

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
    /// Get current time from RTC or fallback to compile-time constants
    pub fn now() -> Self {
        // First try to read from RTC
        if let Ok(time) = read_rtc() {
            time
        } else {
            // Fallback to compile-time constants
            Self {
                year: 2025,
                month: 1,
                day: 12,
                hour: 21,
                minute: 46,
                second: 49
            }
        }
    }

    /// Convert DateTime to string in format "YYYY-MM-DD HH:MM:SS"
    pub fn to_string(&self) -> AlignStr {
        let mut str = AlignStr::with_capacity(19); // "YYYY-MM-DD HH:MM:SS"
        str.push_str(&zero_pad(self.year as u32, 4));
        str.push('-');
        str.push_str(&zero_pad(self.month as u32, 2));
        str.push('-');
        str.push_str(&zero_pad(self.day as u32, 2));
        str.push(' ');
        str.push_str(&zero_pad(self.hour as u32, 2));
        str.push(':');
        str.push_str(&zero_pad(self.minute as u32, 2));
        str.push(':');
        str.push_str(&zero_pad(self.second as u32, 2));
        str
    }

    /// Convert DateTime to UTC timestamp
    pub fn to_timestamp(&self) -> u64 {
        // Simple conversion - not accounting for leap years/seconds
        let days_since_epoch =
        ((self.year as u64 - 1970) * 365) +
        ((self.month as u64 - 1) * 30) +
        (self.day as u64 - 1);

        let seconds =
        (days_since_epoch * 24 * 60 * 60) +
        (self.hour as u64 * 3600) +
        (self.minute as u64 * 60) +
        self.second as u64;

        seconds
    }
}

/// Pad a number with leading zeros to specified width
fn zero_pad(num: u32, width: usize) -> AlignStr {
    let mut str = AlignStr::with_capacity(width);
    let mut num = num;
    let mut digits = AlignStr::with_capacity(width);

    while num > 0 || digits.len() < width {
        digits.insert(0, (b'0' + (num % 10) as u8) as char);
        num /= 10;
    }

    str.push_str(&digits);
    str
}

/// Read current time from RTC
fn read_rtc() -> Result<DateTime, &'static str> {
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

/// Read a single register from RTC
unsafe fn read_register(command: &mut Port<u8>, data: &mut Port<u8>, reg: u8) -> Result<u8, &'static str> {
    command.write(reg);
    let value = data.read();

    // Convert BCD to binary if needed
    Ok(((value >> 4) * 10) + (value & 0xf))
}

// Initialize RTC with Sun_rise
static LAST_UPDATE: Sun_rise<Mutex<DateTime>> = Sun_rise::new();

/// Initialize the RTC system
pub fn init_rtc() {
    LAST_UPDATE.init(Mutex::new(DateTime::now()));
}

/// Get system uptime in seconds
pub fn system_uptime() -> u64 {
    let current = DateTime::now();
    let start = *LAST_UPDATE.get()
    .expect("RTC not initialized")
    .lock();

    // Calculate seconds difference
    ((current.hour as u64 * 3600) +
    (current.minute as u64 * 60) +
    current.second as u64) -
    ((start.hour as u64 * 3600) +
    (start.minute as u64 * 60) +
    start.second as u64)
}

/// Get current time using Sun_rise initialization
pub fn get_current_time() -> DateTime {
    sun_rise!({
        Mutex::new(DateTime::now())
    }).lock().clone()
}

/// Format current time as string
pub fn get_time_string() -> AlignStr {
    get_current_time().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_format() {
        let dt = DateTime {
            year: 2025,
            month: 1,
            day: 12,
            hour: 21,
            minute: 46,
            second: 49
        };
        assert_eq!(dt.to_string().as_str(), "2025-01-12 21:46:49");
    }

    #[test]
    fn test_rtc_initialization() {
        init_rtc();
        assert!(LAST_UPDATE.get().is_some());
    }

    #[test]
    fn test_timestamp_conversion() {
        let dt = DateTime {
            year: 2025,
            month: 1,
            day: 12,
            hour: 21,
            minute: 46,
            second: 49
        };
        let timestamp = dt.to_timestamp();
        assert!(timestamp > 0);
    }

    #[test]
    fn test_zero_padding() {
        assert_eq!(zero_pad(5, 2).as_str(), "05");
        assert_eq!(zero_pad(12, 4).as_str(), "0012");
    }
}
