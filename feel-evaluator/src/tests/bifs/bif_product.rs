use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#"product([2,3,4])"#, 24, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), r#"product(2,3,4,5,6)"#, 720, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#"product(list: [2,3,4])"#, 24, 0);
}

#[test]
fn _0004() {
    te_null(false, &scope!(), r#"product([])"#, r#""#);
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"product(true)"#,
        r#"[core::product] invalid argument type, expected number, actual type is boolean"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"product(l: [1,2,3])"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"product()"#,
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}
