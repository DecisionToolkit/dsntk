use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_years_and_months_duration(
        false,
        &scope!(),
        r#"years and months duration(date("2013-08-24"),date and time("2017-12-15T00:59:59"))"#,
        4,
        3,
    );
}

#[test]
fn _0002() {
    te_years_and_months_duration(
        false,
        &scope!(),
        r#"years and months duration(date("2013-08-24"),date("2017-12-15"))"#,
        4,
        3,
    );
}

#[test]
fn _0003() {
    te_years_and_months_duration(
        false,
        &scope!(),
        r#"years and months duration(date and time("2017-02-28T23:59:59"),date("2019-07-23"))"#,
        2,
        4,
    );
}

#[test]
fn _0004() {
    te_years_and_months_duration(
        false,
        &scope!(),
        r#"years and months duration(date and time("2017-02-28T23:59:59"),date and time("2019-07-23T00:59:59"))"#,
        2,
        4,
    );
}

#[test]
fn _0005() {
    te_years_and_months_duration(
        false,
        &scope!(),
        r#"years and months duration(from: date("2013-08-24"),to: date("2017-12-15"))"#,
        4,
        3,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration(date("2013-08-24"))"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration(f: date("2013-08-24"),to: date("2017-12-15"))"#,
        r#"parameter 'from' not found"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration(from: date("2013-08-24"),t: date("2017-12-15"))"#,
        r#"parameter 'to' not found"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration("2013-08-24",date("2017-12-15"))"#,
        r#"[core::years and months duration] invalid argument type, expected date, date and time, actual type is string"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration("2013-08-24",date and time("2017-12-15T00:59:59"))"#,
        r#"[core::years and months duration] invalid argument type, expected date, date and time, actual type is string"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration(date("2013-08-24"),2017)"#,
        r#"[core::years and months duration] invalid argument type, expected date, date and time, actual type is number"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"years and months duration(date and time("2013-08-24T01:00:00"),2017)"#,
        r#"[core::years and months duration] invalid argument type, expected date, date and time, actual type is number"#,
    );
}
