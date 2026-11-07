mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_ln_001() {
    assert!(num!(-1).ln().is_none());
}

#[test]
fn test_ln_002() {
    assert!(num!(0).ln().is_none());
}

#[test]
fn test_ln_003() {
    eqs!("0", num!(1).ln().unwrap());
}

#[test]
fn test_ln_004() {
    eqs!("1.386294361119890618834464242916353", num!(4).ln().unwrap());
}

#[test]
fn test_ln_005() {
    eqs!(
        "2.302585092994045684017991454684364",
        num!(10).ln().unwrap()
    );
}
