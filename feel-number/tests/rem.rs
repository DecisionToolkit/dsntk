mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_rem_001() {
    eqs!("2", FeelNumber::new(12, 0) % FeelNumber::new(5, 0));
}

#[test]
fn test_rem_002() {
    eqs!("3", FeelNumber::new(-12, 0) % FeelNumber::new(5, 0));
}

#[test]
fn test_rem_003() {
    eqs!("-3", FeelNumber::new(12, 0) % FeelNumber::new(-5, 0));
}

#[test]
fn test_rem_004() {
    eqs!("-2", FeelNumber::new(-12, 0) % FeelNumber::new(-5, 0));
}

#[test]
fn test_rem_005() {
    eqs!("1.1", FeelNumber::new(101, 1) % FeelNumber::new(45, 1));
}

#[test]
fn test_rem_006() {
    eqs!("3.4", FeelNumber::new(-101, 1) % FeelNumber::new(45, 1));
}

#[test]
fn test_rem_007() {
    eqs!("-3.4", FeelNumber::new(101, 1) % FeelNumber::new(-45, 1));
}

#[test]
fn test_rem_008() {
    eqs!("-1.1", FeelNumber::new(-101, 1) % FeelNumber::new(-45, 1));
}

#[test]
fn test_rem_009() {
    let mut x = FeelNumber::new(101, 1);
    x %= FeelNumber::new(-45, 1);
    eqs!("-3.4", x);
}
