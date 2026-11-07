mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_abs_001() {
    eqs!("0", num!(0).abs());
}

#[test]
fn test_abs_002() {
    eqs!("0", num!(-0).abs());
}

#[test]
fn test_abs_003() {
    eqs!("1", num!(1).abs());
}

#[test]
fn test_abs_004() {
    eqs!("1", num!(-1).abs());
}

#[test]
fn test_abs_005() {
    eqs!("0.123456", num!(0.123456).abs());
}

#[test]
fn test_abs_006() {
    eqs!("0.123456", num!(-0.123456).abs());
}
