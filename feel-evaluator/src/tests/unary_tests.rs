use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    valid_unary_tests(false, &scope!(), "-");
}

#[test]
fn _0002() {
    valid_unary_tests(false, &scope!(), "1");
}

#[test]
fn _0003() {
    valid_unary_tests(false, &scope!(), "1,2");
}

#[test]
fn _0004() {
    valid_unary_tests(false, &scope!(), "1,2,3");
}

#[test]
fn _0005() {
    valid_unary_tests(false, &scope!(), "[]");
}

#[test]
fn _0006() {
    valid_unary_tests(false, &scope!(), "[1]");
}

#[test]
fn _0007() {
    valid_unary_tests(false, &scope!(), "[1,2]");
}

#[test]
fn _0008() {
    valid_unary_tests(false, &scope!(), "[1,2,3]");
}

#[test]
fn _0009() {
    valid_unary_tests(false, &scope!(), "[1..2]");
}

#[test]
fn _0010() {
    valid_unary_tests(false, &scope!(), "not(1,2,3)");
}

#[test]
fn _0011() {
    valid_unary_tests(false, &scope!(), "<1");
}

#[test]
fn _0012() {
    valid_unary_tests(false, &scope!(), "<=1");
}

#[test]
fn _0013() {
    valid_unary_tests(false, &scope!(), ">1");
}

#[test]
fn _0014() {
    valid_unary_tests(false, &scope!(), ">=1");
}

#[test]
fn _0015() {
    valid_unary_tests(false, &scope!(), "=1");
}

#[test]
fn _0016() {
    valid_unary_tests(false, &scope!(), "!=1");
}
