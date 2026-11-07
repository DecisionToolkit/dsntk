use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), "-0", 0, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), "-0", -0, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), "0", 0, 0);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), "--0", 0, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "-1", -1, 0);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "--1", 1, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), " -10.2 ", -102, 1);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), " - 10 ", -10, 0);
}

#[test]
fn _0009() {
    te_number(false, &scope!(), "(-20)", -20, 0);
}

#[test]
fn _0010() {
    te_number(false, &scope!(), " ( -21 ) ", -21, 0);
}

#[test]
fn _0011() {
    te_number(false, &scope!(), " ( - 22 ) ", -22, 0);
}

#[test]
fn _0012() {
    te_number(false, &scope!(), " ((( - 23 ))) ", -23, 0);
}

#[test]
fn _0013() {
    te_number(false, &scope!(), " - - 24 ", 24, 0);
}

#[test]
fn _0014() {
    te_number(false, &scope!(), " - ( - 25 ) ", 25, 0);
}

#[test]
fn _0015() {
    te_days_and_time_duration_x(false, &scope!(), r#"duration("-PT1H")"#, "-PT1H")
}

#[test]
fn _0016() {
    te_days_and_time_duration_x(false, &scope!(), r#" - @"PT1H" "#, "-PT1H")
}

#[test]
fn _0017() {
    te_years_and_months_duration_x(false, &scope!(), r#" - @"P1Y" "#, "-P1Y")
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#" -"a" "#,
        "unexpected type in arithmetic negation: string",
    )
}
