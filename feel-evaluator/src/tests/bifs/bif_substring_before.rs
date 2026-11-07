use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_string(
        false,
        &scope!(),
        r#"substring before("foobar","bar")"#,
        "foo",
    );
}

#[test]
fn _0002() {
    te_string(
        false,
        &scope!(),
        r#"substring before(string: "foobar", match: "bar")"#,
        "foo",
    );
}

#[test]
fn _0003() {
    te_string(false, &scope!(), r#"substring before("foobar","xyz")"#, "");
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        r#"substring before()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"substring before("a","b","c")"#,
        r#"expected 2 parameters, actual number of parameters is 3"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"substring before(s: "foobar", match: "bar")"#,
        r#"parameter 'string' not found"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"substring before(string: "foobar", m: "bar")"#,
        r#"parameter 'match' not found"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"substring before(["foobar"], "ob")"#,
        r#"substring before: expected string, actual input type is: list<string>"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"substring before("foobar", ["ob"])"#,
        r#"substring before: expected string, actual match type is: list<string>"#,
    );
}
