use x86_64::instructions::port::Port;

const CMOS_COMMAND: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

// CMOS register indices
const CMOS_SECONDS: u8 = 0x00;
const CMOS_MINUTES: u8 = 0x02;
const CMOS_HOURS: u8 = 0x04;
const CMOS_DAY: u8 = 0x07;
const CMOS_MONTH: u8 = 0x08;
const CMOS_YEAR: u8 = 0x09;
const CMOS_STATUS_B: u8 = 0x0B;

pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

fn read_cmos(reg: u8) -> u8 {
    unsafe {
        let mut command_port = Port::new(CMOS_COMMAND);
        let mut data_port = Port::new(CMOS_DATA);
        command_port.write(reg);
        data_port.read()
    }
}

fn is_updating() -> bool {
    unsafe {
        let mut port = Port::new(CMOS_COMMAND);
        port.write(0x0A);
        let mut data_port = Port::new(CMOS_DATA);
        (data_port.read() & 0x80) != 0
    }
}

fn bcd_to_binary(bcd: u8) -> u8 {
    ((bcd & 0xF0) >> 4) * 10 + (bcd & 0x0F)
}

pub fn read_rtc() -> DateTime {
    // Wait until RTC is not updating
    while is_updating() {}

    // Check if values are in BCD or binary format
    let status_b = read_cmos(CMOS_STATUS_B);
    let is_bcd = (status_b & 0x04) == 0;

    let mut seconds = read_cmos(CMOS_SECONDS);
    let mut minutes = read_cmos(CMOS_MINUTES);
    let mut hours = read_cmos(CMOS_HOURS);
    let mut day = read_cmos(CMOS_DAY);
    let mut month = read_cmos(CMOS_MONTH);
    let mut year = read_cmos(CMOS_YEAR) as u16;

    if is_bcd {
        seconds = bcd_to_binary(seconds);
        minutes = bcd_to_binary(minutes);
        hours = bcd_to_binary(hours);
        day = bcd_to_binary(day);
        month = bcd_to_binary(month);
        year = bcd_to_binary(year as u8) as u16;
    }

    // Convert 2-digit year to 4-digit year
    year += 2000;

    DateTime {
        year,
        month,
        day,
        hours,
        minutes,
        seconds,
    }
}

pub fn format_datetime(dt: &DateTime) -> alloc::string::String {
    use alloc::format;
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        dt.year, dt.month, dt.day,
        dt.hours, dt.minutes, dt.seconds
    )
}
