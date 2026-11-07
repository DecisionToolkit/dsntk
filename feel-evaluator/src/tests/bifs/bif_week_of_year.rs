use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#"week of year(date(2019,9,17))"#, 38, 0);
}

#[test]
fn _0002() {
    te_number(
        false,
        &scope!(),
        r#"week of year(date(262142,12,28))"#,
        52,
        0,
    );
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#"week of year(date(-262143,1,1))"#, 1, 0);
}

#[test]
fn _0004() {
    te_number(
        false,
        &scope!(),
        r#"week of year(date and time("2019-09-17T00:00:00"))"#,
        38,
        0,
    );
}

#[test]
fn _0005() {
    te_number(
        false,
        &scope!(),
        r#"week of year(date: date(2019,9,17))"#,
        38,
        0,
    );
}

#[test]
fn _0006() {
    te_number(
        false,
        &scope!(),
        r#"week of year(date: date and time("2019-09-17T00:00:00"))"#,
        38,
        0,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"week of year(date: date(999999999,9,17))"#,
        "[core::week of year] invalid argument type, expected date, date and time, actual type is Null",
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"week of year(d: date(2021,9,17))"#,
        "parameter 'date' not found",
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"week of year(date: 10)"#,
        "[core::week of year] invalid argument type, expected date, date and time, actual type is number",
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"week of year()"#,
        "expected 1 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"week of year(date(2019,9,17),date(2019,9,17))"#,
        "expected 1 parameters, actual number of parameters is 2",
    );
}
