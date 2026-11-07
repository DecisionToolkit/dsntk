mod common;

use dsntk_feel_number::FeelNumber;
use std::cmp::Ordering;

#[test]
fn test_comparison_001() {
    assert!((num!(120000) == num!(120000)));
}

#[test]
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn test_comparison_002() {
    assert!(!(num!(0) > num!(0)));
}

#[test]
fn test_comparison_003() {
    assert!((num!(1234.56) == num!(1234.56)));
}

#[test]
fn test_comparison_004() {
    assert!(!(num!(1234.56) == num!(-1234.56)));
}

#[test]
fn test_comparison_005() {
    assert!((num!(1234.56) > num!(123.456)));
}

#[test]
fn test_comparison_006() {
    assert!((num!(123.456) < num!(1234.56)));
}

#[test]
fn test_comparison_007() {
    assert!((num!(1234.56) <= num!(1234.56)));
}

#[test]
fn test_comparison_008() {
    assert!((num!(1234.56) >= num!(1234.56)));
}

#[test]
fn test_comparison_009() {
    assert!((11_isize > num!(10)));
}

#[test]
fn test_comparison_010() {
    assert!((num!(11) > 10));
}

#[test]
fn test_comparison_011() {
    assert!((-6111..6176).contains(&num!(0)));
}

#[test]
fn test_comparison_012() {
    assert!((0..6176).contains(&num!(6175)));
}

#[test]
fn test_comparison_013() {
    assert!((-3..2).contains(&num!(-3)));
}

#[test]
fn test_comparison_014() {
    assert!(!(-3..2).contains(&num!(-4)));
}

#[test]
fn test_comparison_015() {
    assert!((0..60).contains(&num!(00)));
}

#[test]
fn test_comparison_016() {
    assert!((0..60).contains(&num!(59.999999999)));
}

#[test]
fn test_comparison_017() {
    assert!(!(0..60).contains(&num!(60)));
}

#[test]
fn test_comparison_018() {
    assert!((num!(0) == num!(0)));
}

#[test]
fn test_comparison_019() {
    assert!(!(num!(0) == num!(1)));
}

#[test]
fn test_comparison_020() {
    assert!(!(num!(1) == num!(0)));
}

#[test]
fn test_comparison_021() {
    assert!((num!(0) == 0_isize));
}

#[test]
fn test_comparison_022() {
    assert!((num!(1) == 1_isize));
}

#[test]
fn test_comparison_023() {
    assert!((num!(-1) == -1_isize));
}

#[test]
fn test_comparison_024() {
    assert!((0_isize == num!(0)));
}

#[test]
fn test_comparison_025() {
    assert!((1_isize == num!(1)));
}

#[test]
fn test_comparison_026() {
    assert!((-1_isize == num!(-1)));
}

#[test]
fn test_comparison_027() {
    assert!((num!(0).partial_cmp(&num!(0)).unwrap() == Ordering::Equal));
}

#[test]
fn test_comparison_028() {
    assert!((num!(1).partial_cmp(&num!(0)).unwrap() == Ordering::Greater));
}

#[test]
fn test_comparison_029() {
    assert!((num!(0).partial_cmp(&num!(1)).unwrap() == Ordering::Less));
}

#[test]
fn test_comparison_030() {
    assert!(num!(0).is_zero());
}

#[test]
fn test_comparison_031() {
    assert!(num!(0).is_zero());
}
