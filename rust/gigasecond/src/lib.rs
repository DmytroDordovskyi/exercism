use std::ops::Add;
use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

// Returns a DateTime one billion seconds after start.

pub fn after(start: DateTime) -> DateTime {
    start.add(1000000000.seconds())
}
