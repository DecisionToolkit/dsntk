mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_is_positive_001() {
    assert!(!num!(-1.23).is_positive());
}

#[test]
fn test_is_positive_002() {
    assert!(!num!(-1).is_positive());
}

#[test]
fn test_is_positive_003() {
    assert!(!num!(-0).is_positive());
}

#[test]
fn test_is_positive_004() {
    assert!(!num!(0).is_positive());
}

#[test]
fn test_is_positive_005() {
    assert!(num!(1).is_positive());
}

#[test]
fn test_is_positive_006() {
    assert!(num!(1.23).is_positive());
}
