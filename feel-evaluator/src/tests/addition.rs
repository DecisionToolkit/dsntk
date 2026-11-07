use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), "1+1", 2, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), " 1 + 2 ", 3, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), " 5 +2 +1 ", 8, 0);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), "20+200+2", 222, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "( 1 + 2 ) + ( 3 + 4 )", 10, 0);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "( ( ( 1 + 2 ) ) )", 3, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "(1+2)*(3+2)", 15, 0);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), "1+2*3+2", 9, 0);
}

#[test]
fn _0009() {
    te_number(false, &scope!(), ".25 + .2", 45, 2);
}

#[test]
fn _0010() {
    let scope = &te_scope(r#"{Full Name:"John Doe"}"#);
    te_string(false, scope, r#""Hello " + Full Name"#, "Hello John Doe");
}

#[test]
fn _0011() {
    let scope = &te_scope(r#"{Employment Status:"EMPLOYED"}"#);
    te_string(
        false,
        scope,
        r#""You are " + Employment Status"#,
        "You are EMPLOYED",
    );
}

#[test]
fn _0012() {
    let scope = &te_scope(r#"{Full Name:"John Doe", Employment Status:"EMPLOYED"}"#);
    te_string(
        false,
        scope,
        r#""Hello " + Full Name + ", you are " + Employment Status"#,
        "Hello John Doe, you are EMPLOYED",
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"1.25 + "alfa""#,
        r#"incompatible types in addition: 1.25(number) + "alfa"(string)"#,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#" "a" + 1 "#,
        r#"expected string as a second argument in addition"#,
    );
}

#[test]
fn _0015() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"2021-01-12T10:10:10" + @"P1DT1H" "#,
        (2021, 1, 13),
        (11, 10, 10, 0),
    );
}

#[test]
fn _0016() {
    te_date(
        false,
        &scope!(),
        r#" @"2021-01-12" + @"P1DT1H" "#,
        2021,
        1,
        13,
    );
}

#[test]
fn _0017() {
    te_date(
        false,
        &scope!(),
        r#" @"2021-01-12" + (-@"P1D") "#,
        2021,
        1,
        11,
    );
}

#[test]
fn _0018() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"P1DT1H" + @"2021-01-12T10:10:10" "#,
        (2021, 1, 13),
        (11, 10, 10, 0),
    );
}

#[test]
fn _0019() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"2021-01-12T10:10:10" + @"P1Y1M" "#,
        (2022, 2, 12),
        (10, 10, 10, 0),
    );
}

#[test]
fn _0020() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"P1Y1M" + @"2021-01-12T10:10:10" "#,
        (2022, 2, 12),
        (10, 10, 10, 0),
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"@"2021-01-12T10:10:10" + 1"#,
        "[builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is number",
    );
}

#[test]
fn _0022() {
    te_years_and_months_duration_x(false, &scope!(), r#" @"P1Y1M" + @"P2Y3M" "#, r#"P3Y4M"#);
}

#[test]
fn _0023() {
    te_days_and_time_duration_x(false, &scope!(), r#" @"P1DT2H" + @"P2DT3H" "#, r#"P3DT5H"#);
}

#[test]
fn _0024() {
    te_null(false, &scope!(), r#" null + @"P1DT2H" "#, r#""#);
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#" @"P1D" + 1 "#,
        r#"[builders::add] invalid argument type, expected days and time duration, date and time, actual type is number"#,
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#" @"P1Y1M" + 1 "#,
        r#"[builders::add] invalid argument type, expected years and months duration, date and time, actual type is number"#,
    );
}

#[test]
fn _0027() {
    te_null(
        false,
        &scope!(),
        r#" true + @"P1DT2H" "#,
        r#"[builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean"#,
    );
}

#[test]
fn _0028() {
    te_date(false, &scope!(), r#" @"2023-02-05" + @"P2D" "#, 2023, 2, 7);
}

#[test]
fn _0029() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-06" + @"P999999Y" "#,
        "invalid result while adding years and months duration to date",
    );
}

#[test]
fn _0030() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-06" + @"P999999999999999Y" "#,
        "invalid result while adding years and months duration to date",
    );
}

#[test]
fn _0031() {
    te_date(false, &scope!(), r#" @"P2D" + @"2023-02-05" "#, 2023, 2, 7);
}

#[test]
fn _0032() {
    te_null(
        false,
        &scope!(),
        r#" @"262142-01-01" + @"P99999D" "#,
        r#"invalid result while adding days and time duration to date"#,
    );
}

#[test]
fn _0033() {
    te_date(
        false,
        &scope!(),
        r#" @"2023-02-05" + @"P2Y1M" "#,
        2025,
        3,
        5,
    );
}

#[test]
fn _0034() {
    te_date(
        false,
        &scope!(),
        r#" @"P2Y1M" + @"2023-02-05" "#,
        2025,
        3,
        5,
    );
}

#[test]
fn _0035() {
    te_null(
        false,
        &scope!(),
        r#" @"262142-01-01" + @"P100Y" "#,
        r#"invalid result while adding years and months duration to date"#,
    );
}

#[test]
#[cfg(not(target_os = "windows"))]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#" @"262142-12-31T10:01:02" + @"P99999D" "#,
        r#"invalid result while adding days and time duration to date and time"#,
    );
}

#[test]
fn _0037() {
    te_null(
        false,
        &scope!(),
        r#" @"262043-01-01T10:01:02" + @"P100Y" "#,
        r#"invalid result while adding years and months duration to date and time"#,
    );
}

#[test]
fn _0038() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-05" + 1 "#,
        r#"[builders::add] invalid argument type, expected years and months duration, actual type is number"#,
    );
}

#[test]
fn _0039() {
    te_null(
        false,
        &scope!(),
        r#" @"P99999D" + @"262142-01-01" "#,
        r#"invalid result while adding date to days and time duration"#,
    );
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        r#" @"P100Y" + @"262043-01-01" "#,
        r#"invalid result while adding date to years and months duration"#,
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        r#" @"P99999D" + @"262142-02-03T10:01:02" "#,
        r#"invalid result while adding date and time to days and time duration"#,
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#" @"P100Y" + @"262043-02-03T10:01:02" "#,
        r#"invalid result while adding date and time to years and months duration"#,
    );
}

#[test]
fn _0043() {
    te_time(
        false,
        &scope!(),
        r#" @"10:11:12" + @"PT2H" "#,
        FeelTime::local(12, 11, 12, 0),
    );
}

#[test]
fn _0044() {
    te_time(
        false,
        &scope!(),
        r#" @"PT2H" + @"10:11:12" "#,
        FeelTime::local(12, 11, 12, 0),
    );
}

#[test]
fn _0045() {
    te_null(
        false,
        &scope!(),
        r#" @"10:11:12" + 1 "#,
        "[builders::add] invalid argument type, expected days and time duration, actual type is number",
    );
}
