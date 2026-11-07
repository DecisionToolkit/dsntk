use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), "1*1", 1, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), " 1 * 2 ", 2, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), " 5 *2 *3", 30, 0);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), "10*2*5", 100, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "( 1 * 2 ) * ( 3 * 4 )", 24, 0);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "( ( ( 4 * 3 ) ) )", 12, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "(3*2)+(4*5)", 26, 0);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), "3*2+4*5", 26, 0);
}

#[test]
fn _0009() {
    te_number(false, &scope!(), "3*(2+4)*5", 90, 0);
}

#[test]
fn _0010() {
    te_number(false, &scope!(), ".10 * 30.00", 3, 0);
}

#[test]
fn _0011() {
    te_number(false, &scope!(), "1.0*10**3", 1000, 0);
}

#[test]
fn _0012() {
    let scope = &te_scope("{Monthly Salary:10000}");
    te_number(false, scope, "12 * Monthly Salary", 120000, 0);
}

#[test]
fn _0013() {
    te_days_and_time_duration_x(false, &scope!(), r#" 1.5 * @"P4DT1H" "#, "P6DT1H30M");
}

#[test]
fn _0014() {
    te_days_and_time_duration_x(false, &scope!(), r#" @"P4DT1H" * 1.5 "#, "P6DT1H30M");
}

#[test]
fn _0015() {
    te_years_and_months_duration_x(false, &scope!(), r#" 10 * @"P1Y" "#, "P10Y");
}

#[test]
fn _0016() {
    te_years_and_months_duration_x(false, &scope!(), r#" @"P1Y" * 10 "#, "P10Y");
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#" 1 * "a" "#,
        r#"[multiplication] incompatible types: 1 * "a""#,
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#" @"P4DT1H" * @"P2Y" "#,
        r#"[multiplication] incompatible types: P4DT1H * P2Y"#,
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#" @"P2Y" * @"P4DT1H" "#,
        r#"[multiplication] incompatible types: P2Y * P4DT1H"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#" 999999999999999999999 * @"P1D" "#,
        r#"multiplication result is out of range of days and time duration"#,
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#" 999999999999999999999 * @"P1Y" "#,
        r#"multiplication result is out of range of years and months duration"#,
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#" @"P1D" * 999999999999999999999 "#,
        r#"multiplication result is out of range of days and time duration"#,
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#" @"P1Y" * 999999999999999999999 "#,
        r#"multiplication result is out of range of years and months duration"#,
    );
}

#[test]
fn _0024() {
    te_null(false, &scope!(), r#" null * @"P1Y" "#, r#""#);
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#" "a" * 2 "#,
        r#"unexpected value type in multiplication: string"#,
    );
}
