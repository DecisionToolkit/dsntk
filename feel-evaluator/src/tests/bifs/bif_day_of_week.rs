use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(2019,9,17))"#,
        "Tuesday",
    );
}

#[test]
fn _0002() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2019-09-17T00:00:00"))"#,
        "Tuesday",
    );
}

#[test]
fn _0003() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2019-09-17T23:59:59"))"#,
        "Tuesday",
    );
}

#[test]
fn _0004() {
    te_string(false, &scope!(), r#"day of week(date(2022,2,7))"#, "Monday");
}

#[test]
fn _0005() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2022-02-07T12:00:00.0"))"#,
        "Monday",
    );
}

#[test]
fn _0006() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(2022,2,8))"#,
        "Tuesday",
    );
}

#[test]
fn _0007() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2022-02-08T12:00:00.0"))"#,
        "Tuesday",
    );
}

#[test]
fn _0008() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(2022,2,9))"#,
        "Wednesday",
    );
}

#[test]
fn _0009() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2022-02-09T12:00:00.0"))"#,
        "Wednesday",
    );
}

#[test]
fn _0010() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(2022,2,10))"#,
        "Thursday",
    );
}

#[test]
fn _0011() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2022-02-10T12:00:00.0"))"#,
        "Thursday",
    );
}

#[test]
fn _0012() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(2022,2,11))"#,
        "Friday",
    );
}

#[test]
fn _0013() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2022-02-11T12:00:00.0"))"#,
        "Friday",
    );
}

#[test]
fn _0014() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(2022,2,12))"#,
        "Saturday",
    );
}

#[test]
fn _0015() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2022-02-12T12:00:00.0"))"#,
        "Saturday",
    );
}

#[test]
fn _0016() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(2022,2,13))"#,
        "Sunday",
    );
}

#[test]
fn _0017() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date and time("2022-02-13T12:00:00.0"))"#,
        "Sunday",
    );
}

#[test]
fn _0018() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date: date(2019,9,17))"#,
        "Tuesday",
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"day of week(date(2019,2,30))"#,
        r#"[core::day of week] invalid argument type, expected date, date and time, actual type is Null"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"day of week(date(999999999,1,1))"#,
        r#"[core::day of week] invalid argument type, expected date, date and time, actual type is Null"#,
    );
}

#[test]
fn _0021() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(262142,12,28))"#,
        "Friday",
    );
}

#[test]
fn _0022() {
    te_string(
        false,
        &scope!(),
        r#"day of week(date(-262143,1,1))"#,
        "Thursday",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"day of week(d: date(2019,1,15))"#,
        r#"parameter 'date' not found"#,
    );
}

#[test]
fn _0024() {
    te_null(
        false,
        &scope!(),
        r#"day of week()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"day of week(date(2019,1,15),date(2019,1,15))"#,
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}
