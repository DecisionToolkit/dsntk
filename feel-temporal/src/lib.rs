#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate dsntk_macros;

mod defs;
mod errors;
mod feel_date;
mod feel_date_time;
mod feel_dt_duration;
mod feel_time;
mod feel_ym_duration;
mod feel_zone;

#[cfg(test)]
mod tests;

pub use defs::{Day, DayOfWeek, DayOfYear, Month, MonthOfYear, WeekOfYear, Year};
pub use feel_date::FeelDate;
pub use feel_date_time::FeelDateTime;
pub use feel_dt_duration::FeelDaysAndTimeDuration;
pub use feel_time::FeelTime;
pub use feel_ym_duration::FeelYearsAndMonthsDuration;
pub use feel_zone::FeelZone;
