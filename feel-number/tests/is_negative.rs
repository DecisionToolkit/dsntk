mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_is_negative_001() {
    assert!(num!(-1.23).is_negative());
}

#[test]
fn test_is_negative_002() {
    assert!(num!(-1).is_negative());
}

#[test]
fn test_is_negative_003() {
    assert!(!num!(-0).is_negative());
}

#[test]
fn test_is_negative_004() {
    assert!(!num!(0).is_negative());
}

#[test]
fn test_is_negative_005() {
    assert!(!num!(1).is_negative());
}

#[test]
fn test_is_negative_006() {
    assert!(!num!(1.23).is_negative());
}
