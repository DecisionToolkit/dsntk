mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_sqrt_001() {
    assert!(num!(-1).sqrt().is_none());
}

#[test]
fn test_sqrt_002() {
    eqs!("0", num!(0).sqrt().unwrap());
}

#[test]
fn test_sqrt_003() {
    eqs!("1", num!(1).sqrt().unwrap());
}

#[test]
fn test_sqrt_004() {
    eqs!(
        "1.414213562373095048801688724209698",
        num!(2).sqrt().unwrap()
    );
}

#[test]
fn test_sqrt_005() {
    assert!(FeelNumber::infinite().sqrt().is_none());
}
