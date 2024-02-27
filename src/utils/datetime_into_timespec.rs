use chrono::NaiveDateTime;
use time::Timespec;

pub fn datetime_into_timespec(from: NaiveDateTime) -> Timespec {
    Timespec {
        sec: from.timestamp(),
        nsec: from.timestamp_subsec_nanos() as i32,
    }
}
