use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let scope = &te_scope(r#"{}"#);
    te_number(false, scope, "1-1", 0, 0);
}

#[test]
fn _0002() {
    let scope = &te_scope(r#"{}"#);
    te_number(false, scope, " 1 - 2 ", -1, 0);
}

#[test]
fn _0003() {
    let scope = &te_scope(r#"{}"#);
    te_number(false, scope, " 5 -2 -1 ", 2, 0);
}

#[test]
fn _0004() {
    let scope = &te_scope(r#"{}"#);
    te_number(false, scope, "1000-200-2", 798, 0);
}

#[test]
fn _0005() {
    let scope = &te_scope(r#"{}"#);
    te_number(false, scope, "( 1 - 2 ) - ( 3 - 4 )", 0, 0);
}

#[test]
fn _0006() {
    let scope = &te_scope(r#"{}"#);
    te_number(false, scope, "( ( ( 4 - 3 ) ) )", 1, 0);
}

#[test]
fn _0007() {
    let scope = &te_scope("{ a: 11.2, b: 0.2}");
    te_number(false, scope, "a-b", 11, 0);
}

#[test]
fn _0008() {
    let scope = &te_scope("{ a: 11.2, b: 0.2, c: 5.351 }");
    te_number(false, scope, "a-b-c", 5649, 3);
}

#[test]
fn _0009() {
    let scope = &te_scope("{ a: 11.2, b: 0.2, c: 5.351 }");
    te_number(false, scope, " a  \n -  b \n  -  c", 5649, 3);
}

#[test]
fn _0010() {
    te_time(
        false,
        &scope!(),
        r#"time("12:34:56") - duration("P0D")"#,
        FeelTime::local_opt(12, 34, 56, 0).unwrap(),
    );
}

#[test]
fn _0011() {
    te_time(
        false,
        &scope!(),
        r#"time("12:34:56") - duration("P1D")"#,
        FeelTime::local_opt(12, 34, 56, 0).unwrap(),
    );
}

#[test]
fn _0012() {
    te_time(
        false,
        &scope!(),
        r#"time("12:34:56") - duration("P11D")"#,
        FeelTime::local_opt(12, 34, 56, 0).unwrap(),
    );
}

#[test]
fn _0013() {
    te_time(
        false,
        &scope!(),
        r#"time("12:34:56") - duration("PT10S")"#,
        FeelTime::local_opt(12, 34, 46, 0).unwrap(),
    );
}

#[test]
fn _0014() {
    te_time(
        false,
        &scope!(),
        r#"time("12:34:23") - duration("PT25S")"#,
        FeelTime::local_opt(12, 33, 58, 0).unwrap(),
    );
}

#[test]
fn _0015() {
    te_time(
        false,
        &scope!(),
        r#"time("12:34:23") - duration("PT40M")"#,
        FeelTime::local_opt(11, 54, 23, 0).unwrap(),
    );
}

#[test]
fn _0016() {
    te_date(
        false,
        &scope!(),
        r#" @"2020-03-01" - @"PT24H" "#,
        2020,
        2,
        29,
    );
}

#[test]
fn _0017() {
    te_date(
        false,
        &scope!(),
        r#" @"2021-01-02" - @"PT25H" "#,
        2020,
        12,
        31,
    );
}

#[test]
fn _0018() {
    te_days_and_time_duration_x(
        false,
        &scope!(),
        r#" @"2023-02-06" - @"2021-02-06" "#,
        "P730D",
    );
}

#[test]
fn _0019() {
    te_days_and_time_duration_x(
        false,
        &scope!(),
        r#" @"2023-02-06" - @"2021-02-06T12:13:18Z" "#,
        "P729DT11H46M42S",
    );
}

#[test]
fn _0020() {
    te_days_and_time_duration_x(
        false,
        &scope!(),
        r#" @"262142-12-31T00:00:00" - @"262142-12-31T00:00:00" "#,
        "PT0S",
    );
}

#[test]
fn _0021() {
    te_date(
        false,
        &scope!(),
        r#" @"2023-02-06" - @"P1Y1M" "#,
        2022,
        1,
        6,
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-06" - @"P999999Y" "#,
        "[subtraction] incompatible types: 2023-02-06 - P999999Y",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-06" - @"P999999999999999Y" "#,
        "[subtraction] incompatible types: 2023-02-06 - P999999999999999Y",
    );
}

#[test]
fn _0024() {
    te_date(
        false,
        &scope!(),
        r#" @"2021-02-06" + (-@"P2Y") "#,
        2019,
        2,
        6,
    );
}

#[test]
fn _0025() {
    te_days_and_time_duration_x(
        false,
        &scope!(),
        r#" @"2021-02-06" - @"2023-02-18" "#,
        "-P742D",
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#" @"23746-01-01" + @"P99999999Y" "#,
        r#"invalid result while adding years and months duration to date"#,
    );
}

#[test]
fn _0027() {
    te_days_and_time_duration_x(
        false,
        &scope!(),
        r#" @"2023-02-06T12:13:18Z" - @"2021-02-06" "#,
        "P730DT12H13M18S",
    );
}

#[test]
fn _0028() {
    te_days_and_time_duration_x(
        false,
        &scope!(),
        r#" @"2023-02-06T12:13:18" - @"2021-02-06T12:13:18" "#,
        "P730D",
    );
}

#[test]
fn _0029() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"2023-02-06T12:13:18" - @"P1DT2H" "#,
        (2023, 2, 5),
        (10, 13, 18, 0),
    );
}

#[test]
fn _0030() {
    te_date_time_utc(
        false,
        &scope!(),
        r#" @"2023-02-06T12:13:18Z" - @"P1DT2H" "#,
        (2023, 2, 5),
        (10, 13, 18, 0),
    );
}

#[test]
fn _0031() {
    te_days_and_time_duration_x(
        false,
        &scope!(),
        r#" @"12:34:56" - @"09:15:36" "#,
        "PT3H19M20S",
    );
}

#[test]
fn _0032() {
    te_time(
        false,
        &scope!(),
        r#" @"12:34:56" - (-@"P1DT2H") "#,
        FeelTime::local(14, 34, 56, 0),
    );
}

#[test]
fn _0033() {
    te_time(
        false,
        &scope!(),
        r#" @"12:34:56" - @"PT14H" "#,
        FeelTime::local(22, 34, 56, 0),
    );
}

#[test]
fn _0034() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"2023-02-06T12:13:18" - @"P1Y" "#,
        (2022, 2, 6),
        (12, 13, 18, 0),
    );
}

#[test]
fn _0035() {
    te_days_and_time_duration_x(false, &scope!(), r#" @"P2DT3H" - @"PT3H" "#, "P2D");
}

#[test]
fn _0036() {
    te_years_and_months_duration_x(false, &scope!(), r#" @"P2Y5M" - @"P5M" "#, "P2Y");
}

/// Date converted to date and time has the timezone equal to UTC 00:00:00. Subtracted date and time has local timezone.
/// Such subtraction is not allowed (both date and times should have offset defined), that's why null value should be returned.
#[test]
fn _0037() {
    te_null(
        false,
        &scope!(),
        r#" @"2021-01-02" - @"2021-01-01T10:10:10" "#,
        "[subtraction] incompatible types: 2021-01-02 - 2021-01-01T10:10:10",
    );
}

#[test]
fn _0038() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-06T12:13:18" - 5 "#,
        "[subtraction] incompatible types: 2023-02-06T12:13:18 - 5",
    );
}

#[test]
fn _0039() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"262142-12-31T00:00:00" - @"P20D" "#,
        (262142, 12, 11),
        (0, 0, 0, 0),
    );
}

#[test]
fn _0040() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"262142-12-31T00:00:00" - @"P20Y" "#,
        (262122, 12, 31),
        (0, 0, 0, 0),
    );
}

#[test]
fn _0041() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"262142-12-31T00:00:00" - @"P2DT3H" "#,
        (262142, 12, 28),
        (21, 0, 0, 0),
    );
}
