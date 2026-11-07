mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_div_001() {
    eqs!("2.5", num!(20) / num!(8));
}

#[test]
fn test_div_002() {
    let mut x = FeelNumber::new(20, 0);
    x /= FeelNumber::new(8, 0);
    eqs!("2.5", x);
}
