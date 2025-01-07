//IMPORTS
use x86_64::instructions::port::Port;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::splat::{self, SplatLevel};
use core::time::Duration;
use alloc::string::String;
use alloc::format;

pub mod rtc;

// RTC Hardware Constants
const RTC_ADDRESS_PORT: u16 = 0x70;
const RTC_DATA_PORT: u16 = 0x71;
const RTC_UPDATE_IN_PROGRESS_FLAG: u8 = 0x80;

// RTC Registers
const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;
const RTC_STATUS_A: u8 = 0x0A;
const RTC_STATUS_B: u8 = 0x0B;
const RTC_STATUS_C: u8 = 0x0C;
const RTC_STATUS_D: u8 = 0x0D;

// Configuration Constants
const MAX_RTC_ATTEMPTS: u8 = 3;
const CENTURY_BASE: u16 = 2000;
const RTC_TIMEOUT: u16 = 1000;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl DateTime {
    pub fn now() -> Self {
        RTC_DEVICE.lock().get_datetime()
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day,
            self.hours, self.minutes, self.seconds
        )
    }

    fn is_valid(&self) -> bool {
        let days_in_month = match self.month {
            2 => if self.year % 4 == 0 { 29 } else { 28 },
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => return false,
        };

        self.month <= 12 &&
        self.day <= days_in_month &&
        self.hours < 24 &&
        self.minutes < 60 &&
        self.seconds < 60
    }
}

#[derive(Debug)]
pub enum RTCError {
    UpdateInProgress,
    InvalidValue(u8),
    ReadError,
    Timeout,
    ValidationFailed,
    LockError,
}

pub struct RTC {
    address: Port<u8>,
    data: Port<u8>,
    last_read: Option<DateTime>,
    update_count: u32,
    error_count: u32,
}

impl RTC {
    pub const fn new() -> RTC {
        RTC {
            address: Port::new(RTC_ADDRESS_PORT),
            data: Port::new(RTC_DATA_PORT),
            last_read: None,
            update_count: 0,
            error_count: 0,
        }
    }

    unsafe fn read_register(&mut self, reg: u8) -> Result<u8, RTCError> {
        let mut timeout = RTC_TIMEOUT;

        // Disable NMI while reading
        self.address.write(reg | 0x80);

        // Wait for update to complete
        while timeout > 0 {
            self.address.write(RTC_STATUS_A);
            if self.data.read() & RTC_UPDATE_IN_PROGRESS_FLAG == 0 {
                break;
            }
            timeout -= 1;
        }

        if timeout == 0 {
            self.error_count += 1;
            return Err(RTCError::Timeout);
        }

        // Read the actual register
        self.address.write(reg);
        let value = self.data.read();

        // Re-enable NMI
        self.address.write(reg);

        // Validate value
        if value > 0x99 {
            self.error_count += 1;
            return Err(RTCError::InvalidValue(value));
        }

        Ok(value)
    }

    fn bcd_to_binary(bcd: u8) -> u8 {
        ((bcd & 0xF0) >> 4) * 10 + (bcd & 0x0F)
    }

    pub fn get_datetime(&mut self) -> DateTime {
        let mut attempts = 0;

        while attempts < MAX_RTC_ATTEMPTS {
            match self.try_get_datetime() {
                Ok(datetime) => {
                    if datetime.is_valid() {
                        self.last_read = Some(datetime);
                        self.update_count += 1;
                        return datetime;
                    }
                }
                Err(e) => {
                    splat::log(
                        SplatLevel::BitsNBytes,
                        &format!("RTC read attempt {} failed: {:?}", attempts + 1, e)
                    );
                }
            }
            attempts += 1;
        }

        // Return last known good time or default
        self.error_count += 1;
        self.last_read.unwrap_or(DateTime {
            year: CENTURY_BASE + 24,
            month: 1,
            day: 1,
            hours: 0,
            minutes: 0,
            seconds: 0,
        })
    }

    fn try_get_datetime(&mut self) -> Result<DateTime, RTCError> {
        unsafe {
            let seconds = Self::bcd_to_binary(self.read_register(RTC_SECONDS)?);
            let minutes = Self::bcd_to_binary(self.read_register(RTC_MINUTES)?);
            let hours = Self::bcd_to_binary(self.read_register(RTC_HOURS)?);
            let day = Self::bcd_to_binary(self.read_register(RTC_DAY)?);
            let month = Self::bcd_to_binary(self.read_register(RTC_MONTH)?);
            let year = CENTURY_BASE + Self::bcd_to_binary(self.read_register(RTC_YEAR)?) as u16;

            let datetime = DateTime {
                year,
                month,
                day,
                hours,
                minutes,
                seconds,
            };

            if !datetime.is_valid() {
                return Err(RTCError::ValidationFailed);
            }

            Ok(datetime)
        }
    }

    pub fn uptime(&self) -> Duration {
        if let Some(initial) = self.last_read {
            let now = self.get_datetime();
            let seconds = (now.hours as u64 * 3600 +
            now.minutes as u64 * 60 +
            now.seconds as u64) -
            (initial.hours as u64 * 3600 +
            initial.minutes as u64 * 60 +
            initial.seconds as u64);
            Duration::from_secs(seconds)
        } else {
            Duration::from_secs(0)
        }
    }

    pub fn get_stats(&self) -> (u32, u32) {
        (self.update_count, self.error_count)
    }
}

lazy_static! {
    pub static ref RTC_DEVICE: Mutex<RTC> = {
        let rtc = RTC::new();
        splat::log(SplatLevel::BitsNBytes, "RTC hardware initialized");
        Mutex::new(rtc)
    };
}

// Public interface
pub fn get_current_time() -> Result<DateTime, RTCError> {
    RTC_DEVICE
    .try_lock()
    .map(|mut rtc| rtc.get_datetime())
    .ok_or(RTCError::LockError)
}

pub fn log_system_uptime() {
    if let Some(rtc) = RTC_DEVICE.try_lock() {
        let uptime = rtc.uptime();
        let (updates, errors) = rtc.get_stats();
        splat::log(
            SplatLevel::BitsNBytes,
            &format!(
                "RTC Status:\n\
└─ Uptime: {}s\n\
└─ Updates: {}\n\
└─ Errors: {}\n\
└─ Reliability: {:.2}%",
uptime.as_secs(),
                     updates,
                     errors,
                     if updates > 0 {
                         100.0 * (1.0 - (errors as f32 / updates as f32))
                     } else {
                         0.0
                     }
            )
        );
    }
}

pub fn test_rtc() -> bool {
    match RTC_DEVICE.try_lock() {
        Some(mut rtc) => match rtc.try_get_datetime() {
            Ok(datetime) => {
                splat::log(
                    SplatLevel::BitsNBytes,
                    &format!("RTC test successful: {}", datetime.to_string())
                );
                true
            }
            Err(e) => {
                splat::log(
                    SplatLevel::Critical,
                    &format!("RTC test failed: {:?}", e)
                );
                false
            }
        },
        None => {
            splat::log(SplatLevel::Critical, "RTC locked during test");
            false
        }
    }
}
