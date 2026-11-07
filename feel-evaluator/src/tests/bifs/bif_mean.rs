use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#"mean([1,2,3])"#, 2, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), r#"mean(1,2,3,4,5,6)"#, 35, 1);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#"mean(list: [1,2,3,4])"#, 25, 1);
}

#[test]
fn _0004() {
    te_null(false, &scope!(), r#"mean([])"#, r#""#);
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"mean(true)"#,
        r#"[core::mean] invalid argument type, expected number, actual type is boolean"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"mean(l: [1,2,3])"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"mean()"#,
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}
