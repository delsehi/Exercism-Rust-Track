use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, min) = Clock::wrap_time(hours, minutes);
        Self {
            hours: h,
            minutes: min,
        }
    }

    fn wrap_time(hours: i32, minutes: i32) -> (i32, i32) {
        let mut h = (hours + (minutes / 60)) % 24; // h is calculated by how many hours there are in minutes
                                                   // add those hours to total hours, and then wrap that around 24 hours
        let mut min = minutes % 60; // Add the rest of the minutes, the ones not added above.
        if minutes < 0 {
            // If the minutes are negative
            min = 60 + min; // Then add the minutes, min is negative here (-30 to 11:00 should give e.g. 10:30 )
            h -= 1; // And subtract one hour.
        }
        if h < 0 {
            // If the result is negative
            h = 24 + h; // it should go backwards from 00:00, h is negative here.
        }
        if min >= 60 {
            // Should hour increase? If it's beyond 59 it should
            h += 1; // Add an hour
            let tmp = min; // Save the value
            min = 0; // reset minutes to zero
            min += tmp % 60; // Add any remainder
        }
        (h, min)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, min) = Clock::wrap_time(self.hours, self.minutes + minutes);
        Self {
            hours: h,
            minutes: min,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (self.hours < 10, self.minutes < 10) {
            (true, true) => write!(f, "0{}:0{}", self.hours, self.minutes),
            (true, false) => write!(f, "0{}:{}", self.hours, self.minutes),
            (false, false) => write!(f, "{}:{}", self.hours, self.minutes),
            (false, true) => write!(f, "{}:0{}", self.hours, self.minutes),
        }
    }
}
