use core::fmt::Display;

use crate::drivers::rtc::{Cmos, RtcData};

pub const CURRENT_YEAR: usize = 2022;

pub struct Time {
    second: u8,
    minute: u8,
    hour: u8,
}

impl Time {
    pub const fn new(second: u8, minute: u8, hour: u8) -> Self {
        Self {
            second,
            minute,
            hour,
        }
    }

    pub fn get_current() -> Self {
        Self::from(Cmos::new().read_rtc())
    }
}

pub struct Date {
    day: u8,
    month: u8,
    year: usize,
}

impl Date {
    pub const fn new(day: u8, month: u8, year: usize) -> Self {
        Self { day, month, year }
    }

    pub fn get_current() -> Self {
        Self::from(Cmos::new().read_rtc())
    }
}

impl From<RtcData> for Time {
    fn from(rtc: RtcData) -> Self {
        Self::new(rtc.second, rtc.minute, rtc.hour)
    }
}

impl From<RtcData> for Date {
    fn from(rtc: RtcData) -> Self {
        Self::new(rtc.day, rtc.month, rtc.year)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.", self.day, self.month, self.year)
    }
}
