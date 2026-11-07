use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"starts with("foobar", "fo")"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"starts with("foobar", "oob")"#, false);
}

#[test]
fn _0003() {
    te_bool(
        false,
        &scope!(),
        r#"starts with(string: "foobar", match: "fo")"#,
        true,
    );
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"starts with(string: "foobar", match: "oob")"#,
        false,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"starts with(8,"bar")"#,
        r#"[core::starts with] invalid argument type, expected string, actual type is number"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"starts with("foo",7)"#,
        r#"[core::starts with] invalid argument type, expected string, actual type is number"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"starts with(s:"foo",match:"o")"#,
        r#"parameter 'string' not found"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"starts with(string:"foo",m:"o")"#,
        r#"parameter 'match' not found"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"starts with()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"starts with("foo")"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}
