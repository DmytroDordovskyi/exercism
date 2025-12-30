// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    const DAYS_IN_YEAR: f64 = 365.25 * Self::ORBITAL_PERIOD;
    const HOURS_IN_DAY: f64 = 24.0;
    const MINUTES_IN_HOUR: f64 = 60.0;
    const SECONDS_IN_MINUTE: f64 = 60.0;
    const SECONDS_IN_YEAR: f64 =
        Self::DAYS_IN_YEAR * Self::HOURS_IN_DAY * Self::MINUTES_IN_HOUR * Self::SECONDS_IN_MINUTE;

    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / Self::SECONDS_IN_YEAR
    }
}

macro_rules! planet {
    ($name:ident, $orbital_period:literal) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $orbital_period;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
