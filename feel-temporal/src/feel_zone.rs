//! Implementation of FEEL timezone.

use crate::defs::*;
use crate::errors::*;
use dsntk_common::DsntkError;
use regex::Captures;
use std::fmt;
use std::ops::{Div, Rem};

/// FEEL time zone.
#[derive(Debug, Clone)]
pub enum FeelZone {
  /// UTC time zone.
  Utc,
  /// Local time zone.
  Local,
  /// Time zone defined as an offset from UTC in seconds.
  /// The *recoverable timezone* of a date will always be a duration between '+12:00' and '-11:59'.
  /// +12:00 == +43200 seconds
  /// -11:59 == -43140 seconds
  Offset(i32),
  /// Time zone defined as a value from IANA database.
  Zone(String),
}

impl fmt::Display for FeelZone {
  /// Converts [FeelZone] into its text representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FeelZone::Utc => write!(f, "Z"),
      FeelZone::Local => write!(f, ""),
      FeelZone::Offset(offset) => {
        let hours = offset / 3_600;
        let reminder_seconds = offset.unsigned_abs().rem(3_600);
        let minutes = reminder_seconds.div(60);
        let seconds = reminder_seconds.rem(60);
        if seconds > 0 {
          write!(f, "{hours:+03}:{minutes:02}:{seconds:02}")
        } else {
          write!(f, "{hours:+03}:{minutes:02}")
        }
      }
      FeelZone::Zone(zone) => write!(f, "@{zone}"),
    }
  }
}

impl PartialEq for FeelZone {
  /// Returns `true` when two time zones are considered equal.
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
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
      (FeelZone::Offset(_), FeelZone::Offset(_)) => true,
      (FeelZone::Zone(zone_name1), FeelZone::Zone(zone_name2)) => zone_name1 == zone_name2,
      _ => false,
    }
  }
}

impl TryFrom<i32> for FeelZone {
  type Error = DsntkError;
  /// Creates [FeelZone] based on the offset from UTC given in seconds.
  fn try_from(offset: i32) -> Result<Self, Self::Error> {
    if (-43140..=43200).contains(&offset) {
      if offset != 0 { Ok(Self::Offset(offset)) } else { Ok(Self::Utc) }
    } else {
      Err(err_invalid_time_zone_offset(offset))
    }
  }
}

impl FeelZone {
  /// Creates [FeelZone] from timezone captures taken from regular expression.
  pub fn from_captures(captures: &Captures) -> Option<Self> {
    if captures.name("zulu").is_some() {
      return Some(FeelZone::Utc);
    }
    if let Some(sign_match) = captures.name("offSign")
      && let Some(hours_match) = captures.name("offHours")
      && let Ok(hours) = hours_match.as_str().parse::<i32>()
      && let Some(minutes_match) = captures.name("offMinutes")
      && let Ok(minutes) = minutes_match.as_str().parse::<i32>()
    {
      let mut offset = 3600 * hours + 60 * minutes;
      if let Some(seconds_match) = captures.name("offSeconds")
        && let Ok(seconds) = seconds_match.as_str().parse::<i32>()
      {
        offset += seconds;
      }
      if offset > 50_400 {
        return None;
      }
      if sign_match.as_str() == "-" {
        offset = -offset;
      }
      return Some(if offset != 0 { Self::Offset(offset) } else { Self::Utc });
    }
    if let Some(zone_match) = captures.name("zone") {
      return if zone_match.as_str().parse::<chrono_tz::Tz>().is_ok() {
        Some(FeelZone::Zone(zone_match.as_str().to_string()))
      } else {
        None
      };
    }
    Some(FeelZone::Local)
  }

  /// Returns `true` when this [FeelZone] has defined offset (is not local).
  pub fn has_offset(&self) -> bool {
    *self != FeelZone::Local
  }
}

#[cfg(test)]
mod tests {
  use crate::defs::*;
  use crate::feel_zone::FeelZone;

  macro_rules! assert_zone {
    ($zone:expr, $time:expr) => {
      assert_eq!($zone, FeelZone::from_captures(&RE_TIME.captures($time).unwrap()));
    };
  }

  #[test]
  fn test_format_offset() {
    assert_eq!("+05:00", FeelZone::try_from(18_000).unwrap().to_string());
    assert_eq!("+05:00", FeelZone::Offset(18_000).to_string());
    assert_eq!("-05:00", FeelZone::try_from(-18_000).unwrap().to_string());
    assert_eq!("-05:00", FeelZone::Offset(-18_000).to_string());
    assert_eq!("+05:00:01", FeelZone::try_from(18_001).unwrap().to_string());
    assert_eq!("+05:00:01", FeelZone::Offset(18_001).to_string());
    assert_eq!("-05:00:01", FeelZone::try_from(-18_001).unwrap().to_string());
    assert_eq!("-05:00:01", FeelZone::Offset(-18_001).to_string());
    assert_eq!("-11:59", FeelZone::try_from(-43_140).unwrap().to_string());
    assert_eq!("+12:00", FeelZone::try_from(43200).unwrap().to_string());
    assert!(FeelZone::try_from(-43_141).is_err());
    assert!(FeelZone::try_from(43201).is_err());
  }

  #[test]
  fn test_format_utc() {
    assert_eq!("Z", FeelZone::try_from(0).unwrap().to_string());
    assert_eq!("Z", FeelZone::Utc.to_string());
  }

  #[test]
  fn test_format_local() {
    assert_eq!("", FeelZone::Local.to_string());
  }

  #[test]
  fn test_from_captures() {
    assert_zone!(Some(FeelZone::Local), "00:00:00");
    assert_zone!(Some(FeelZone::Utc), "00:00:00Z");
    assert_zone!(Some(FeelZone::Utc), "00:00:00z");
    assert_zone!(Some(FeelZone::Utc), "00:00:00+00:00");
    assert_zone!(Some(FeelZone::Utc), "00:00:00-00:00");
    assert_zone!(Some(FeelZone::Utc), "00:00:00-00:00");
    assert_zone!(Some(FeelZone::Offset(18_000)), "00:00:00+05:00");
    assert_zone!(Some(FeelZone::Offset(18_001)), "00:00:00+05:00:01");
    assert_zone!(Some(FeelZone::Offset(18_060)), "00:00:00+05:01:00");
    assert_zone!(Some(FeelZone::Offset(-18_000)), "00:00:00-05:00");
    assert_zone!(Some(FeelZone::Offset(-18_001)), "00:00:00-05:00:01");
    assert_zone!(Some(FeelZone::Offset(-18_060)), "00:00:00-05:01:00");
    assert_zone!(Some(FeelZone::Offset(50_400)), "00:00:00+14:00");
    assert_zone!(Some(FeelZone::Offset(-50_400)), "00:00:00-14:00");
    assert_zone!(None, "00:00:00+14:01");
    assert_zone!(None, "00:00:00+14:00:01");
    assert_zone!(None, "00:00:00-14:01");
    assert_zone!(None, "00:00:00-14:00:01");
    assert_zone!(Some(FeelZone::Zone("Europe/Warsaw".to_string())), "00:00:00@Europe/Warsaw");
    assert_zone!(None, "00:00:00@abc/xyz");
  }

  #[test]
  fn test_eq() {
    assert!((FeelZone::Local == FeelZone::Local));
    assert!((FeelZone::Local != FeelZone::Utc));
    assert!((FeelZone::Local != FeelZone::Offset(18_400)));
    assert!((FeelZone::Local != FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Utc == FeelZone::Utc));
    assert!((FeelZone::Utc != FeelZone::Offset(18_400)));
    assert!((FeelZone::Utc != FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Offset(1) == FeelZone::Offset(1)));
    assert!((FeelZone::Offset(1) != FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Offset(2) != FeelZone::Zone("Europe/Sofa".to_string())));
    assert!((FeelZone::Offset(1) == FeelZone::Offset(2)));
    assert!((FeelZone::Zone("Europe/Warsaw".to_string()) == FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Zone("Europe/Warsaw".to_string()) != FeelZone::Zone("Australia/Sydney".to_string())));
  }

  #[test]
  fn test_debug() {
    assert_eq!(r#"Local"#, format!("{:?}", FeelZone::Local));
    assert_eq!(r#"Utc"#, format!("{:?}", FeelZone::Utc));
    assert_eq!(r#"Offset(18000)"#, format!("{:?}", FeelZone::Offset(18_000)));
    assert_eq!(r#"Zone("Europe/Warsaw")"#, format!("{:?}", FeelZone::Zone("Europe/Warsaw".to_string())));
  }
}
