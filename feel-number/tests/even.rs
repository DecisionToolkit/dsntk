mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_even_001() {
    assert!(num!(-4).even());
}

#[test]
fn test_even_002() {
    assert!(!num!(-3).even());
}

#[test]
fn test_even_003() {
    assert!(num!(-2).even());
}

#[test]
fn test_even_004() {
    assert!(!num!(-1).even());
}

#[test]
fn test_even_005() {
    assert!(num!(-0).even());
}

#[test]
fn test_even_006() {
    assert!(num!(0).even());
}

#[test]
fn test_even_007() {
    assert!(!num!(1).even());
}

#[test]
fn test_even_008() {
    assert!(num!(2).even());
}

#[test]
fn test_even_009() {
    assert!(!num!(3).even());
}

#[test]
fn test_even_010() {
    assert!(num!(4).even());
}

#[test]
fn test_even_011() {
    assert!(num!(0.0).even());
}

#[test]
fn test_even_012() {
    assert!(num!(2.000).even());
}

#[test]
fn test_even_013() {
    assert!(num!(-2.000).even());
}

#[test]
fn test_even_014() {
    assert!(!FeelNumber::new(41, 1).even());
}

#[test]
fn test_even_015() {
    assert!(!FeelNumber::new(41, 78).even());
}
