use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#"sum(101.5)"#, 1015, 1);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), r#"sum(1,2,3,4,5,6,7,8,9,10)"#, 55, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#"sum([1,2,3,4,5,6,7,8,9,10])"#, 55, 0);
}

#[test]
fn _0004() {
    te_number(
        false,
        &scope!(),
        r#"sum(list: [1,2,3,4,5,6,7,8,9,10])"#,
        55,
        0,
    );
}

#[test]
fn _0005() {
    te_null(false, &scope!(), r#"sum([])"#, r#""#);
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"sum()"#,
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"sum(1,"a")"#,
        r#"[core::sum] invalid argument type, expected number, actual type is string"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"sum(true,1,"a")"#,
        r#"[core::sum] invalid argument type, expected number, actual type is boolean"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"sum(l: [1,2,3])"#,
        r#"parameter 'list' not found"#,
    );
}
