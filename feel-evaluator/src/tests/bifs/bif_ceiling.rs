use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), "ceiling(1.5)", 2, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), "ceiling(1.51, 1)", 16, 1);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), "ceiling(1.51, 6144)", 151, 2);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), "ceiling(1.51, -6111)", 1, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "ceiling(123.12, -1)", 130, 0);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "ceiling(-1.5)", -1, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "ceiling(-1.51, 1)", -15, 1);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), "ceiling(-1.53, -1)", 0, 0);
}

#[test]
fn _0009() {
    te_number(false, &scope!(), "ceiling(-123, -1)", -120, 0);
}

#[test]
fn _0010() {
    te_number(false, &scope!(), "ceiling(-9.51, 6144)", -951, 2);
}

#[test]
fn _0011() {
    te_number(false, &scope!(), "ceiling(-9.51, -6111)", 0, 0);
}

#[test]
fn _0012() {
    te_number(false, &scope!(), "ceiling(--1)", 1, 0);
}

#[test]
fn _0013() {
    te_number(false, &scope!(), "ceiling(-5/2.3*5)", -10, 0);
}

#[test]
fn _0014() {
    te_number(false, &scope!(), "ceiling(n: 5.777)", 6, 0);
}

#[test]
fn _0015() {
    te_number(false, &scope!(), "ceiling(n: 5.777, scale: 1)", 58, 1);
}

#[test]
fn _0016() {
    te_number(false, &scope!(), "ceiling(n:-.33333)", 0, 0);
}

#[test]
fn _0017() {
    te_number(false, &scope!(), "ceiling(n:.33333)", 1, 0);
}

#[test]
fn _0018() {
    let scope = &te_scope("{ Order size: 23.27 }");
    te_number(false, scope, "ceiling(n:Order size)", 24, 0);
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        "ceiling(number:5.777)",
        "parameter 'n' not found",
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        "ceiling(number: 5.777, scale: 1)",
        "parameter 'n' not found",
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        "ceiling(n: 5.777, s: 1)",
        "parameter 'scale' not found",
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        "ceiling(n: 5.777, scale: 1, foo: 11)",
        "expected 1,2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"ceiling(true)"#,
        r#"[core::ceiling] invalid argument type, expected number, actual type is boolean"#,
    );
}

#[test]
fn _0024() {
    te_null(
        false,
        &scope!(),
        r#"ceiling()"#,
        "expected 1,2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"ceiling(1,2,3)"#,
        "expected 1,2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#"ceiling(n: 5.777, scale: "1")"#,
        "[core::ceiling] invalid argument type, expected number, actual type is string",
    );
}

#[test]
fn _0027() {
    //--------------------------------------------------------------------------------------------------------------------
    // First check of the scale. Should be in i32 range.
    //--------------------------------------------------------------------------------------------------------------------
    te_null(
        false,
        &scope!(),
        r#"ceiling(n: 5.777, scale: 2147483648)"#,
        "[core::ceiling] invalid scale: 2147483648",
    );
}

#[test]
fn _0028() {
    //--------------------------------------------------------------------------------------------------------------------
    // First check of the scale. Should be in i32 range.
    //--------------------------------------------------------------------------------------------------------------------
    te_null(
        false,
        &scope!(),
        r#"ceiling(n: 5.777, scale: -2147483649)"#,
        "[core::ceiling] invalid scale: -2147483649",
    );
}

#[test]
fn _0029() {
    //--------------------------------------------------------------------------------------------------------------------
    // Now the scale is truncated to integer.
    // When non-integer scale should be reported as an error, then this test should be modified.
    //--------------------------------------------------------------------------------------------------------------------
    te_number(false, &scope!(), "ceiling(1.51, 1.28)", 16, 1);
}

#[test]
fn _0030() {
    te_null(
        false,
        &scope!(),
        r#"ceiling(n: 5.777, scale: 6145)"#,
        "[core::ceiling] <FeelNumberError> invalid scale, allowed range is -6111..6144, actual is 6145",
    );
}

#[test]
fn _0031() {
    te_null(
        false,
        &scope!(),
        r#"ceiling(n: 5.777, scale: -6112)"#,
        "[core::ceiling] <FeelNumberError> invalid scale, allowed range is -6111..6144, actual is -6112",
    );
}
