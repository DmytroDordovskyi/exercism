use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_hours = hours + minutes.div_euclid(MINUTES_PER_HOUR);

        Self {
            hours: total_hours.rem_euclid(HOURS_PER_DAY),
            minutes: minutes.rem_euclid(MINUTES_PER_HOUR),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes;
        let total_hours = self.hours + total_minutes.div_euclid(MINUTES_PER_HOUR);

        Self {
            hours: total_hours.rem_euclid(HOURS_PER_DAY),
            minutes: total_minutes.rem_euclid(MINUTES_PER_HOUR),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
