//! Implementation of FEEL years and months duration.

use crate::errors::*;
use dsntk_common::DsntkError;
use regex::Regex;
use std::sync::LazyLock;
use std::{fmt, ops};

/// Regular expression pattern for parsing years and months duration.
const REGEX_YEARS_AND_MONTHS: &str = r#"^(?P<sign>-)?P((?P<years>[0-9]+)Y)?((?P<months>[0-9]+)M)?$"#;

/// Number of months in a year.
const MONTHS_IN_YEAR: i64 = 12;

/// Regular expression for parsing years and months duration.
static RE_YEARS_AND_MONTHS: LazyLock<Regex> = LazyLock::new(|| Regex::new(REGEX_YEARS_AND_MONTHS).unwrap());

/// Years and months duration in FEEL.
///
/// Holds the number of months in the duration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct FeelYearsAndMonthsDuration(i64);

impl FeelYearsAndMonthsDuration {
  /// Created a new years and months duration from given number of `years` and `months`.
  pub fn from_ym(years: i64, months: i64) -> Self {
    Self(years * MONTHS_IN_YEAR + months)
  }

  /// Created a new years and months duration from given number of `months`.
  pub fn from_m(months: i64) -> Self {
    Self(months)
  }

  /// Returns the number of years in this duration.
  pub fn years(&self) -> i64 {
    self.0 / MONTHS_IN_YEAR
  }

  /// Returns the number of months in this duration.
  pub fn months(&self) -> i64 {
    self.0 % MONTHS_IN_YEAR
  }

  /// Returns the total number of months of this duration.
  pub fn as_months(&self) -> i64 {
    self.0
  }

  /// Returns absolute value of the duration.
  pub fn abs(&self) -> Self {
    FeelYearsAndMonthsDuration(self.0.abs())
  }

  /// Returns `true` when duration is negative.
  pub fn is_negative(&self) -> bool {
    self.0 < 0
  }
}

impl ops::Add<FeelYearsAndMonthsDuration> for FeelYearsAndMonthsDuration {
  type Output = Self;
  /// Returns the sum of durations.
  fn add(self, rhs: FeelYearsAndMonthsDuration) -> Self {
    Self(self.0 + rhs.0)
  }
}

impl ops::Sub<FeelYearsAndMonthsDuration> for FeelYearsAndMonthsDuration {
  type Output = Self;
  /// Returns the subtraction of durations.
  fn sub(self, rhs: FeelYearsAndMonthsDuration) -> Self {
    Self(self.0 - rhs.0)
  }
}

impl ops::Neg for FeelYearsAndMonthsDuration {
  type Output = Self;
  /// Returns the arithmetic negation of this duration.
  fn neg(self) -> Self {
    Self(-self.0)
  }
}

impl fmt::Display for FeelYearsAndMonthsDuration {
  /// Converts [FeelYearsAndMonthsDuration] into string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let sign = if self.0 < 0 { "-" } else { "" };
    let mut month = self.0.abs();
    let year = month / MONTHS_IN_YEAR;
    month -= year * MONTHS_IN_YEAR;
    match (year > 0, month > 0) {
      (false, false) => write!(f, "P0M"),
      (false, true) => write!(f, "{sign}P{month}M"),
      (true, false) => write!(f, "{sign}P{year}Y"),
      (true, true) => write!(f, "{sign}P{year}Y{month}M"),
    }
  }
}

impl TryFrom<&str> for FeelYearsAndMonthsDuration {
  type Error = DsntkError;
  /// Converts a text into [FeelYearsAndMonthsDuration].
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_YEARS_AND_MONTHS.captures(value) {
      let mut total_months = 0_i64;
      let mut contains_years = false;
      let mut contains_months = false;
      let mut valid_years = false;
      let mut valid_months = false;
      if let Some(years_match) = captures.name("years") {
        contains_years = true;
        if let Ok(years) = years_match.as_str().parse::<i64>()
          && let Some(months) = years.checked_mul(MONTHS_IN_YEAR)
          && let Some(total) = total_months.checked_add(months)
        {
          total_months = total;
          valid_years = true;
        }
      }
      if let Some(months_match) = captures.name("months") {
        contains_months = true;
        if let Ok(months) = months_match.as_str().parse::<i64>()
          && let Some(total) = total_months.checked_add(months)
        {
          total_months = total;
          valid_months = true;
        }
      }
      if (contains_years && valid_years) || (contains_months && valid_months) {
        if captures.name("sign").is_some() {
          total_months = -total_months;
        }
        return Ok(FeelYearsAndMonthsDuration(total_months));
      }
    }
    Err(err_invalid_years_and_months_duration_literal(value))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Utility function for testing years and months durations equality.
  fn equals(months: i64, text: &str) {
    let expected = FeelYearsAndMonthsDuration(months);
    let actual = FeelYearsAndMonthsDuration::try_from(text).unwrap();
    assert_eq!(expected, actual);
  }

  /// Utility function for testing invalid years and months durations.
  fn invalid(text: &str) {
    let actual = FeelYearsAndMonthsDuration::try_from(text);
    assert!(actual.is_err());
  }

  /// Utility function for testing equality of textual forms of years and months durations.
  fn equals_str(expected: &str, years: i64, months: i64) {
    let actual: String = FeelYearsAndMonthsDuration(years * MONTHS_IN_YEAR + months).to_string();
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_parsing_from_string_should_pass() {
    equals(1, "P1M");
    equals(-1, "-P1M");
    equals(12, "P1Y");
    equals(-12, "-P1Y");
    equals(15, "P1Y3M");
    equals(-15, "-P1Y3M");
    equals(999999999 * MONTHS_IN_YEAR, "P999999999Y");
    equals(-999999999 * MONTHS_IN_YEAR, "-P999999999Y");
  }

  #[test]
  fn test_parsing_from_string_should_fail() {
    invalid("P");
    invalid("-P");
    invalid("K1Y1M");
    invalid("-K1Y1M");
    invalid("P1M1Y");
    invalid("-P1M1Y");
    invalid("P10");
    invalid("-P10");
    invalid("PY");
    invalid("-PY");
    invalid("PM");
    invalid("-PM");
    invalid("P1Y3MT1D");
    invalid("-P1Y3MT1D");
  }

  #[test]
  fn test_converting_to_string_should_pass() {
    equals_str("P0M", 0, 0);
    equals_str("P1M", 0, 1);
    equals_str("-P1M", 0, -1);
    equals_str("P1Y", 1, 0);
    equals_str("-P1Y", -1, 0);
    equals_str("P2Y3M", 2, 3);
    equals_str("-P2Y3M", -2, -3);
  }

  #[test]
  fn test_eq_should_pass() {
    let a = FeelYearsAndMonthsDuration::from_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 0);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::from_ym(2, 3);
    let b = FeelYearsAndMonthsDuration::from_ym(2, 3);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::from_ym(-3, 4);
    let b = FeelYearsAndMonthsDuration::from_ym(-3, 4);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::from_m(0);
    let b = FeelYearsAndMonthsDuration::from_m(0);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::from_m(-0);
    let b = FeelYearsAndMonthsDuration::from_m(0);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::from_m(-15);
    let b = FeelYearsAndMonthsDuration::from_m(-15);
    assert!((a == b));
  }

  #[test]
  fn test_ne_should_pass() {
    let a = FeelYearsAndMonthsDuration::from_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 1);
    assert!((a != b));
    let a = FeelYearsAndMonthsDuration::from_m(1);
    let b = FeelYearsAndMonthsDuration::from_m(0);
    assert!((a != b));
  }

  #[test]
  fn test_lt_should_pass() {
    let a = FeelYearsAndMonthsDuration::from_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 1);
    assert!(a < b);
    let a = FeelYearsAndMonthsDuration::from_m(4);
    let b = FeelYearsAndMonthsDuration::from_m(5);
    assert!(a < b);
  }

  #[test]
  fn test_le_should_pass() {
    let a = FeelYearsAndMonthsDuration::from_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 1);
    assert!(a <= b);
    let a = FeelYearsAndMonthsDuration::from_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 1);
    assert!(a <= b);
    let a = FeelYearsAndMonthsDuration::from_m(12);
    let b = FeelYearsAndMonthsDuration::from_m(18);
    assert!(a <= b);
    let a = FeelYearsAndMonthsDuration::from_m(16);
    let b = FeelYearsAndMonthsDuration::from_m(16);
    assert!(a <= b);
  }

  #[test]
  fn test_gt_should_pass() {
    let a = FeelYearsAndMonthsDuration::from_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 0);
    assert!(a > b);
    let a = FeelYearsAndMonthsDuration::from_m(5);
    let b = FeelYearsAndMonthsDuration::from_m(4);
    assert!(a > b);
  }

  #[test]
  fn test_ge_should_pass() {
    let a = FeelYearsAndMonthsDuration::from_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 0);
    assert!(a >= b);
    let a = FeelYearsAndMonthsDuration::from_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::from_ym(0, 1);
    assert!(a >= b);
    let a = FeelYearsAndMonthsDuration::from_m(18);
    let b = FeelYearsAndMonthsDuration::from_m(12);
    assert!(a >= b);
    let a = FeelYearsAndMonthsDuration::from_m(16);
    let b = FeelYearsAndMonthsDuration::from_m(16);
    assert!(a >= b);
  }

  #[test]
  fn test_abs_should_pass() {
    let ym_duration = FeelYearsAndMonthsDuration::try_from("P2Y3M").unwrap();
    assert_eq!("P2Y3M", ym_duration.abs().to_string());
    let duration = FeelYearsAndMonthsDuration::try_from("-P2Y3M").unwrap();
    assert_eq!("P2Y3M", duration.abs().to_string());
  }

  #[test]
  fn test_properties() {
    let ym_duration = FeelYearsAndMonthsDuration::try_from("P2Y3M").unwrap();
    assert_eq!(2, ym_duration.years());
    assert_eq!(3, ym_duration.months());
    assert_eq!(27, ym_duration.as_months());
  }
}
