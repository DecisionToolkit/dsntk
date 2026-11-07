//! Test cases for `FEEL` time type.

use crate::{FeelTime, FeelZone};

#[test]
fn _0001() {
    let time_a = FeelTime::offset_opt(10, 11, 12, 0, 7).unwrap();
    let time_b =
        FeelTime::zone_opt(10, 11, 12, 0, FeelZone::Zone("Europe/Sofa".to_string())).unwrap();
    assert!(!(time_a == time_b));
}

#[test]
fn _0002() {
    let time_a = FeelTime::offset_opt(10, 11, 12, 0, 7).unwrap();
    let time_b =
        FeelTime::zone_opt(10, 11, 12, 0, FeelZone::Zone("Europe/Sofa".to_string())).unwrap();
    assert!((time_a - time_b).is_none());
}

#[test]
fn _0003() {
    assert!(FeelTime::local_opt(26, 11, 12, 0).is_none());
    assert!(FeelTime::local_opt(24, 1, 0, 0).is_none());
}

#[test]
fn _0004() {
    assert!(FeelTime::offset_opt(26, 11, 12, 0, 0).is_none());
    assert!(FeelTime::offset_opt(24, 1, 0, 0, 0).is_none());
}

#[test]
fn _0005() {
    assert!(FeelTime::zone_opt(26, 11, 12, 0, FeelZone::Utc).is_none());
    assert!(FeelTime::zone_opt(24, 1, 0, 0, FeelZone::Local).is_none());
}
