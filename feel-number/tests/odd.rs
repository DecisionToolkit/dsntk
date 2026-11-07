mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_odd_001() {
    assert!(!num!(-4).odd());
}

#[test]
fn test_odd_002() {
    assert!(num!(-3).odd());
}

#[test]
fn test_odd_003() {
    assert!(!num!(-2).odd());
}

#[test]
fn test_odd_004() {
    assert!(num!(-1).odd());
}

#[test]
fn test_odd_005() {
    assert!(!num!(-0).odd());
}

#[test]
fn test_odd_006() {
    assert!(!num!(0).odd());
}

#[test]
fn test_odd_007() {
    assert!(num!(1).odd());
}

#[test]
fn test_odd_008() {
    assert!(!num!(2).odd());
}

#[test]
fn test_odd_009() {
    assert!(num!(3).odd());
}

#[test]
fn test_odd_010() {
    assert!(!num!(4).odd());
}

#[test]
fn test_odd_011() {
    assert!(!num!(3.1).odd());
}
