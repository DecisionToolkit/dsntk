mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_is_integer_001() {
    assert!(num!(41).is_integer());
}

#[test]
fn test_is_integer_002() {
    assert!(!num!(4.1).is_integer());
}
