mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_mul_001() {
    eqs!("12", num!(1.2) * num!(10));
}

#[test]
fn test_mul_002() {
    let mut x = FeelNumber::new(12, 1);
    x *= FeelNumber::new(10, 0);
    eqs!("12", x);
}

#[test]
fn test_mul_003() {
    eqs!("66000", num!(330000) * num!(0.2));
}
