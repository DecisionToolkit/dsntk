use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#" 1/1 "#, 1, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), r#" 1 / 2 "#, 5, 1);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#" 5 / 2 / 4 "#, 625, 3);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), r#" 10 / 2 / 5"#, 1, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), r#"( 1 / 2 ) / ( 12 / 6 )"#, 25, 2);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), r#"( ( ( 6 / 3 ) ) )"#, 2, 0);
}

#[test]
fn _0007() {
    te_number_x(
        false,
        &scope!(),
        r#"1/3"#,
        r#"0.3333333333333333333333333333333333"#,
    );
}

#[test]
fn _0008() {
    te_number(false, &scope!(), r#"1.01/2"#, 505, 3);
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"0.0 / 0.0"#,
        r#"[division] division by zero"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#" 0.0 / "a" "#,
        r#"[division] incompatible types: 0.0 / "a""#,
    );
}

#[test]
fn _0011() {
    te_days_and_time_duration_x(false, &scope!(), r#" @"P10D" / 2 "#, r#"P5D"#);
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#" @"P10D" / 0.00000000000000000000000000000000000000000000000001 "#,
        r#"[division] error: P10D / 0.00000000000000000000000000000000000000000000000001"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#" @"P10D" / 0 "#,
        r#"[division] division by zero"#,
    );
}

#[test]
fn _0014() {
    te_number(false, &scope!(), r#" @"P10D" / @"P5D" "#, 2, 0);
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#" @"P10D" / @"P0D" "#,
        r#"[division] division by zero"#,
    );
}

#[test]
fn _0016() {
    te_years_and_months_duration_x(false, &scope!(), r#" @"P10Y" / 2 "#, r#"P5Y"#);
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#" @"P10Y" / 0.00000000000000000000000000000000000000000000000001 "#,
        r#"[division] error: P10Y / 0.00000000000000000000000000000000000000000000000001"#,
    );
}

#[test]
fn _0018() {
    te_number(false, &scope!(), r#" @"P10Y" / @"P5Y" "#, 2, 0);
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#" @"P10Y" / 0 "#,
        r#"[division] division by zero"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#" @"P10Y" / @"P0Y" "#,
        r#"[division] division by zero"#,
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#" @"P10D" / "a" "#,
        r#"[division] incompatible types: P10D / "a""#,
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#" @"P10Y" / "a" "#,
        r#"[division] incompatible types: P10Y / "a""#,
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#" "a" / 2.21 "#,
        r#"[division] incompatible types: "a" / 2.21"#,
    );
}
