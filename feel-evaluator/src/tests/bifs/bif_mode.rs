use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_null(
        false,
        &scope!(),
        "mode()",
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0002() {
    te_null(
        false,
        &scope!(),
        "mode(l:[])",
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        "mode(l:[1,2,3])",
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        "mode([true,false])",
        r#"[core::mode] invalid argument type, expected number, actual type is boolean"#,
    );
}

#[test]
fn _0005() {
    te_be_value(false, &scope!(), "mode(list:[])", "[]");
}

#[test]
fn _0006() {
    te_be_value(false, &scope!(), "mode([])", "[]");
}

#[test]
fn _0007() {
    te_be_value(false, &scope!(), "mode(list:[23])", "[23]");
}

#[test]
fn _0008() {
    te_be_value(false, &scope!(), "mode([23])", "[23]");
}

#[test]
fn _0009() {
    te_be_value(false, &scope!(), "mode(23)", "[23]");
}

#[test]
fn _0010() {
    te_be_value(false, &scope!(), "mode(list:[6,3,9,6,6])", "[6]");
}

#[test]
fn _0011() {
    te_be_value(false, &scope!(), "mode([6,3,9,6,6])", "[6]");
}

#[test]
fn _0012() {
    te_be_value(false, &scope!(), "mode(6,3,9,6,6)", "[6]");
}

#[test]
fn _0013() {
    te_be_value(false, &scope!(), "mode(list:[6,1,9,6,1])", "[1,6]");
}

#[test]
fn _0014() {
    te_be_value(false, &scope!(), "mode([6,1,9,6,1])", "[1,6]");
}

#[test]
fn _0015() {
    te_be_value(false, &scope!(), "mode(6,1,9,6,1)", "[1,6]");
}
