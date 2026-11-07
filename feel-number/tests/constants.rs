mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_constants_001() {
    eqs!("0", FeelNumber::zero());
}

#[test]
fn test_constants_002() {
    eqs!("1", FeelNumber::one());
}

#[test]
fn test_constants_003() {
    eqs!("2", FeelNumber::two());
}

#[test]
fn test_constants_004() {
    eqs!("1000000000", FeelNumber::billion());
}
