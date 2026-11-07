use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#"count([])"#, 0, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), r#"count(["A"])"#, 1, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#"count(list: ["A"])"#, 1, 0);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), r#"count(["A","B"])"#, 2, 0);
}

#[test]
fn _0005() {
    te_number(
        false,
        &scope!(),
        r#"count(["A",1,true,date("2021-01-24"),null])"#,
        5,
        0,
    );
}

#[test]
fn _0006() {
    // non list value os coerced into a list with one number
    te_number(false, &scope!(), r#"count(1)"#, 1, 0);
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"count()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"count([1,2,3],4)"#,
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"count(l: [1,2,3])"#,
        r#"parameter 'list' not found"#,
    );
}
