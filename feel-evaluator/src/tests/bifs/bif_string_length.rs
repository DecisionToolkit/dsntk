use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#"string length("engos")"#, 5, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), r#"string length(string: "engos")"#, 5, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#"string length("\u0009")"#, 1, 0);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), r#"string length("\\u0009")"#, 6, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), r#"string length("\U000009")"#, 1, 0);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), r#"string length("\uD83D\uDC0E")"#, 1, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), r#"string length("🐎")"#, 1, 0);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), r#"string length("🐎😀")"#, 2, 0);
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"string length()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"string length("abc","def")"#,
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"string length(s: "engos")"#,
        r#"parameter 'string' not found"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"string length(10)"#,
        r#"string length: expected string as an argument"#,
    );
}
