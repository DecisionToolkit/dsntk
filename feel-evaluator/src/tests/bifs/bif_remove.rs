use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_null(
        false,
        &scope!(),
        "remove([1,2,3,4,5],0)",
        r#"probably index is out of range"#,
    );
}

#[test]
fn _0002() {
    te_null(
        false,
        &scope!(),
        "remove([1,2,3,4,5],6)",
        r#"probably index is out of range"#,
    );
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        "remove([1,2,3,4,5],-6)",
        r#"probably index is out of range"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        "remove([1,2,3,4,5])",
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        "remove(6)",
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        "remove([1,2,3,4,5],true)",
        r#"probably index is out of range"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        "remove(l:[1,2,3,4,5],position:1)",
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        "remove(list:[1,2,3,4,5],p:1)",
        r#"parameter 'position' not found"#,
    );
}

#[test]
fn _0009() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],1)", "[2,3,4,5]");
}

#[test]
fn _0010() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],2)", "[1,3,4,5]");
}

#[test]
fn _0011() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],3)", "[1,2,4,5]");
}

#[test]
fn _0012() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],4)", "[1,2,3,5]");
}

#[test]
fn _0013() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],5)", "[1,2,3,4]");
}

#[test]
fn _0014() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],-5)", "[2,3,4,5]");
}

#[test]
fn _0015() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],-4)", "[1,3,4,5]");
}

#[test]
fn _0016() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],-3)", "[1,2,4,5]");
}

#[test]
fn _0017() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],-2)", "[1,2,3,5]");
}

#[test]
fn _0018() {
    te_be_value(false, &scope!(), "remove([1,2,3,4,5],-1)", "[1,2,3,4]");
}

#[test]
fn _0019() {
    te_be_value(
        false,
        &scope!(),
        "remove(list:[1,2,3,4,5],position:1)",
        "[2,3,4,5]",
    );
}
