use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"contains("foobar","ob")"#, true);
}

#[test]
fn _0002() {
    te_bool(
        false,
        &scope!(),
        r#"contains(string: "foobar",match: "ob")"#,
        true,
    );
}

#[test]
fn _0003() {
    te_bool(
        false,
        &scope!(),
        r#"contains(match: "ob",string: "foobar")"#,
        true,
    );
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"contains("foobar","of")"#, false);
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"contains()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"contains("foo")"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"contains(string: "foobar", m: "ob")"#,
        r#"parameter 'match' not found"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"contains(s: "foobar", match: "ob")"#,
        r#"parameter 'string' not found"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"contains("foobar", 23)"#,
        r#"[core::contains] invalid argument type, expected string, actual type is number"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"contains(false, "ob")"#,
        r#"[core::contains] invalid argument type, expected string, actual type is boolean"#,
    );
}
