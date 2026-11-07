use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"not(true)"#, false);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"not(false)"#, true);
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        r#"not(1)"#,
        r#"[core::not] invalid argument type, expected boolean, actual type is number"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        r#"not("a")"#,
        r#"[core::not] invalid argument type, expected boolean, actual type is string"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"not(time("22:01:23"))"#,
        r#"[core::not] invalid argument type, expected boolean, actual type is time"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"not(date("2022-02-01"))"#,
        r#"[core::not] invalid argument type, expected boolean, actual type is date"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"not(date("2022-02-01T22:01:23"))"#,
        r#"[core::not] invalid argument type, expected boolean, actual type is Null"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"not(null)"#,
        r#"[core::not] invalid argument type, expected boolean, actual type is Null"#,
    );
}
