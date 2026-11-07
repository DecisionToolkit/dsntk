use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_string(false, &scope!(), r#"upper case("xyZ ")"#, r#"XYZ "#);
}

#[test]
fn _0002() {
    te_string(false, &scope!(), r#"upper case(string:"xyZ ")"#, r#"XYZ "#);
}

#[test]
fn _0003() {
    te_string(false, &scope!(), r#"upper case("")"#, r#""#);
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        r#"upper case()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"upper case("ABC",4)"#,
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"upper case(date("2021-01-24"))"#,
        r#"[core::upper case] invalid argument type, expected string, actual type is date"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"upper case(s: "ABc")"#,
        r#"parameter 'string' not found"#,
    );
}
