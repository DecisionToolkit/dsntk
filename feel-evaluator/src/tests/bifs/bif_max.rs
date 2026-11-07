use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_null(
        false,
        &scope!(),
        "max()",
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0002() {
    te_null(false, &scope!(), "max([])", r#""#);
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        "max(l:[])",
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        "max(l:[1,2,3])",
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "max([2021])", 2021, 0);
}

#[test]
fn _0006() {
    te_null(false, &scope!(), "max([])", r#""#);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "max(2021)", 2021, 0);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), "max([1,2,3])", 3, 0);
}

#[test]
fn _0009() {
    te_number(false, &scope!(), "max(list: [2,3,1])", 3, 0);
}

#[test]
fn _0010() {
    te_number(false, &scope!(), "max(1,2,3)", 3, 0);
}

#[test]
fn _0011() {
    te_number(
        false,
        &scope!(),
        "max([8,4,2.89,3,8.0001,2.89,5])",
        80001,
        4,
    );
}

#[test]
fn _0012() {
    te_number(false, &scope!(), "max(8,4,2.89,3,8.0001,2.89,5)", 80001, 4);
}

#[test]
fn _0013() {
    te_number(
        false,
        &scope!(),
        "max([2837465.9584,-39408573.456749])",
        28374659584,
        4,
    );
}

#[test]
fn _0014() {
    te_number(
        false,
        &scope!(),
        "max(2837465.9584,-39408573.456749)",
        28374659584,
        4,
    );
}

#[test]
fn _0015() {
    te_string(false, &scope!(), r#"max(["a","b","c"])"#, r#"c"#);
}

#[test]
fn _0016() {
    te_string(false, &scope!(), r#"max("a","b","c")"#, r#"c"#);
}

#[test]
fn _0017() {
    te_string(false, &scope!(), r#"max(["John","Johnny"])"#, r#"Johnny"#);
}

#[test]
fn _0018() {
    te_string(false, &scope!(), r#"max("John","Johnny")"#, r#"Johnny"#);
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"max(1,2,3,"a")"#,
        r#"[core::max] invalid argument type, expected number, actual type is string"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"max("a","b","c",true)"#,
        r#"[core::max] invalid argument type, expected string, actual type is boolean"#,
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"max(true, "a","b","c",1,2)"#,
        r#"[core::max] invalid argument type, expected number, string, actual type is boolean"#,
    );
}
