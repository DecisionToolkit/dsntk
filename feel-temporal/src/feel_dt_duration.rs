//! FEEL days and time duration.

use crate::defs::*;
use crate::errors::*;
use dsntk_common::DsntkError;
use regex::Regex;
use std::sync::LazyLock;
use std::{fmt, ops};

/// Regular expression pattern for parsing days and time duration.
const REGEX_DAYS_AND_TIME: &str = r"^(?P<sign>-)?P((?P<days>[0-9]+)D)?(T((?P<hours>[0-9]+)H)?((?P<minutes>[0-9]+)M)?((?P<seconds>[0-9]+)(?P<fractional>\.[0-9]*)?S)?)?$";

/// Number of nanoseconds in a day.
const NANOSECONDS_IN_DAY: i64 = 24 * NANOSECONDS_IN_HOUR;
/// Number of nanoseconds in an hour.
const NANOSECONDS_IN_HOUR: i64 = 60 * NANOSECONDS_IN_MINUTE;
/// Number of nanoseconds in a minute.
const NANOSECONDS_IN_MINUTE: i64 = 60 * NANOSECONDS_IN_SECOND;
/// Number of nanoseconds in a second.
const NANOSECONDS_IN_SECOND: i64 = 1_000_000_000;

static RE_DAYS_AND_TIME: LazyLock<Regex> = LazyLock::new(|| Regex::new(REGEX_DAYS_AND_TIME).unwrap());

/// FEEL days and time duration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct FeelDaysAndTimeDuration(i64);

impl FeelDaysAndTimeDuration {
  /// Creates [FeelDaysAndTimeDuration] from nanoseconds.
  pub fn from_n(nanos: i64) -> Self {
    Self(nanos)
  }

  /// Creates [FeelDaysAndTimeDuration] from seconds.
  pub fn from_s(seconds: i64) -> Self {
    Self(seconds * NANOSECONDS_IN_SECOND)
  }

  /// Creates [FeelDaysAndTimeDuration] from seconds and nanoseconds.
  pub fn from_sn(seconds: i64, nanos: i64) -> Self {
    Self(seconds * NANOSECONDS_IN_SECOND + nanos)
  }

  /// Returns the number of days in this duration.
  pub fn get_days(&self) -> usize {
    (self.0.abs() / NANOSECONDS_IN_DAY) as usize
  }

  /// Returns the number of hours in this duration.
  pub fn get_hours(&self) -> usize {
    ((self.0.abs() % NANOSECONDS_IN_DAY) / NANOSECONDS_IN_HOUR) as usize
  }

  /// Returns the number of minutes in this duration.
  pub fn get_minutes(&self) -> usize {
    ((self.0.abs() % NANOSECONDS_IN_DAY % NANOSECONDS_IN_HOUR) / NANOSECONDS_IN_MINUTE) as usize
  }

  /// Returns the number of seconds in this duration.
  pub fn get_seconds(&self) -> usize {
    ((self.0.abs() % NANOSECONDS_IN_DAY % NANOSECONDS_IN_HOUR % NANOSECONDS_IN_MINUTE) / NANOSECONDS_IN_SECOND) as usize
  }

  /// Returns the seconds component of this duration with sign.
  pub fn as_seconds(&self) -> isize {
    (self.0 / NANOSECONDS_IN_SECOND) as isize
  }

  /// Returns the this duration as nanoseconds.
  pub fn as_nanos(&self) -> i64 {
    self.0
  }

  /// Returns absolute value of the duration.
  pub fn abs(&self) -> Self {
    Self(self.0.abs())
  }

  /// Returns `true` when the duration is negative.
  pub fn is_negative(&self) -> bool {
    self.0 < 0
  }
}

impl ops::Add<FeelDaysAndTimeDuration> for FeelDaysAndTimeDuration {
  type Output = Self;
  /// Returns the sum of durations.
  fn add(self, rhs: FeelDaysAndTimeDuration) -> Self {
    Self(self.0 + rhs.0)
  }
}

impl ops::Sub<FeelDaysAndTimeDuration> for FeelDaysAndTimeDuration {
  type Output = Self;
  /// Returns the subtraction of durations.
  fn sub(self, rhs: FeelDaysAndTimeDuration) -> Self {
    Self(self.0 - rhs.0)
  }
}

impl ops::Neg for FeelDaysAndTimeDuration {
  type Output = Self;
  /// Returns the arithmetic negation of this duration.
  fn neg(self) -> Self {
    Self(-self.0)
  }
}

impl fmt::Display for FeelDaysAndTimeDuration {
  /// Converts [FeelDaysAndTimeDuration] into string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let sign = if self.0 < 0 { "-" } else { "" };
    let mut nanoseconds = self.0.abs();
    let day = nanoseconds / NANOSECONDS_IN_DAY;
    nanoseconds -= day * NANOSECONDS_IN_DAY;
    let hour = nanoseconds / NANOSECONDS_IN_HOUR;
    nanoseconds -= hour * NANOSECONDS_IN_HOUR;
    let minute = nanoseconds / NANOSECONDS_IN_MINUTE;
    nanoseconds -= minute * NANOSECONDS_IN_MINUTE;
    let seconds = nanoseconds / NANOSECONDS_IN_SECOND;
    nanoseconds -= seconds * NANOSECONDS_IN_SECOND;
    let nanoseconds_str = nanos_to_string(nanoseconds as u64);
    match (day > 0, hour > 0, minute > 0, seconds > 0, nanoseconds > 0) {
      (false, false, false, false, false) => write!(f, "PT0S"),
      (false, false, false, true, false) => write!(f, "{sign}PT{seconds}S"),
      (false, false, true, false, false) => write!(f, "{sign}PT{minute}M"),
      (false, false, true, true, false) => write!(f, "{sign}PT{minute}M{seconds}S"),
      (false, true, false, false, false) => write!(f, "{sign}PT{hour}H"),
      (false, true, false, true, false) => write!(f, "{sign}PT{hour}H{seconds}S"),
      (false, true, true, false, false) => write!(f, "{sign}PT{hour}H{minute}M"),
      (false, true, true, true, false) => write!(f, "{sign}PT{hour}H{minute}M{seconds}S"),
      (true, false, false, false, false) => write!(f, "{sign}P{day}D"),
      (true, false, false, true, false) => write!(f, "{sign}P{day}DT{seconds}S"),
      (true, false, true, false, false) => write!(f, "{sign}P{day}DT{minute}M"),
      (true, false, true, true, false) => write!(f, "{sign}P{day}DT{minute}M{seconds}S"),
      (true, true, false, false, false) => write!(f, "{sign}P{day}DT{hour}H"),
      (true, true, false, true, false) => write!(f, "{sign}P{day}DT{hour}H{seconds}S"),
      (true, true, true, false, false) => write!(f, "{sign}P{day}DT{hour}H{minute}M"),
      (true, true, true, true, false) => write!(f, "{sign}P{day}DT{hour}H{minute}M{seconds}S"),
      (false, false, false, false, true) => write!(f, "{sign}PT0.{nanoseconds_str}S"),
      (false, false, false, true, true) => write!(f, "{sign}PT{seconds}.{nanoseconds_str}S"),
      (false, false, true, false, true) => write!(f, "{sign}PT{minute}M0.{nanoseconds_str}S"),
      (false, false, true, true, true) => write!(f, "{sign}PT{minute}M{seconds}.{nanoseconds_str}S"),
      (false, true, false, false, true) => write!(f, "{sign}PT{hour}H0.{nanoseconds_str}S"),
      (false, true, false, true, true) => write!(f, "{sign}PT{hour}H{seconds}.{nanoseconds_str}S"),
      (false, true, true, false, true) => write!(f, "{sign}PT{hour}H{minute}M0.{nanoseconds_str}S"),
      (false, true, true, true, true) => write!(f, "{sign}PT{hour}H{minute}M{seconds}.{nanoseconds_str}S"),
      (true, false, false, false, true) => write!(f, "{sign}P{day}DT0.{nanoseconds_str}S"),
      (true, false, false, true, true) => write!(f, "{sign}P{day}DT{seconds}.{nanoseconds_str}S"),
      (true, false, true, false, true) => write!(f, "{sign}P{day}DT{minute}M0.{nanoseconds_str}S"),
      (true, false, true, true, true) => write!(f, "{sign}P{day}DT{minute}M{seconds}.{nanoseconds_str}S"),
      (true, true, false, false, true) => write!(f, "{sign}P{day}DT{hour}H0.{nanoseconds_str}S",),
      (true, true, false, true, true) => write!(f, "{sign}P{day}DT{hour}H{seconds}.{nanoseconds_str}S"),
      (true, true, true, false, true) => write!(f, "{sign}P{day}DT{hour}H{minute}M0.{nanoseconds_str}S"),
      (true, true, true, true, true) => write!(f, "{sign}P{day}DT{hour}H{minute}M{seconds}.{nanoseconds_str}S"),
    }
  }
}

impl TryFrom<&str> for FeelDaysAndTimeDuration {
  type Error = DsntkError;
  /// Converts a text form of the days and time duration into [FeelDaysAndTimeDuration] struct.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_DAYS_AND_TIME.captures(value) {
      let mut is_valid = false;
      let mut nanoseconds = 0_i64;
      if let Some(days_match) = captures.name("days")
        && let Ok(days_u64) = days_match.as_str().parse::<u64>()
        && let Ok(days) = <u64 as TryInto<i64>>::try_into(days_u64)
        && let Some(a) = days.checked_mul(NANOSECONDS_IN_DAY)
      {
        nanoseconds += a;
        is_valid = true;
      }
      if let Some(hours_match) = captures.name("hours")
        && let Ok(hours) = hours_match.as_str().parse::<u64>()
      {
        nanoseconds += (hours as i64) * NANOSECONDS_IN_HOUR;
        is_valid = true;
      }
      if let Some(minutes_match) = captures.name("minutes")
        && let Ok(minutes) = minutes_match.as_str().parse::<u64>()
      {
        nanoseconds += (minutes as i64) * NANOSECONDS_IN_MINUTE;
        is_valid = true;
      }
      if let Some(seconds_match) = captures.name("seconds")
        && let Ok(seconds) = seconds_match.as_str().parse::<u64>()
      {
        nanoseconds += (seconds as i64) * NANOSECONDS_IN_SECOND;
        is_valid = true;
      }
      if let Some(fractional_match) = captures.name("fractional")
        && let Ok(fractional) = fractional_match.as_str().parse::<f64>()
      {
        nanoseconds += (fractional * NANOSECONDS_IN_SECOND as f64).trunc() as i64;
        is_valid = true;
      }
      if captures.name("sign").is_some() {
        nanoseconds = -nanoseconds;
      }
      if is_valid {
        return Ok(FeelDaysAndTimeDuration(nanoseconds));
      }
    }
    Err(err_invalid_date_and_time_duration_literal(value.to_string()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cmp::Ordering;

  /// Utility function for testing days and time durations equality.
  fn equals(neg: bool, sec: i64, nano: i64, text: &str) {
    let expected = if neg {
      -FeelDaysAndTimeDuration::from_sn(sec, nano)
    } else {
      FeelDaysAndTimeDuration::from_sn(sec, nano)
    };
    let actual = FeelDaysAndTimeDuration::try_from(text).unwrap();
    assert_eq!(expected, actual);
  }

  /// Utility function for testing invalid days and time durations.
  fn invalid(text: &str) {
    let actual = FeelDaysAndTimeDuration::try_from(text);
    assert!(actual.is_err());
  }

  /// Utility function for testing equality of textual forms of days and time durations.
  fn equals_str(expected: &str, neg: bool, sec: i64, nano: i64) {
    let actual: String = if neg {
      (-FeelDaysAndTimeDuration::from_sn(sec, nano)).to_string()
    } else {
      FeelDaysAndTimeDuration::from_sn(sec, nano).to_string()
    };
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_parsing_from_string_should_pass() {
    equals(false, 86_400, 0, "P1D");
    equals(true, 86_400, 0, "-P1D");
    equals(false, 97200, 0, "P1DT3H");
    equals(true, 97200, 0, "-P1DT3H");
    equals(false, 97980, 0, "P1DT3H13M");
    equals(true, 97980, 0, "-P1DT3H13M");
    equals(false, 98023, 0, "P1DT3H13M43S");
    equals(true, 98023, 0, "-P1DT3H13M43S");
    equals(false, 10800, 0, "PT3H");
    equals(true, 10800, 0, "-PT3H");
    equals(false, 12960, 0, "PT3H36M");
    equals(true, 12960, 0, "-PT3H36M");
    equals(false, 12982, 0, "PT3H36M22S");
    equals(true, 12982, 0, "-PT3H36M22S");
    equals(false, 3540, 0, "PT59M");
    equals(true, 3540, 0, "-PT59M");
    equals(false, 3558, 0, "PT59M18S");
    equals(true, 3558, 0, "-PT59M18S");
    equals(false, 31, 0, "PT31S");
    equals(true, 31, 0, "-PT31S");
    equals(false, 87180, 0, "P1DT13M");
    equals(true, 87180, 0, "-P1DT13M");
    equals(false, 97243, 0, "P1DT3H43S");
    equals(true, 97243, 0, "-P1DT3H43S");
    equals(false, 0, 999_000_000, "PT0.999S");
    equals(false, 0, 0, "PT0.S");
    equals(false, 58, 123_123_123, "PT58.123123123S");
    equals(false, 999, 999_999_999, "PT999.999999999999S");
  }

  #[test]
  fn test_parsing_from_string_should_fail() {
    invalid("P");
    invalid("-P");
    invalid("PT");
    invalid("-PT");
    invalid("T");
    invalid("-T");
    invalid("P11");
    invalid("-P11");
    invalid("PT1S1M");
    invalid("-PT1S1M");
    invalid("PT2M3H12S");
    invalid("-PT2M3H12S");
  }

  #[test]
  fn test_converting_to_string_should_pass() {
    equals_str("PT0.999S", false, 0, 999_000_000);
    equals_str("PT0S", false, 0, 0);
    equals_str("PT1S", false, 1, 0);
    equals_str("-PT1S", true, 1, 0);
    equals_str("PT1.123S", false, 1, 123_000_000);
    equals_str("-PT1.123S", true, 1, 123_000_000);
    equals_str("PT59S", false, 59, 0);
    equals_str("-PT59S", true, 59, 0);
    equals_str("PT1M", false, 60, 0);
    equals_str("-PT1M", true, 60, 0);
    equals_str("PT1M0.987987987S", false, 60, 987987987);
    equals_str("-PT1M0.987987987S", true, 60, 987987987);
    equals_str("PT1M1S", false, 61, 0);
    equals_str("-PT1M1S", true, 61, 0);
    equals_str("PT1M1.584S", false, 61, 584_000_000);
    equals_str("-PT1M1.584S", true, 61, 584_000_000);
    equals_str("PT59M59S", false, 3_599, 0);
    equals_str("-PT59M59S", true, 3_599, 0);
    equals_str("PT59M59.999999999S", false, 3_599, 999_999_999);
    equals_str("-PT59M59.999999999S", true, 3_599, 999_999_999);
    equals_str("PT1H", false, 3_600, 0);
    equals_str("-PT1H", true, 3_600, 0);
    equals_str("PT1H10S", false, 3_610, 0);
    equals_str("-PT1H10S", true, 3_610, 0);
    equals_str("PT1H0.3459S", false, 3_600, 345_900_000);
    equals_str("-PT1H0.3459S", true, 3_600, 345_900_000);
    equals_str("PT1H59.11S", false, 3_659, 110_000_000);
    equals_str("-PT1H59.11S", true, 3_659, 110_000_000);
    equals_str("PT1H1M", false, 3_660, 0);
    equals_str("-PT1H1M", true, 3_660, 0);
    equals_str("PT1H1M0.123S", false, 3_660, 123_000_000);
    equals_str("-PT1H1M0.123S", true, 3_660, 123_000_000);
    equals_str("PT1H1M1S", false, 3_661, 0);
    equals_str("-PT1H1M1S", true, 3_661, 0);
    equals_str("PT1H1M1.123S", false, 3_661, 123_000_000);
    equals_str("-PT1H1M1.123S", true, 3_661, 123_000_000);
    equals_str("PT23H59M59S", false, 86_399, 0);
    equals_str("-PT23H59M59S", true, 86_399, 0);
    equals_str("PT23H59M59.123S", false, 86_399, 123_000_000);
    equals_str("-PT23H59M59.123S", true, 86_399, 123_000_000);
    equals_str("P1D", false, 86_400, 0);
    equals_str("-P1D", true, 86_400, 0);
    equals_str("P1DT0.123S", false, 86_400, 123_000_000);
    equals_str("-P1DT0.123S", true, 86_400, 123_000_000);
    equals_str("P1DT1S", false, 86_401, 0);
    equals_str("-P1DT1S", true, 86_401, 0);
    equals_str("P1DT1.123S", false, 86_401, 123_000_000);
    equals_str("-P1DT1.123S", true, 86_401, 123_000_000);
    equals_str("P1DT59S", false, 86_459, 0);
    equals_str("-P1DT59S", true, 86_459, 0);
    equals_str("P1DT59.123S", false, 86_459, 123_000_000);
    equals_str("-P1DT59.123S", true, 86_459, 123_000_000);
    equals_str("P1DT1M", false, 86_460, 0);
    equals_str("-P1DT1M", true, 86_460, 0);
    equals_str("P1DT1M0.123S", false, 86_460, 123_000_000);
    equals_str("-P1DT1M0.123S", true, 86_460, 123_000_000);
    equals_str("P1DT1M1S", false, 86_461, 0);
    equals_str("-P1DT1M1S", true, 86_461, 0);
    equals_str("P1DT1M1.123S", false, 86_461, 123_000_000);
    equals_str("-P1DT1M1.123S", true, 86_461, 123_000_000);
    equals_str("P1DT59M59S", false, 89_999, 0);
    equals_str("-P1DT59M59S", true, 89_999, 0);
    equals_str("P1DT59M59.123S", false, 89_999, 123_000_000);
    equals_str("-P1DT59M59.123S", true, 89_999, 123_000_000);
    equals_str("P1DT1H", false, 90_000, 0);
    equals_str("-P1DT1H", true, 90_000, 0);
    equals_str("P1DT1H0.123S", false, 90_000, 123_000_000);
    equals_str("-P1DT1H0.123S", true, 90_000, 123_000_000);
    equals_str("P1DT1H59S", false, 90_059, 0);
    equals_str("-P1DT1H59S", true, 90_059, 0);
    equals_str("P1DT1H59.123S", false, 90_059, 123_000_000);
    equals_str("-P1DT1H59.123S", true, 90_059, 123_000_000);
    equals_str("P1DT1H1M", false, 90_060, 0);
    equals_str("-P1DT1H1M", true, 90_060, 0);
    equals_str("P1DT1H1M0.123S", false, 90_060, 123_000_000);
    equals_str("-P1DT1H1M0.123S", true, 90_060, 123_000_000);
    equals_str("P1DT1H1M1S", false, 90_061, 0);
    equals_str("-P1DT1H1M1S", true, 90_061, 0);
    equals_str("P1DT1H1M1.123S", false, 90_061, 123_000_000);
    equals_str("-P1DT1H1M1.123S", true, 90_061, 123_000_000);
    equals_str("P1DT23H59M59S", false, 172_799, 0);
    equals_str("-P1DT23H59M59S", true, 172_799, 0);
    equals_str("P1DT23H59M59.123S", false, 172_799, 123_000_000);
    equals_str("-P1DT23H59M59.123S", true, 172_799, 123_000_000);
  }

  #[test]
  fn test_eq_should_pass() {
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::from_sn(0, 0).partial_cmp(&FeelDaysAndTimeDuration::from_sn(0, 0))
    );
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::from_sn(0, 10).partial_cmp(&FeelDaysAndTimeDuration::from_sn(0, 10))
    );
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::from_sn(0, 999_999_999).partial_cmp(&FeelDaysAndTimeDuration::from_sn(0, 999_999_999))
    );
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::from_sn(86_400, 999_999_999).partial_cmp(&FeelDaysAndTimeDuration::from_sn(86_400, 999_999_999))
    );
    assert_eq!(FeelDaysAndTimeDuration::from_n(0), FeelDaysAndTimeDuration::from_n(0));
    assert_eq!(FeelDaysAndTimeDuration::from_n(0), FeelDaysAndTimeDuration::from_n(-0));
    assert_eq!(FeelDaysAndTimeDuration::from_sn(0, 10), FeelDaysAndTimeDuration::from_sn(0, 10));
    assert_eq!(FeelDaysAndTimeDuration::from_sn(0, 999_999_999), FeelDaysAndTimeDuration::from_sn(0, 999_999_999));
    assert_eq!(FeelDaysAndTimeDuration::from_sn(86_400, 999_999_999), FeelDaysAndTimeDuration::from_sn(86_400, 999_999_999));
  }

  #[test]
  fn test_lt_should_pass() {
    assert_eq!(Some(Ordering::Less), FeelDaysAndTimeDuration::from_s(10).partial_cmp(&FeelDaysAndTimeDuration::from_s(11)));
    assert_eq!(
      Some(Ordering::Less),
      FeelDaysAndTimeDuration::from_sn(10, 1).partial_cmp(&FeelDaysAndTimeDuration::from_sn(10, 2))
    );
    assert!(FeelDaysAndTimeDuration::from_s(10) < FeelDaysAndTimeDuration::from_s(11));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 1) < FeelDaysAndTimeDuration::from_sn(10, 2));
    assert!(FeelDaysAndTimeDuration::from_s(11) >= FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 2) >= FeelDaysAndTimeDuration::from_sn(10, 1));
  }

  #[test]
  fn test_le_should_pass() {
    assert!(FeelDaysAndTimeDuration::from_s(10) <= FeelDaysAndTimeDuration::from_s(11));
    assert!(FeelDaysAndTimeDuration::from_s(10) <= FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_s(10) <= FeelDaysAndTimeDuration::from_sn(10, 1));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 1) <= FeelDaysAndTimeDuration::from_sn(10, 1));
    assert!(FeelDaysAndTimeDuration::from_s(11) > FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 2) > FeelDaysAndTimeDuration::from_sn(10, 1));
  }

  #[test]
  fn test_gt_should_pass() {
    assert_eq!(
      Some(Ordering::Greater),
      FeelDaysAndTimeDuration::from_s(11).partial_cmp(&FeelDaysAndTimeDuration::from_s(10))
    );
    assert_eq!(
      Some(Ordering::Greater),
      FeelDaysAndTimeDuration::from_sn(10, 1).partial_cmp(&FeelDaysAndTimeDuration::from_s(10))
    );
    assert!(FeelDaysAndTimeDuration::from_s(11) > FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 1) > FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_s(10) <= FeelDaysAndTimeDuration::from_s(11));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 1) <= FeelDaysAndTimeDuration::from_sn(10, 2));
  }

  #[test]
  fn test_ge_should_pass() {
    assert!(FeelDaysAndTimeDuration::from_s(11) >= FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_s(10) >= FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 1) >= FeelDaysAndTimeDuration::from_s(10));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 1) >= FeelDaysAndTimeDuration::from_sn(10, 1));
    assert!(FeelDaysAndTimeDuration::from_s(10) < FeelDaysAndTimeDuration::from_s(11));
    assert!(FeelDaysAndTimeDuration::from_sn(10, 1) < FeelDaysAndTimeDuration::from_sn(10, 2));
  }

  #[test]
  fn test_add_should_pass() {
    let a = FeelDaysAndTimeDuration::from_s(11);
    let b = FeelDaysAndTimeDuration::from_s(83);
    let c = FeelDaysAndTimeDuration::from_s(94);
    assert_eq!(c, a + b);
    let a = FeelDaysAndTimeDuration::from_sn(11, 2_837);
    let b = FeelDaysAndTimeDuration::from_sn(83, 23);
    let c = FeelDaysAndTimeDuration::from_sn(94, 2_860);
    assert_eq!(c, a + b);
    let a = FeelDaysAndTimeDuration::from_sn(1, 999_999_999);
    let b = FeelDaysAndTimeDuration::from_sn(1, 2);
    let c = FeelDaysAndTimeDuration::from_sn(3, 1);
    assert_eq!(c, a + b);
  }

  #[test]
  fn test_sub_should_pass() {
    let a = FeelDaysAndTimeDuration::from_s(12);
    let b = FeelDaysAndTimeDuration::from_s(2);
    let c = FeelDaysAndTimeDuration::from_s(10);
    assert_eq!(c, a - b);
    let a = FeelDaysAndTimeDuration::from_sn(99, 999_999_999);
    let b = FeelDaysAndTimeDuration::from_sn(77, 888_888_888);
    let c = FeelDaysAndTimeDuration::from_sn(22, 111_111_111);
    assert_eq!(c, a - b);
    let a = FeelDaysAndTimeDuration::from_sn(1, 1);
    let b = FeelDaysAndTimeDuration::from_n(2);
    let c = FeelDaysAndTimeDuration::from_n(999_999_999);
    assert_eq!(c, a - b);
  }

  #[test]
  fn test_duration_as_seconds() {
    let dt_duration_a = FeelDaysAndTimeDuration::try_from("PT12S").unwrap();
    assert_eq!(12, dt_duration_a.as_seconds());
    let dt_duration_b = FeelDaysAndTimeDuration::try_from("-PT12S").unwrap();
    assert_eq!(-12, dt_duration_b.as_seconds());
    let dt_duration_c = FeelDaysAndTimeDuration::try_from("PT2H").unwrap();
    assert_eq!(7_200, dt_duration_c.as_seconds());
    let dt_duration_d = FeelDaysAndTimeDuration::try_from("-PT2H").unwrap();
    assert_eq!(-7_200, dt_duration_d.as_seconds());
  }

  #[test]
  fn test_duration_properties() {
    let dt_duration_a = FeelDaysAndTimeDuration::try_from("P3DT5H18M36S").unwrap();
    assert_eq!(3, dt_duration_a.get_days());
    assert_eq!(5, dt_duration_a.get_hours());
    assert_eq!(18, dt_duration_a.get_minutes());
    assert_eq!(36, dt_duration_a.get_seconds());
    assert_eq!(278_316, dt_duration_a.as_seconds());
    let dt_duration_b = FeelDaysAndTimeDuration::try_from("-P3DT5H18M36S").unwrap();
    assert_eq!(3, dt_duration_b.get_days());
    assert_eq!(5, dt_duration_b.get_hours());
    assert_eq!(18, dt_duration_b.get_minutes());
    assert_eq!(36, dt_duration_b.get_seconds());
    assert_eq!(-278_316, dt_duration_b.as_seconds());
  }

  #[test]
  fn test_abs_should_pass() {
    let dt_duration_a = FeelDaysAndTimeDuration::try_from("PT12S").unwrap();
    assert_eq!("PT12S", dt_duration_a.abs().to_string());
    let dt_duration_b = FeelDaysAndTimeDuration::try_from("-PT12S").unwrap();
    assert_eq!("PT12S", dt_duration_b.abs().to_string());
    let dt_duration_c = FeelDaysAndTimeDuration::try_from("P3DT5H18M36S").unwrap();
    assert_eq!("P3DT5H18M36S", dt_duration_c.abs().to_string());
    let dt_duration_d = FeelDaysAndTimeDuration::try_from("-P3DT5H18M36S").unwrap();
    assert_eq!("P3DT5H18M36S", dt_duration_d.abs().to_string());
  }

  #[test]
  #[should_panic]
  fn test_large_days() {
    //TODO Maybe it is possible NOT TO convert the number of days into nanoseconds?
    FeelDaysAndTimeDuration::try_from("P3000000000000000000D").unwrap();
  }
}
