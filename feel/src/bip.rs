//! # Built-in properties

const BIP_END: &str = "end";
const BIP_END_INCLUDED: &str = "end included";
const BIP_START: &str = "start";
const BIP_START_INCLUDED: &str = "start included";
const BIP_YEAR: &str = "year";
const BIP_YEARS: &str = "years";
const BIP_MONTH: &str = "month";
const BIP_MONTHS: &str = "months";
const BIP_DAY: &str = "day";
const BIP_DAYS: &str = "days";
const BIP_WEEKDAY: &str = "weekday";
const BIP_HOUR: &str = "hour";
const BIP_HOURS: &str = "hours";
const BIP_MINUTE: &str = "minute";
const BIP_MINUTES: &str = "minutes";
const BIP_SECOND: &str = "second";
const BIP_SECONDS: &str = "seconds";
const BIP_TIME_OFFSET: &str = "time offset";
const BIP_TIMEZONE: &str = "timezone";

/// Built-in property enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bip {
  End,
  EndIncluded,
  Start,
  StartIncluded,
  Year,
  Years,
  Month,
  Months,
  Day,
  Days,
  Weekday,
  Hour,
  Hours,
  Minute,
  Minutes,
  Second,
  Seconds,
  TimeOffset,
  Timezone,
  Unknown(String),
}

impl Bip {
  /// Converts a string into corresponding variant of the [Bip] enumeration.
  pub fn new(property: impl AsRef<str>) -> Self {
    match property.as_ref() {
      BIP_END => Bip::End,
      BIP_END_INCLUDED => Self::EndIncluded,
      BIP_START => Self::Start,
      BIP_START_INCLUDED => Self::StartIncluded,
      BIP_YEAR => Self::Year,
      BIP_YEARS => Self::Years,
      BIP_MONTH => Self::Month,
      BIP_MONTHS => Self::Months,
      BIP_DAY => Self::Day,
      BIP_DAYS => Self::Days,
      BIP_WEEKDAY => Self::Weekday,
      BIP_HOUR => Self::Hour,
      BIP_HOURS => Self::Hours,
      BIP_MINUTE => Self::Minute,
      BIP_MINUTES => Self::Minutes,
      BIP_SECOND => Self::Second,
      BIP_SECONDS => Self::Seconds,
      BIP_TIME_OFFSET => Self::TimeOffset,
      BIP_TIMEZONE => Self::Timezone,
      other => Self::Unknown(other.to_string()),
    }
  }
}

impl std::fmt::Display for Bip {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Bip::End => BIP_END,
        Bip::EndIncluded => BIP_END_INCLUDED,
        Bip::Start => BIP_START,
        Bip::StartIncluded => BIP_START_INCLUDED,
        Bip::Year => BIP_YEAR,
        Bip::Years => BIP_YEARS,
        Bip::Month => BIP_MONTH,
        Bip::Months => BIP_MONTHS,
        Bip::Day => BIP_DAY,
        Bip::Days => BIP_DAYS,
        Bip::Weekday => BIP_WEEKDAY,
        Bip::Hour => BIP_HOUR,
        Bip::Hours => BIP_HOURS,
        Bip::Minute => BIP_MINUTE,
        Bip::Minutes => BIP_MINUTES,
        Bip::Second => BIP_SECOND,
        Bip::Seconds => BIP_SECONDS,
        Bip::TimeOffset => BIP_TIME_OFFSET,
        Bip::Timezone => BIP_TIMEZONE,
        Bip::Unknown(property) => property,
      }
    )
  }
}

/// Returns `true` when the specified name is a built-in property name.
pub fn is_built_in_property_name(name: impl AsRef<str>) -> bool {
  matches!(
    name.as_ref(),
    BIP_END
      | BIP_END_INCLUDED
      | BIP_START
      | BIP_START_INCLUDED
      | BIP_YEAR
      | BIP_YEARS
      | BIP_MONTH
      | BIP_MONTHS
      | BIP_DAY
      | BIP_DAYS
      | BIP_WEEKDAY
      | BIP_HOUR
      | BIP_HOURS
      | BIP_MINUTE
      | BIP_MINUTES
      | BIP_SECOND
      | BIP_SECONDS
      | BIP_TIME_OFFSET
      | BIP_TIMEZONE
  )
}
