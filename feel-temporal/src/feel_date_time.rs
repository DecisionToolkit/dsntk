//! Implementation of FEEL date and time.

use super::errors::err_invalid_date_time_literal;
use super::feel_date::FeelDate;
use super::feel_time::FeelTime;
use super::feel_zone::FeelZone;
use crate::FeelDaysAndTimeDuration;
use crate::defs::*;
use crate::errors::err_date_time_conversion_failed;
use crate::feel_ym_duration::FeelYearsAndMonthsDuration;
use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, Timelike};
use dsntk_common::{DsntkError, Result};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Sub};

/// FEEL date and time.
#[derive(Debug, Clone)]
pub struct FeelDateTime(FeelDate, FeelTime);

impl Display for FeelDateTime {
  /// Implements [Display] trait for date and time.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}T{}", self.0, self.1)
  }
}

impl TryFrom<&str> for FeelDateTime {
  type Error = DsntkError;
  /// Converts string into [FeelDateTime].
  fn try_from(s: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_DATE_AND_TIME.captures(s)
      && let Some(year_match) = captures.name("year")
      && let Ok(mut year) = year_match.as_str().parse::<Year>()
    {
      if captures.name("sign").is_some() {
        year = -year;
      }
      if let Some(month_match) = captures.name("month")
        && let Ok(month) = month_match.as_str().parse::<Month>()
        && let Some(day_match) = captures.name("day")
        && let Ok(day) = day_match.as_str().parse::<Day>()
        && let Some(hour_match) = captures.name("hours")
        && let Ok(mut hour) = hour_match.as_str().parse::<u8>()
        && let Some(min_match) = captures.name("minutes")
        && let Ok(min) = min_match.as_str().parse::<u8>()
        && let Some(sec_match) = captures.name("seconds")
        && let Ok(sec) = sec_match.as_str().parse::<u8>()
      {
        let fractional = if let Some(frac_match) = captures.name("fractional") {
          frac_match.as_str().parse::<f64>().unwrap_or(0.0)
        } else {
          0.0
        };
        let nanos = (fractional * 1e9).trunc() as u64;
        if let Some(mut date) = FeelDate::new(year, month, day)
          && let Some(zone) = FeelZone::from_captures(&captures)
        {
          if !is_valid_time(hour, min, sec) {
            return Err(err_invalid_date_time_literal(s));
          }
          if hour == 24 {
            if let Some(updated_date) = date.add_days(1) {
              hour = 0;
              date = updated_date;
            } else {
              return Err(err_invalid_date_time_literal(s));
            }
          }
          if let Some(time) = FeelTime::zone_opt(hour, min, sec, nanos, zone) {
            return Ok(FeelDateTime(date, time));
          }
        }
      }
    }
    Err(err_invalid_date_time_literal(s))
  }
}

impl PartialEq for FeelDateTime {
  fn eq(&self, rhs: &Self) -> bool {
    let lhs_zone = self.1.zone();
    let rhs_zone = rhs.1.zone();
    let lhs_date_tuple = self.0.as_tuple();
    let lhs_time_tuple = ((self.1).hour() as u32, (self.1).minute() as u32, (self.1).second() as u32, 0);
    let lhs_offset_opt = match lhs_zone {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_t(lhs_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, lhs_date_tuple, lhs_time_tuple),
    };
    let rhs_date_tuple = rhs.0.as_tuple();
    let rhs_time_tuple = ((rhs.1).hour() as u32, (rhs.1).minute() as u32, (rhs.1).second() as u32, 0);
    let rhs_offset_opt = match rhs_zone {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_t(rhs_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, rhs_date_tuple, rhs_time_tuple),
    };
    if let Some((lhs_offset, rhs_offset)) = lhs_offset_opt.zip(rhs_offset_opt) {
      let lhs_date_opt = date_time_offset_dt(lhs_date_tuple, lhs_time_tuple, lhs_offset);
      let rhs_date_opt = date_time_offset_dt(rhs_date_tuple, rhs_time_tuple, rhs_offset);
      if let Some((lhs_date, rhs_date)) = lhs_date_opt.zip(rhs_date_opt) {
        return lhs_date == rhs_date;
      }
    }
    false
  }
}

impl PartialOrd for FeelDateTime {
  /// Returns the ordering of two date and times.
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let d = self.0.cmp(&other.0);
    if let Some(t) = self.1.partial_cmp(&other.1) {
      return match (d, t) {
        (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
        (Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
        (Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
        (Ordering::Less, _) => Some(Ordering::Less),
        (Ordering::Greater, _) => Some(Ordering::Greater),
      };
    }
    None
  }
}

impl Add<FeelYearsAndMonthsDuration> for FeelDateTime {
  type Output = Option<Self>;
  /// Adds [FeelYearsAndMonthsDuration] to [FeelDateTime].
  fn add(self, rhs: FeelYearsAndMonthsDuration) -> Self::Output {
    if rhs.is_negative() {
      return self.sub(rhs.abs());
    }
    if let Some(date) = self.0.add(rhs) { Some(FeelDateTime(date, self.1)) } else { None }
  }
}

impl Sub<FeelYearsAndMonthsDuration> for FeelDateTime {
  type Output = Option<Self>;
  /// Subtracts [FeelYearsAndMonthsDuration] from [FeelDateTime].
  fn sub(self, rhs: FeelYearsAndMonthsDuration) -> Self::Output {
    if rhs.is_negative() {
      return self.add(rhs.abs());
    }
    if let Some(date) = self.0.sub(rhs) { Some(FeelDateTime(date, self.1)) } else { None }
  }
}

impl Add<FeelDaysAndTimeDuration> for FeelDateTime {
  type Output = Option<Self>;
  /// Adds [FeelDaysAndTimeDuration] to [FeelDateTime].
  fn add(self, rhs: FeelDaysAndTimeDuration) -> Self::Output {
    if rhs.is_negative() {
      return self.sub(rhs.abs());
    }
    let zone = self.1.zone().clone();
    let date_time = <FeelDateTime as TryInto<DateTime<FixedOffset>>>::try_into(self)
      .ok()?
      .checked_add_signed(Duration::nanoseconds(rhs.as_nanos()))?;
    let date = FeelDate::new(date_time.year(), date_time.month(), date_time.day())?;
    let time = FeelTime::zone_opt(
      date_time.hour() as u8,
      date_time.minute() as u8,
      date_time.second() as u8,
      date_time.nanosecond() as u64,
      zone,
    )?;
    Some(FeelDateTime(date, time))
  }
}

impl Sub<FeelDaysAndTimeDuration> for FeelDateTime {
  type Output = Option<Self>;
  /// Subtracts [FeelDaysAndTimeDuration] from [FeelDateTime].
  fn sub(self, rhs: FeelDaysAndTimeDuration) -> Self::Output {
    if rhs.is_negative() {
      return self.add(rhs.abs());
    }
    let zone = self.1.zone().clone();
    let mut date_time = <FeelDateTime as TryInto<DateTime<FixedOffset>>>::try_into(self).ok()?;
    date_time -= Duration::nanoseconds(rhs.as_nanos());
    let date = FeelDate::new(date_time.year(), date_time.month(), date_time.day())?;
    let time = FeelTime::zone_opt(
      date_time.hour() as u8,
      date_time.minute() as u8,
      date_time.second() as u8,
      date_time.nanosecond() as u64,
      zone,
    )?;
    Some(FeelDateTime(date, time))
  }
}

impl Sub<FeelDateTime> for FeelDateTime {
  type Output = Option<FeelDaysAndTimeDuration>;
  fn sub(self, other: FeelDateTime) -> Self::Output {
    let me_zone = self.1.zone();
    let other_zone = other.1.zone();
    if me_zone.has_offset() != other_zone.has_offset() {
      return None;
    }
    let me_date_tuple = self.0.as_tuple();
    let me_time_tuple = ((self.1).hour() as u32, (self.1).minute() as u32, (self.1).second() as u32, (self.1).nanos() as u32);
    let me_offset_opt = match me_zone {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_dt(me_date_tuple, me_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, me_date_tuple, me_time_tuple),
    };
    let other_date_tuple = other.0.as_tuple();
    let other_time_tuple = ((other.1).hour() as u32, (other.1).minute() as u32, (other.1).second() as u32, (other.1).nanos() as u32);
    let other_offset_opt = match other_zone {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_dt(other_date_tuple, other_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, other_date_tuple, other_time_tuple),
    };
    if let Some((me_offset, other_offset)) = me_offset_opt.zip(other_offset_opt) {
      let me_date_opt = date_time_offset_dt(me_date_tuple, me_time_tuple, me_offset);
      let other_date_opt = date_time_offset_dt(other_date_tuple, other_time_tuple, other_offset);
      if let Some((me_date, other_date)) = me_date_opt.zip(other_date_opt)
        && let Some(nanos) = me_date.sub(other_date).num_nanoseconds()
      {
        return Some(FeelDaysAndTimeDuration::from_n(nanos));
      }
    }
    None
  }
}

impl TryFrom<FeelDateTime> for DateTime<FixedOffset> {
  type Error = DsntkError;
  fn try_from(value: FeelDateTime) -> Result<Self, Self::Error> {
    let me_date_tuple = value.0.as_tuple();
    let me_time_tuple = ((value.1).hour() as u32, (value.1).minute() as u32, (value.1).second() as u32, (value.1).nanos() as u32);
    let me_offset_opt = match (value.1).zone() {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_dt(me_date_tuple, me_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, me_date_tuple, me_time_tuple),
    };
    if let Some(me_offset) = me_offset_opt
      && let Some(me_date) = date_time_offset_dt(me_date_tuple, me_time_tuple, me_offset)
    {
      return Ok(me_date);
    }
    Err(err_date_time_conversion_failed(&value.to_string()))
  }
}

impl FeelDateTime {
  /// Creates date and time from provided [FeelDate] and [FeelTime] values.
  pub fn new(date: FeelDate, time: FeelTime) -> Self {
    Self(date, time)
  }

  /// Creates UTC date and time from specified date and time values.
  pub fn utc(year: Year, month: Month, day: Day, hour: u8, minute: u8, second: u8, nanosecond: u64) -> Option<Self> {
    let date = FeelDate::new(year, month, day)?;
    let time = FeelTime::utc_opt(hour, minute, second, nanosecond)?;
    Some(Self(date, time))
  }

  /// Creates local date and time from specified date and time values.
  pub fn local(year: Year, month: Month, day: Day, hour: u8, minute: u8, second: u8, nanosecond: u64) -> Option<Self> {
    let date = FeelDate::new(year, month, day)?;
    let time = FeelTime::local_opt(hour, minute, second, nanosecond)?;
    Some(Self(date, time))
  }

  /// Creates  date and time from specified date, time and offset values.
  pub fn offset(date: (Year, Month, Day), time: (u8, u8, u8, u64), offset: i32) -> Option<Self> {
    let date = FeelDate::new(date.0, date.1, date.2)?;
    let time = FeelTime::offset_opt(time.0, time.1, time.2, time.3, offset)?;
    Some(Self(date, time))
  }

  /// Returns today's date and time (local time).
  pub fn now() -> Self {
    let now = Local::now();
    Self(
      FeelDate::new(now.year(), now.month(), now.day()).unwrap(), // Unwrap is ok here, date can not be invalid.
      FeelTime::local(now.hour() as u8, now.minute() as u8, now.second() as u8, now.nanosecond() as u64),
    )
  }

  /// Returns the `Date` part from date and time value.
  pub fn date(&self) -> FeelDate {
    self.0.clone()
  }

  /// Returns the `Time` part from date and time value.
  pub fn time(&self) -> FeelTime {
    self.1.clone()
  }

  pub fn year(&self) -> Year {
    self.0.year()
  }

  pub fn month(&self) -> Month {
    self.0.month()
  }

  pub fn day(&self) -> Day {
    self.0.day()
  }

  pub fn day_of_week(&self) -> Option<DayOfWeek> {
    self.0.day_of_week()
  }

  pub fn day_of_year(&self) -> Option<DayOfYear> {
    self.0.day_of_year()
  }

  pub fn week_of_year(&self) -> Option<WeekOfYear> {
    self.0.week_of_year()
  }

  pub fn month_of_year(&self) -> Option<MonthOfYear> {
    self.0.month_of_year()
  }

  pub fn hour(&self) -> u8 {
    self.1.hour()
  }

  pub fn minute(&self) -> u8 {
    self.1.minute()
  }

  pub fn second(&self) -> u8 {
    self.1.second()
  }

  pub fn feel_time_offset(&self) -> Option<i32> {
    feel_time_offset(self)
  }

  pub fn feel_time_zone(&self) -> Option<String> {
    feel_time_zone(self)
  }

  pub fn is(&self, rhs: &FeelDateTime) -> bool {
    match (self.1.zone(), rhs.1.zone()) {
      (FeelZone::Offset(offset_1), FeelZone::Offset(offset_2)) if offset_1 != offset_2 => {
        // When offsets differ, then even the date and times are equal,
        // then treat them semantically not equal.
        return false;
      }
      (FeelZone::Offset(_), FeelZone::Zone(_)) => {
        // When one date time has offset, and the other one has zone name
        // then treat them semantically not equal.
        return false;
      }
      (FeelZone::Zone(zone_name_1), FeelZone::Zone(zone_name_2)) if zone_name_1 != zone_name_2 => {
        // When zone names differ, then even the date and times are equal,
        // then treat them semantically not equal.
        return false;
      }
      (FeelZone::Zone(_), FeelZone::Offset(_)) => {
        // When one date time has zone name, and the other one has offset
        // then treat them semantically not equal.
        return false;
      }
      _ => {}
    };
    self == rhs
  }
}
