use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let scope = scope!();
    te_number(false, &scope, "abs(0)", 0, 0);
}

#[test]
fn _0002() {
    let scope = scope!();
    te_number(false, &scope, "abs(-0)", 0, 0);
}

#[test]
fn _0003() {
    let scope = scope!();
    te_number(false, &scope, "abs(1)", 1, 0);
}

#[test]
fn _0004() {
    let scope = scope!();
    te_number(false, &scope, "abs(12.01)", 1201, 2);
}

#[test]
fn _0005() {
    let scope = scope!();
    te_number(false, &scope, "abs(-12.01)", 1201, 2);
}

#[test]
fn _0006() {
    let scope = scope!();
    te_number(false, &scope, "abs(-1)", 1, 0);
}

#[test]
fn _0007() {
    let scope = &scope!();
    te_number(false, scope, "abs(n:-34)", 34, 0);
}

#[test]
fn _0008() {
    let scope = &scope!();
    te_null(
        false,
        scope,
        "abs(number:-34)",
        r#"parameter 'n' not found"#,
    );
}

#[test]
fn _0009() {
    let scope = &te_scope("{ Order size: -4.5 }");
    te_number(false, scope, "abs(Order size)", 45, 1);
}

#[test]
fn _0010() {
    te_days_and_time_duration(
        false,
        &scope!(),
        r#"abs(duration("-P1D"))"#,
        false,
        24 * 60 * 60,
        0,
    );
}

#[test]
fn _0011() {
    te_years_and_months_duration(false, &scope!(), r#"abs(duration("-P1Y"))"#, 1, 0);
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"abs(null)"#,
        r#"[core::abs] invalid argument type, expected number, actual type is Null"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"abs(true)"#,
        r#"[core::abs] invalid argument type, expected number, actual type is boolean"#,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"abs("-57")"#,
        r#"[core::abs] invalid argument type, expected number, actual type is string"#,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"abs()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"abs(1,2)"#,
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}
