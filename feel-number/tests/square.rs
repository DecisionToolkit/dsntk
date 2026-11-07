mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_square_001() {
    eqs!("4", num!(2).square().unwrap());
}

#[test]
fn test_square_002() {
    eqs!("25", num!(5).square().unwrap());
}

#[test]
fn test_square_003() {
    assert!("1E+6144".parse::<FeelNumber>().unwrap().square().is_none());
}
