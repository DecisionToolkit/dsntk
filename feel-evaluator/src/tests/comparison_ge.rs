use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"2>=1"#, true);
}

#[test]
fn _0002() {
    te_null(
        false,
        &scope!(),
        r#" 2 >= "a" "#,
        "eval_greater_or_equal_number",
    );
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        r#" false >= 21 "#,
        "eval_greater_or_equal",
    );
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"1>=1"#, true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"1.277>=1.276"#, true);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), r#"1.276>=1.276"#, true);
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), r#".5>=-.54635"#, true);
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), r#".57>=-.57"#, true);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), r#"2>=2.1"#, false);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), r#"(1+1.2)>=2.01"#, true);
}

#[test]
fn _0011() {
    te_bool(false, &scope!(), r#"(1+1)>=2.0"#, true);
}

#[test]
fn _0012() {
    te_bool(false, &scope!(), r#"(1.1+2)>=3.11"#, false);
}

#[test]
fn _0013() {
    te_bool(false, &scope!(), r#" ( 1.02 + 0.99 ) >= 2.0"#, true);
}

#[test]
fn _0014() {
    te_bool(false, &scope!(), r#" ( 1 + 1.0 ) >= 2.0"#, true);
}

#[test]
fn _0015() {
    te_bool(false, &scope!(), r#" ( ( ( 1.1 + 3.1 ) ) ) >= 4.21"#, false);
}

#[test]
fn _0016() {
    te_bool(false, &scope!(), r#"(0.9+1)>=(5.1-3.21)"#, true);
}

#[test]
fn _0017() {
    te_bool(false, &scope!(), r#"(0.9+1.1)>=(5.0-3)"#, true);
}

#[test]
fn _0018() {
    te_bool(false, &scope!(), r#"(1*2)>=(10.0/5.1)"#, true);
}

#[test]
fn _0019() {
    te_bool(false, &scope!(), r#"(1*2)>=(10.0/5.0)"#, true);
}

#[test]
fn _0020() {
    te_bool(false, &scope!(), r#" "alfa" >= "alfa" "#, true);
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#" "alfa" >= 1 "#,
        "eval_greater_or_equal_string",
    );
}

#[test]
fn _0022() {
    te_bool(false, &scope!(), r#" "beta" >= "alfa" "#, true);
}

#[test]
fn _0023() {
    te_bool(false, &scope!(), r#" "alfa" >= "beta" "#, false);
}

#[test]
fn _0024() {
    te_bool(false, &scope!(), r#"@"2021-11-02" >= @"2021-11-01""#, true);
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#" @"2021-11-02" >= 1 "#,
        "eval_greater_or_equal_date",
    );
}

#[test]
fn _0026() {
    te_bool(false, &scope!(), r#"@"2021-11-02" >= @"2021-11-01""#, true);
}

#[test]
fn _0027() {
    te_bool(false, &scope!(), r#"@"2021-11-01" >= @"2021-11-01""#, true);
}

#[test]
fn _0028() {
    te_bool(false, &scope!(), r#"@"2021-10-31" >= @"2021-11-01""#, false);
}

#[test]
fn _0029() {
    te_bool(
        false,
        &scope!(),
        r#" @"2021-11-01T11:10:12" >= @"2021-11-01T11:10:12" "#,
        true,
    );
}

#[test]
fn _0030() {
    te_bool(
        false,
        &scope!(),
        r#" @"2023-02-09T23:59:59Z" >= @"2023-02-09T00:00:00" "#,
        false,
    );
}

#[test]
fn _0031() {
    te_null(
        false,
        &scope!(),
        r#" @"2021-11-01T11:10:12" >= false "#,
        "eval_greater_or_equal_date_time",
    );
}

#[test]
fn _0032() {
    te_bool(
        false,
        &scope!(),
        r#" @"2021-11-01T11:10:13" >= @"2021-11-01T11:10:12" "#,
        true,
    );
}

#[test]
fn _0033() {
    te_bool(
        false,
        &scope!(),
        r#" @"2021-11-01T11:10:11" >= @"2021-11-01T11:10:12" "#,
        false,
    );
}

#[test]
fn _0034() {
    te_bool(false, &scope!(), r#" @"11:10:12" >= @"11:10:12" "#, true);
}

#[test]
fn _0035() {
    te_null(
        false,
        &scope!(),
        r#" @"11:10:12" >= @"P11D" "#,
        "eval_greater_or_equal_time",
    );
}

#[test]
fn _0036() {
    te_bool(false, &scope!(), r#" @"11:10:13" >= @"11:10:12" "#, true);
}

#[test]
fn _0037() {
    te_bool(false, &scope!(), r#" @"11:10:11" >= @"11:10:12" "#, false);
}

#[test]
fn _0038() {
    te_bool(false, &scope!(), r#" @"P1D" >= @"P1D" "#, true);
}

#[test]
fn _0039() {
    te_null(
        false,
        &scope!(),
        r#" @"P1D" >= 1 "#,
        "eval_greater_or_equal_days_and_time_duration",
    );
}

#[test]
fn _0040() {
    te_bool(false, &scope!(), r#" @"P2D" >= @"P1D" "#, true);
}

#[test]
fn _0041() {
    te_bool(false, &scope!(), r#" @"P1D" >= @"P2D" "#, false);
}

#[test]
fn _0042() {
    te_bool(false, &scope!(), r#" @"P1Y" >= @"P1Y" "#, true);
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#" @"P1Y" >= 1 "#,
        "eval_greater_or_equal_years_and_months_duration",
    );
}

#[test]
fn _0044() {
    te_bool(false, &scope!(), r#" @"P2Y" >= @"P1Y" "#, true);
}

#[test]
fn _0045() {
    te_bool(false, &scope!(), r#" @"P1Y" >= @"P2Y" "#, false);
}
