use super::super::*;
use crate::bifs::core::sqrt;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), "sqrt(4)", 2, 0);
}

#[test]
fn _0002() {
    te_null(
        false,
        &scope!(),
        "sqrt(-1)",
        "sqrt: argument must be positive number or zero",
    );
}

#[test]
fn _0003() {
    te_number(false, &scope!(), "sqrt(0)", 0, 0);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), "sqrt(0.0)", 0, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "sqrt(0.0)", -0, 0);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "sqrt(-0.0)", 0, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "sqrt(-0.0)", -0, 0);
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        "sqrt()",
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        "sqrt(4,4)",
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        "sqrt(null)",
        r#"sqrt: argument must be a number"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"sqrt("4")"#,
        r#"sqrt: argument must be a number"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        "sqrt(n:64.0)",
        r#"parameter 'number' not found"#,
    );
}

#[test]
fn _0013() {
    te_number(false, &scope!(), "sqrt(number:81.0)", 9, 0);
}

#[test]
fn _0014() {
    let scope = &te_scope("{ Area: 144.0 }");
    te_number(false, scope, "sqrt(Area)", 12, 0);
}

#[test]
fn _0015() {
    let scope = &te_scope("{ Area: 144.0 }");
    te_null(
        false,
        scope,
        "sqrt(n  :Area)",
        r#"parameter 'number' not found"#,
    );
}

#[test]
fn _0016() {
    let scope = &te_scope("{ Area: 144.0 }");
    te_number(false, scope, "sqrt(number : Area)", 12, 0);
}

#[test]
fn _0017() {
    let scope = &te_scope("{}");
    te_null(
        false,
        scope,
        "sqrt(null)",
        r#"sqrt: argument must be a number"#,
    );
}

#[test]
fn _0018() {
    let result = sqrt(&Value::Number(FeelNumber::infinite()));
    assert_eq!(
        "null(sqrt: result is not a finite number)",
        result.to_string()
    );
}
