mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_neg_001() {
    eqs!("-1.23", -num!(1.23));
}

#[test]
fn test_neg_002() {
    eqs!("1.23", -num!(-1.23));
}
