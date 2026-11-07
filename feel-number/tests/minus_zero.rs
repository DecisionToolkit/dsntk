mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_minus_zero_001() {
    eqs!("0", FeelNumber::new(-0, 0).to_string());
}
