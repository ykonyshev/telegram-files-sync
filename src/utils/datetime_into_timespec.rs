use sea_orm::prelude::DateTimeUtc;
use time::Timespec;

pub fn datetime_into_timespec(from: DateTimeUtc) -> Timespec {
    Timespec {
        sec: from.timestamp(),
        nsec: from.timestamp_subsec_nanos() as i32,
    }
}
