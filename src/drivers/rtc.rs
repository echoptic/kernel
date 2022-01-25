use spin::Once;
use x86_64::instructions::{self, port::Port};

use crate::date_time::CURRENT_YEAR;

// TODO: Find and set century register
static CENTURY_REGISTER: Once<u8> = Once::new();

pub struct Cmos {
    address_port: Port<u8>,
    data_port: Port<u8>,
}

impl Cmos {
    pub const fn new() -> Self {
        Self {
            address_port: Port::new(0x70),
            data_port: Port::new(0x71),
        }
    }

    fn get_rtc_register(&mut self, reg: u8) -> u8 {
        // TODO: Is it necessary to disable interrupts here?
        instructions::interrupts::without_interrupts(|| unsafe {
            self.address_port.write(reg);
            self.data_port.read()
        })
    }

    fn get_update_in_progress_flag(&mut self) -> bool {
        (self.get_rtc_register(0x0a) & 0x80) > 0
    }

    pub fn read_rtc(&mut self) -> RtcData {
        while self.get_update_in_progress_flag() {}
        let mut second = self.get_rtc_register(0x00);
        let mut minute = self.get_rtc_register(0x02);
        let mut hour = self.get_rtc_register(0x04);
        let mut day = self.get_rtc_register(0x07);
        let mut month = self.get_rtc_register(0x08);
        let mut year = self.get_rtc_register(0x09) as usize;
        let mut century = if let Some(reg) = CENTURY_REGISTER.get() {
            self.get_rtc_register(*reg)
        } else {
            0
        };

        let mut last_second;
        let mut last_minute;
        let mut last_hour;
        let mut last_day;
        let mut last_month;
        let mut last_year;
        let mut last_century;

        while {
            last_second = second;
            last_minute = minute;
            last_hour = hour;
            last_day = day;
            last_month = month;
            last_year = year;
            last_century = century;

            while self.get_update_in_progress_flag() {}
            second = self.get_rtc_register(0x00);
            minute = self.get_rtc_register(0x02);
            hour = self.get_rtc_register(0x04);
            day = self.get_rtc_register(0x07);
            month = self.get_rtc_register(0x08);
            year = self.get_rtc_register(0x09) as usize;
            if let Some(reg) = CENTURY_REGISTER.get() {
                century = self.get_rtc_register(*reg);
            }

            (last_second != second)
                || (last_minute != minute)
                || (last_hour != hour)
                || (last_day != day)
                || (last_month != month)
                || (last_year != year)
                || (last_century != century)
        } {}

        let register_b = self.get_rtc_register(0x0b);

        // Convert BCD to binary values if necessary
        if (register_b & 0x04) == 0 {
            second = (second & 0x0F) + ((second / 16) * 10);
            minute = (minute & 0x0F) + ((minute / 16) * 10);
            hour = ((hour & 0x0F) + (((hour & 0x70) / 16) * 10)) | (hour & 0x80);
            day = (day & 0x0F) + ((day / 16) * 10);
            month = (month & 0x0F) + ((month / 16) * 10);
            year = (year & 0x0F) + ((year / 16) * 10);
            if CENTURY_REGISTER.is_completed() {
                century = (century & 0x0F) + ((century / 16) * 10);
            }
        }

        // Convert 12 hour clock to 24 hour clock if necessary
        if (register_b & 0x02) == 0 && (hour & 0x80) != 0 {
            hour = ((hour & 0x7F) + 12) % 24;
        }

        // Calculate the full (4-digit) year
        if CENTURY_REGISTER.is_completed() {
            year += century as usize * 100;
        } else {
            year += (CURRENT_YEAR / 100) * 100;
            if year < CURRENT_YEAR {
                year += 100;
            }
        }

        RtcData::new(second, minute, hour, day, month, year, century)
    }
}

pub struct RtcData {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: usize,
    pub century: u8,
}

impl RtcData {
    pub const fn new(
        second: u8,
        minute: u8,
        hour: u8,
        day: u8,
        month: u8,
        year: usize,
        century: u8,
    ) -> Self {
        Self {
            second,
            minute,
            hour,
            day,
            month,
            year,
            century,
        }
    }
}
