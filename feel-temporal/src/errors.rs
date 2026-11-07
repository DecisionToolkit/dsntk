//! Implementation of FEEL temporal errors.

use crate::FeelDate;
use dsntk_common::{DsntkError, ToErrorMessage};
use dsntk_feel_number::FeelNumber;

/// FEEL temporal error.
#[derive(ToErrorMessage)]
struct TemporalError(String);

pub fn err_invalid_date(y: FeelNumber, m: FeelNumber, d: FeelNumber) -> DsntkError {
    TemporalError(format!("invalid date {y}-{m}-{d}")).into()
}

pub fn err_invalid_feel_date(date: FeelDate) -> DsntkError {
    TemporalError(format!(
        "invalid date {}-{}-{}",
        date.year(),
        date.month(),
        date.day()
    ))
    .into()
}

pub fn err_invalid_date_literal(s: &str) -> DsntkError {
    TemporalError(format!("invalid date literal '{s}'")).into()
}

pub fn err_invalid_time_literal(s: &str) -> DsntkError {
    TemporalError(format!("invalid time literal '{s}'")).into()
}

pub fn err_invalid_date_time_literal(s: &str) -> DsntkError {
    TemporalError(format!("invalid date and time literal '{s}'")).into()
}

pub fn err_date_time_conversion_failed(s: &str) -> DsntkError {
    TemporalError(format!(
        "conversion from FEEL date '{s}' to DateTime<FixedOffset> failed, see issue #? for details"
    ))
    .into()
}

pub fn err_invalid_years_and_months_duration_literal(s: &str) -> DsntkError {
    TemporalError(format!("invalid years and months literal '{s}'")).into()
}

pub fn err_invalid_time_zone_offset(offset: i32) -> DsntkError {
    TemporalError(format!("invalid time-zone offset '{offset}'")).into()
}

pub fn err_invalid_date_and_time_duration_literal(literal: String) -> DsntkError {
    TemporalError(format!("invalid date and time duration literal: {literal}")).into()
}
