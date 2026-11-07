use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), "any(true)", true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), "any(false)", false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), "any(true,true,true)", true);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), "any(false,false,false)", false);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), "any(false,true,false,false)", true);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), "any(false,true,false,true)", true);
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), "any([true])", true);
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), "any(list: [true])", true);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), "any([false])", false);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), "any([true,true,true])", true);
}

#[test]
fn _0011() {
    te_bool(false, &scope!(), "any([false,false,false])", false);
}

#[test]
fn _0012() {
    te_bool(false, &scope!(), "any([false,true,false,false])", true);
}

#[test]
fn _0013() {
    te_bool(false, &scope!(), "any([false,true,false,true])", true);
}

#[test]
fn _0014() {
    te_null(false, &scope!(), "any([false,null,true])", "");
}

#[test]
fn _0015() {
    te_bool(false, &scope!(), "any([])", false);
}

#[test]
fn _0016() {
    te_null(false, &scope!(), "any(1)", "");
}

#[test]
fn _0017() {
    te_null(false, &scope!(), "any(1,2,3)", "");
}

#[test]
fn _0018() {
    te_null(false, &scope!(), "any([123,false])", "");
}

#[test]
fn _0019() {
    te_null(false, &scope!(), "any([true,8,false])", "");
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        "any()",
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        "any(l: [true])",
        r#"parameter 'list' not found"#,
    );
}
