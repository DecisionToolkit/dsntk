use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_string(false, &scope!(), r#"substring("foobar",3)"#, "obar");
}

#[test]
fn _0002() {
    te_string(false, &scope!(), r#"substring("foobar",-4)"#, "obar");
}

#[test]
fn _0003() {
    te_string(
        false,
        &scope!(),
        r#"substring(string: "foobar", start position: 3)"#,
        "obar",
    );
}

#[test]
fn _0004() {
    te_string(false, &scope!(), r#"substring("foobar",3,3)"#, "oba");
}

#[test]
fn _0005() {
    te_string(
        false,
        &scope!(),
        r#"substring(string: "foobar", start position: 3, length: 3)"#,
        "oba",
    );
}

#[test]
fn _0006() {
    te_string(false, &scope!(), r#"substring("foobar",-2,1)"#, "a");
}

#[test]
fn _0007() {
    te_string(false, &scope!(), r#"substring("foob r",-2,1)"#, " ");
}

#[test]
fn _0008() {
    te_string(false, &scope!(), r#"substring("\U01F40Eab",2)"#, "ab");
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"substring()"#,
        r#"expected 2,3 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"substring("abc",1,2,3)"#,
        r#"expected 2,3 parameters, actual number of parameters is 4"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"substring(s: "foobar", start position: 3, length: 3)"#,
        r#"parameter 'string' not found"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"substring(string: "foobar", start: 3, length: 3)"#,
        r#"parameter 'start position' not found"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"substring("foobar", 9223372036854775808, 3)"#,
        r#"substring: invalid start position value: 9223372036854775808"#,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"substring("foobar", -9223372036854775809, 3)"#,
        r#"substring: invalid start position value: -9223372036854775809"#,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"substring("foobar", 2, 0)"#,
        r#"substring: length is less than 1"#,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"substring("foobar", 2, 922337203685477580893847)"#,
        r#"substring: invalid length value: 922337203685477580893847"#,
    );
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", 9)"#,
        r#"substring: position is out of range, len = 8, position = 9"#,
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", -9)"#,
        r#"substring: position is out of range, len = 8, position = -9"#,
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", 0)"#,
        r#"substring: start position must not be zero"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", 0, 2)"#,
        r#"substring: start position must not be zero"#,
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", 9, 2)"#,
        r#"sublist: invalid range, len = 8, start position = 9, end position = 11"#,
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", -9, 2)"#,
        r#"sublist: invalid range, len = 8, start position = 0, end position = 2"#,
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", 9, "2")"#,
        r#"sublist: expected number, actual length type is string"#,
    );
}

#[test]
fn _0024() {
    te_null(
        false,
        &scope!(),
        r#"substring("homeless", "9", 2)"#,
        r#"sublist: expected number, actual start position type is string"#,
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"substring(["homeless"], 9, 2)"#,
        r#"sublist: expected string, actual value type is list<string>"#,
    );
}
