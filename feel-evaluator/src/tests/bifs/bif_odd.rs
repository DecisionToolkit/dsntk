use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), "odd(2)", false);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), "odd(-2)", false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), "odd(1)", true);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), "odd(-1)", true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), "odd(0)", false);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), "odd(-0)", false);
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        "odd()",
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        "odd(4,4)",
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), "odd(number:4)", false);
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        "odd(n:4)",
        r#"parameter 'number' not found"#,
    );
}

#[test]
fn _0011() {
    let scope = &te_scope("{ even number: 20, odd number: 21 }");
    te_bool(false, scope, "odd(even number)", false);
}

#[test]
fn _0012() {
    let scope = &te_scope("{ even number: 20, odd number: 21 }");
    te_bool(false, scope, "odd(odd number)", true);
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"odd("2")"#,
        r#"[core::odd] invalid argument type, expected number, actual type is string"#,
    );
}
