mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_round_001() {
    eqs!("123.46", num!(123.4567).round(&num!(2)));
}

#[test]
fn test_round_002() {
    eqs!("123.45", num!(123.4547).round(&num!(2)));
}

#[test]
fn test_round_003() {
    eqs!("100", num!(123.4567).round(&num!(-2)));
}

#[test]
fn test_round_004() {
    eqs!("200", num!(163.4567).round(&num!(-2)));
}

#[test]
fn test_round_005() {
    eqs!("0.0", num!(0.0).round(&num!(1)));
}
