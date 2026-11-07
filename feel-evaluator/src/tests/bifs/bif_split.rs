use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"split("John Doe","\\s")"#,
        r#"["John","Doe"]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"split(string: "John Doe", delimiter: "\\s")"#,
        r#"["John","Doe"]"#,
    );
}

#[test]
fn _0003() {
    te_be_value(
        false,
        &scope!(),
        r#"split(delimiter: "\\s", string: "John Doe")"#,
        r#"["John","Doe"]"#,
    );
}

#[test]
fn _0004() {
    te_be_value(
        false,
        &scope!(),
        r#"split("a;b;c;;",";")"#,
        r#"["a","b","c","",""]"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"split()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"split("abc")"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"split("abc","a","b")"#,
        r#"expected 2 parameters, actual number of parameters is 3"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"split(s: "John Doe", delimiter: "\\s")"#,
        r#"parameter 'string' not found"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"split(string: "John Doe", d: "\\s")"#,
        r#"parameter 'delimiter' not found"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"split(10,"\\s")"#,
        r#"split: input must be a string"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"split("John Doe",10)"#,
        r#"split: delimiter must be a string"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"split("John Doe","[a-z")"#,
        r#"split: invalid delimiter"#,
    );
}
