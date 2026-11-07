mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_pow_001() {
    eqs!("1", num!(0).pow(&num!(0)).unwrap());
}

#[test]
fn test_pow_002() {
    eqs!(
        "41959.857373594361860953310707468",
        num!(12.2384283).pow(&num!(4.25)).unwrap()
    );
}

#[test]
fn test_pow_003() {
    assert!(num!(9999).pow(&num!(9999)).is_none());
}
