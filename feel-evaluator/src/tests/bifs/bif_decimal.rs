use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), "decimal(1,2)", 100, 2);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), "decimal(n: 1, scale: 2)", 100, 2);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), "decimal(scale: 2, n: 1)", 100, 2);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), "decimal(1/3,2)", 330, 3);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "decimal(0.505,2)", 50, 2);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "decimal(0.515,2)", 52, 2);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "decimal(1/3, 2.5)", 33, 2);
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        "decimal(1/3, 6177)",
        "[core::decimal] scale is out of range: 6177",
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        "decimal(1/3, -6112)",
        "[core::decimal] scale is out of range: -6112",
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"decimal(1/3, "scale")"#,
        r#"[core::decimal] scale value is not a number: "scale""#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"decimal("number", 6)"#,
        r#"[core::decimal] number value is not a number: "number""#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"decimal()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"decimal("1234")"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"decimal("1234",1,2)"#,
        r#"expected 2 parameters, actual number of parameters is 3"#,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        "decimal(n: 1, s: 2)",
        r#"parameter 'scale' not found"#,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        "decimal(number: 1, scale: 2)",
        r#"parameter 'n' not found"#,
    );
}
