use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let scope = scope!();
    te_bool(false, &scope, "all(true)", true);
}

#[test]
fn _0002() {
    let scope = scope!();
    te_bool(false, &scope, "all(true,true)", true);
}

#[test]
fn _0003() {
    let scope = scope!();
    te_bool(false, &scope, "all(true,true,true)", true);
}

#[test]
fn _0004() {
    let scope = scope!();
    te_bool(false, &scope, "all(false)", false);
}

#[test]
fn _0005() {
    let scope = scope!();
    te_bool(false, &scope, "all(true,false)", false);
}

#[test]
fn _0006() {
    let scope = scope!();
    te_bool(false, &scope, "all(false,true,true)", false);
}

#[test]
fn _0007() {
    let scope = scope!();
    te_null(false, &scope, "all(null,true,true)", r#""#);
}

#[test]
fn _0008() {
    let scope = scope!();
    te_bool(false, &scope, "all(false,null,true)", false);
}

#[test]
fn _0009() {
    let scope = scope!();
    te_bool(false, &scope, "all([])", true);
}

#[test]
fn _0010() {
    let scope = scope!();
    te_bool(false, &scope, "all(list: [])", true);
}

#[test]
fn _0011() {
    let scope = scope!();
    te_bool(false, &scope, "all([true])", true);
}

#[test]
fn _0012() {
    let scope = scope!();
    te_bool(false, &scope, "all(list: [true])", true);
}

#[test]
fn _0013() {
    let scope = scope!();
    te_bool(false, &scope, "all([true,true])", true);
}

#[test]
fn _0014() {
    let scope = scope!();
    te_bool(false, &scope, "all([true,true,true])", true);
}

#[test]
fn _0015() {
    let scope = scope!();
    te_bool(false, &scope, "all([false])", false);
}

#[test]
fn _0016() {
    let scope = scope!();
    te_bool(false, &scope, "all([true,false])", false);
}

#[test]
fn _0017() {
    let scope = scope!();
    te_bool(false, &scope, "all([false,true,true])", false);
}

#[test]
fn _0018() {
    let scope = scope!();
    te_null(false, &scope, "all([null,true,true])", r#""#);
}

#[test]
fn _0019() {
    let scope = scope!();
    te_bool(false, &scope, "all([false,null,true])", false);
}

#[test]
fn _0020() {
    let scope = scope!();
    te_null(false, &scope, "all([1])", r#""#);
}

#[test]
fn _0021() {
    let scope = scope!();
    te_null(false, &scope, "all([true,1])", r#""#);
}

#[test]
fn _0022() {
    let scope = scope!();
    te_bool(false, &scope, "all([false,1])", false);
}

#[test]
fn _0023() {
    let scope = scope!();
    te_null(false, &scope, "all([1,true,true])", r#""#);
}

#[test]
fn _0024() {
    let scope = scope!();
    te_null(
        false,
        &scope,
        "all()",
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0025() {
    let scope = scope!();
    te_null(
        false,
        &scope,
        "all(l: [true,true,true])",
        r#"parameter 'list' not found"#,
    );
}
