use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"ends with("foobar", "ar")"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"ends with("foobar", "ba")"#, false);
}

#[test]
fn _0003() {
    te_bool(
        false,
        &scope!(),
        r#"ends with(string: "foobar", match: "ar")"#,
        true,
    );
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"ends with(string: "foobar", match: "ba")"#,
        false,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"ends with(8,"bar")"#,
        r#"[core::ends with] invalid argument type, expected string, actual type is number"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"ends with("foo",7)"#,
        r#"[core::ends with] invalid argument type, expected string, actual type is number"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"ends with(s:"foo",match:"o")"#,
        r#"parameter 'string' not found"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"ends with(string:"foo",m:"o")"#,
        r#"parameter 'match' not found"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"ends with()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"ends with("foo")"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}
