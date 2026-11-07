mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_frac_001() {
    eqs!("0.6789", FeelNumber::new(123456789, 4).frac());
}
