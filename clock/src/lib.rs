use std::fmt;

const DAY: i32 = 1440;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub const fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Self {
            // minutes: (((hours * 60 + minutes) % DAY) + DAY) % DAY,
            minutes: total_minutes.rem_euclid(DAY),
        }
    }

    #[must_use]
    pub const fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
