use chrono::{DateTime, FixedOffset};
use dsntk_feel_temporal::FeelDateTime;

#[test]
fn _0001() {
    let date_time: FeelDateTime = "2023-01-10T24:00:00".try_into().unwrap();
    assert_eq!("2023-01-11T00:00:00", date_time.to_string());
}

#[test]
fn _0002() {
    let date_time: dsntk_common::Result<FeelDateTime> = "999999999-01-10T24:00:00".try_into();
    assert_eq!(
        "<TemporalError> invalid date and time literal '999999999-01-10T24:00:00'",
        date_time.err().unwrap().to_string()
    );
}

#[test]
fn _0003() {
    let date_time: dsntk_common::Result<FeelDateTime> = "2023-02-09T24:01:00".try_into();
    assert_eq!(
        "<TemporalError> invalid date and time literal '2023-02-09T24:01:00'",
        date_time.err().unwrap().to_string()
    );
}

#[test]
fn _0004() {
    let date_time: dsntk_common::Result<FeelDateTime> =
        "2023-02-09T24:01:00@Europe/Sofa".try_into();
    assert_eq!(
        "<TemporalError> invalid date and time literal '2023-02-09T24:01:00@Europe/Sofa'",
        date_time.err().unwrap().to_string()
    );
}

#[test]
fn _0005() {
    let feel_date_time: FeelDateTime = "262142-12-31T23:59:59.9Z".try_into().unwrap();
    let date_time: DateTime<FixedOffset> = feel_date_time.try_into().unwrap();
    assert_eq!("+262142-12-31 23:59:59.900 +00:00", date_time.to_string());
}

#[test]
fn _0006() {
    let feel_date_time: FeelDateTime = "-262143-01-01T00:00:00.1Z".try_into().unwrap();
    let date_time: DateTime<FixedOffset> = feel_date_time.try_into().unwrap();
    assert_eq!("-262143-01-01 00:00:00.100 +00:00", date_time.to_string());
}

#[test]
fn _0007() {
    let date_time: dsntk_common::Result<FeelDateTime> = "99999999-01-01T00:00:00".try_into();
    assert_eq!(
        "<TemporalError> invalid date and time literal '99999999-01-01T00:00:00'",
        date_time.err().unwrap().to_string()
    );
}

#[test]
fn _0008() {
    let date_time: dsntk_common::Result<FeelDateTime> = "-99999999-12-31T00:00:00".try_into();
    assert_eq!(
        "<TemporalError> invalid date and time literal '-99999999-12-31T00:00:00'",
        date_time.err().unwrap().to_string()
    );
}
