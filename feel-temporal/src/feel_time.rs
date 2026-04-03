//! Implementation of FEEL time.

use super::feel_date::FeelDate;
use super::feel_zone::FeelZone;
use crate::defs::*;
use crate::errors::*;
use crate::feel_date_time::FeelDateTime;
use crate::feel_dt_duration::FeelDaysAndTimeDuration;
use dsntk_common::{DsntkError, Result};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::str::FromStr;

/// FEEL time.
#[derive(Debug, Clone)]
pub struct FeelTime(u8, u8, u8, u64, FeelZone);

impl Display for FeelTime {
  /// Implements [Display] for FEEL time.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.3 > 0 {
      write!(f, "{:02}:{:02}:{:02}.{}{}", self.0, self.1, self.2, nanos_to_string(self.3), self.4)
    } else {
      write!(f, "{:02}:{:02}:{:02}{}", self.0, self.1, self.2, self.4)
    }
  }
}

impl FromStr for FeelTime {
  type Err = DsntkError;
  /// Converts [String] into [FeelTime].
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(captures) = RE_TIME.captures(s)
      && let Some(hour_match) = captures.name("hours")
      && let Ok(mut hour) = hour_match.as_str().parse::<u8>()
      && let Some(min_match) = captures.name("minutes")
      && let Ok(min) = min_match.as_str().parse::<u8>()
      && let Some(sec_match) = captures.name("seconds")
      && let Ok(sec) = sec_match.as_str().parse::<u8>()
    {
      let mut fractional = 0.0;
      if let Some(frac_match) = captures.name("fractional")
        && let Ok(frac) = frac_match.as_str().parse::<f64>()
      {
        fractional = frac;
      }
      let nanos = (fractional * 1e9).trunc() as u64;
      if let Some(zone) = FeelZone::from_captures(&captures)
        && is_valid_time(hour, min, sec)
      {
        if hour == 24 {
          hour = 0;
        }
        let time = FeelTime(hour, min, sec, nanos, zone);
        return Ok(time);
      }
    }
    Err(err_invalid_time_literal(s))
  }
}

impl PartialEq for FeelTime {
  /// nanoseconds are omitted during time comparison
  fn eq(&self, rhs: &Self) -> bool {
    let equal_zones = match (&self.4, &rhs.4) {
      (FeelZone::Utc, FeelZone::Utc) => true,
      (FeelZone::Utc, FeelZone::Zone(zone_name)) | (FeelZone::Zone(zone_name), FeelZone::Utc) => zone_name == ETC_UTC,
      (FeelZone::Utc, FeelZone::Offset(offset)) | (FeelZone::Offset(offset), FeelZone::Utc) => *offset == 0,
      (FeelZone::Local, FeelZone::Local) => true,
      (FeelZone::Offset(offset), FeelZone::Zone(zone_name)) | (FeelZone::Zone(zone_name), FeelZone::Offset(offset)) => {
        (*offset == 0 && zone_name == ETC_UTC)
          || if let Some(zone_offset) = get_zone_offset(zone_name) {
            *offset == zone_offset
          } else {
            false
          }
      }
      (FeelZone::Offset(offset1), FeelZone::Offset(offset2)) => offset1 == offset2,
      (FeelZone::Zone(zone_name1), FeelZone::Zone(zone_name2)) => zone_name1 == zone_name2,
      _ => false,
    };
    self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2 && equal_zones
  }
}

impl PartialOrd for FeelTime {
  /// Returns the ordering of two times.
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.4 == other.4 {
      let h = self.0.cmp(&other.0);
      let m = self.1.cmp(&other.1);
      let s = self.2.cmp(&other.2);
      let n = self.3.cmp(&other.3);
      match (h, m, s, n) {
        (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
        (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
        (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
        (Ordering::Equal, Ordering::Equal, Ordering::Less, _) => Some(Ordering::Less),
        (Ordering::Equal, Ordering::Equal, Ordering::Greater, _) => Some(Ordering::Greater),
        (Ordering::Equal, Ordering::Less, _, _) => Some(Ordering::Less),
        (Ordering::Equal, Ordering::Greater, _, _) => Some(Ordering::Greater),
        (Ordering::Less, _, _, _) => Some(Ordering::Less),
        (Ordering::Greater, _, _, _) => Some(Ordering::Greater),
      }
    } else {
      None
    }
  }
}

impl Sub<FeelTime> for FeelTime {
  type Output = Option<FeelDaysAndTimeDuration>;
  /// Subtracts [FeelTime] values, result is always [FeelDaysAndTimeDuration].
  fn sub(self, other: Self) -> Self::Output {
    let me_time_tuple = (self.0 as u32, self.1 as u32, self.2 as u32, self.3 as u32);
    let me_offset_opt = match &self.4 {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_t(me_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_t(zone_name, me_time_tuple),
    };
    let other_time_tuple = (other.0 as u32, other.1 as u32, other.2 as u32, other.3 as u32);
    let other_offset_opt = match &other.4 {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_t(other_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_t(zone_name, other_time_tuple),
    };
    if let Some((me_offset, other_offset)) = me_offset_opt.zip(other_offset_opt) {
      let me_date_opt = date_time_offset_t(me_time_tuple, me_offset);
      let other_date_opt = date_time_offset_t(other_time_tuple, other_offset);
      if let Some((me_date, other_date)) = me_date_opt.zip(other_date_opt)
        && let Some(nanos) = me_date.sub(other_date).num_nanoseconds()
      {
        return Some(FeelDaysAndTimeDuration::from_n(nanos));
      }
    }
    None
  }
}

impl Add<FeelDaysAndTimeDuration> for FeelTime {
  type Output = FeelTime;
  /// Adds [FeelDaysAndTimeDuration] to [FeelTime].
  fn add(self, rhs: FeelDaysAndTimeDuration) -> Self::Output {
    if rhs.is_negative() {
      return self.sub(rhs.abs());
    }
    let duration_seconds = rhs.get_seconds() + self.2 as usize;
    let carry_minutes = duration_seconds / 60;
    let time_seconds = (duration_seconds % 60) as u8;
    let duration_minutes = rhs.get_minutes() + carry_minutes + self.1 as usize;
    let carry_hours = duration_minutes / 60;
    let time_minutes = (duration_minutes % 60) as u8;
    let duration_hours = rhs.get_hours() + carry_hours + self.0 as usize;
    let time_hours = (duration_hours % 60) as u8;
    FeelTime(time_hours, time_minutes, time_seconds, self.3, self.4)
  }
}

impl Sub<FeelDaysAndTimeDuration> for FeelTime {
  type Output = FeelTime;
  /// Subtracts [FeelDaysAndTimeDuration] from [FeelTime].
  fn sub(self, rhs: FeelDaysAndTimeDuration) -> Self::Output {
    if rhs.is_negative() {
      return self.add(rhs.abs());
    }
    let duration_seconds = rhs.get_seconds();
    let mut carry_minutes = duration_seconds / 60;
    let remainder_seconds = (duration_seconds % 60) as u8;
    let time_seconds = if remainder_seconds <= self.2 {
      self.2 - remainder_seconds
    } else {
      carry_minutes += 1;
      60 - (remainder_seconds - self.2)
    };
    let duration_minutes = rhs.get_minutes() + carry_minutes;
    let mut carry_hours = duration_minutes / 60;
    let remainder_minutes = (duration_minutes % 60) as u8;
    let time_minutes = if remainder_minutes <= self.1 {
      self.1 - remainder_minutes
    } else {
      carry_hours += 1;
      60 - (remainder_minutes - self.1)
    };
    let duration_hours = rhs.get_hours() + carry_hours;
    let remainder_hours = (duration_hours % 60) as u8;
    let time_hours = if remainder_hours <= self.0 {
      self.0 - remainder_hours
    } else {
      24 - (remainder_hours - self.0)
    };
    FeelTime(time_hours, time_minutes, time_seconds, self.3, self.4)
  }
}

impl FeelTime {
  /// Creates UTC time from specified time values.
  pub fn utc(hour: u8, minute: u8, second: u8, nanos: u64) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Utc)
  }

  pub fn utc_opt(hour: u8, minute: u8, second: u8, nanos: u64) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      Some(Self(hour, minute, second, nanos, FeelZone::Utc))
    } else {
      None
    }
  }

  /// Creates local time from specified time values.
  pub fn local(hour: u8, minute: u8, second: u8, nanos: u64) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Local)
  }

  pub fn local_opt(hour: u8, minute: u8, second: u8, nano: u64) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      Some(Self(if hour == 24 { 0 } else { hour }, minute, second, nano, FeelZone::Local))
    } else {
      None
    }
  }

  /// Creates a time from specified time and offset values.
  pub fn offset(hour: u8, minute: u8, second: u8, nanos: u64, offset: i32) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Offset(offset))
  }

  pub fn offset_opt(hour: u8, minute: u8, second: u8, nano: u64, offset: i32) -> Option<Self> {
    if is_valid_time(hour, minute, second)
      && let Ok(zone) = FeelZone::try_from(offset)
    {
      return Some(Self(if hour == 24 { 0 } else { hour }, minute, second, nano, zone));
    }
    None
  }

  pub fn zone_opt(hour: u8, minute: u8, second: u8, nano: u64, zone: FeelZone) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      return Some(Self(if hour == 24 { 0 } else { hour }, minute, second, nano, zone));
    }
    None
  }

  pub fn hour(&self) -> u8 {
    self.0
  }

  pub fn minute(&self) -> u8 {
    self.1
  }

  pub fn second(&self) -> u8 {
    self.2
  }

  pub fn nanos(&self) -> u64 {
    self.3
  }

  pub fn zone(&self) -> &FeelZone {
    &self.4
  }

  pub fn as_tuple(&self) -> (u8, u8, u8, u64, FeelZone) {
    (self.0, self.1, self.2, self.3, self.4.clone())
  }

  pub fn feel_time_offset(&self) -> Option<i32> {
    feel_time_offset(&FeelDateTime::new(FeelDate::today(), self.clone()))
  }

  pub fn feel_time_zone(&self) -> Option<String> {
    feel_time_zone(&FeelDateTime::new(FeelDate::today(), self.clone()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn eq_time(h: u8, m: u8, s: u8, n: u64, z: FeelZone, time: &str) {
    let expected = FeelTime(h, m, s, n, z);
    let actual = FeelTime::from_str(time).expect("should not fail");
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_time_from_str() {
    eq_time(18, 37, 9, 0, FeelZone::Local, "18:37:09");
    eq_time(16, 37, 9, 0, FeelZone::Utc, "16:37:09z");
    eq_time(16, 37, 9, 0, FeelZone::Utc, "16:37:09Z");
    eq_time(16, 37, 9, 0, FeelZone::Zone("Etc/UTC".to_string()), "16:37:09@Etc/UTC");
    eq_time(17, 37, 9, 0, FeelZone::Zone("Europe/London".to_string()), "17:37:09@Europe/London");
    eq_time(18, 37, 9, 0, FeelZone::Zone("Europe/Warsaw".to_string()), "18:37:09@Europe/Warsaw");
    eq_time(18, 37, 9, 0, FeelZone::Zone("Africa/Johannesburg".to_string()), "18:37:09@Africa/Johannesburg");
    eq_time(10, 37, 9, 0, FeelZone::Zone("America/Vancouver".to_string()), "10:37:09@America/Vancouver");
    eq_time(13, 37, 9, 0, FeelZone::Zone("America/New_York".to_string()), "13:37:09@America/New_York");
  }

  #[test]
  fn test_time_from_str_errors() {
    assert_eq!(Err(err_invalid_time_literal("24:37:09")), FeelTime::from_str("24:37:09"));
    assert_eq!(Err(err_invalid_time_literal("18:60:09")), FeelTime::from_str("18:60:09"));
    assert_eq!(Err(err_invalid_time_literal("05:12:60")), FeelTime::from_str("05:12:60"));
  }

  #[test]
  fn test_eq() {
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) == FeelTime(0, 0, 0, 0, FeelZone::Utc)));
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) != FeelTime(0, 0, 0, 0, FeelZone::Local)));
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) != FeelTime(0, 0, 0, 0, FeelZone::Offset(18_000))));
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) != FeelTime(0, 0, 0, 0, FeelZone::Zone("Europe/Warsaw".to_string()))));
  }

  #[test]
  fn test_subtract() {
    let t1 = FeelTime(0, 0, 0, 0, FeelZone::Utc);
    let t2 = FeelTime(0, 0, 0, 0, FeelZone::Utc);
    let d = FeelDaysAndTimeDuration::from_n(0);
    assert_eq!(d, (t1 - t2).unwrap());
    let t1 = FeelTime(0, 0, 59, 0, FeelZone::Utc);
    let t2 = FeelTime(0, 0, 39, 0, FeelZone::Utc);
    let d = FeelDaysAndTimeDuration::from_s(20);
    assert_eq!(d, (t1 - t2).unwrap());
  }
}
