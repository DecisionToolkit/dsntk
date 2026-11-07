use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time(date: date("2017-01-01"), time: time("23:59:01"))"#,
        (2017, 1, 1),
        (23, 59, 1, 0),
    );
}

#[test]
fn _0002() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time(date: date and time("2017-01-01T18:30:25"), time:time("23:59:01"))"#,
        (2017, 1, 1),
        (23, 59, 1, 0),
    );
}

#[test]
fn _0003() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("2012-12-24")"#,
        (2012, 12, 24),
        (0, 0, 0, 0),
    );
}

#[test]
fn _0004() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time(from: "2012-12-24")"#,
        (2012, 12, 24),
        (0, 0, 0, 0),
    );
}

#[test]
fn _0005() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("2012-12-24T23:59:00")"#,
        (2012, 12, 24),
        (23, 59, 0, 0),
    );
}

#[test]
fn _0006() {
    te_date_time_local(
        false,
        &scope!(),
        "date    and  \n \t  time  ( \"2012-12-24T23:59:00\"  )   ",
        (2012, 12, 24),
        (23, 59, 0, 0),
    );
}

#[test]
fn _0007() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("-2017-02-28T02:02:02")"#,
        (-2017, 2, 28),
        (2, 2, 2, 0),
    );
}

#[test]
fn _0008() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("-2016-01-30T09:05:00")"#,
        (-2016, 1, 30),
        (9, 5, 0, 0),
    );
}

#[test]
fn _0009() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("2016-01-30T24:00:00")"#,
        (2016, 1, 31),
        (0, 0, 0, 0),
    );
}

#[test]
fn _0010() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("2015-12-31T23:59:59.9999999")"#,
        (2015, 12, 31),
        (23, 59, 59, 999_999_900),
    );
}

#[test]
fn _0011() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("2018-10-01T12:32:59.111111")"#,
        (2018, 10, 1),
        (12, 32, 59, 111_111_000),
    );
}

#[test]
fn _0012() {
    te_date_time_local(
        false,
        &scope!(),
        r#"date and time("2018-10-01T12:32:59.123123123123")"#,
        (2018, 10, 1),
        (12, 32, 59, 123_123_123),
    );
}

#[test]
fn _0013() {
    te_date_time_utc(
        false,
        &scope!(),
        r#"date and time("2012-12-24T23:59:00Z")"#,
        (2012, 12, 24),
        (23, 59, 0, 0),
    );
}

#[test]
fn _0014() {
    te_date_time_utc(
        false,
        &scope!(),
        r#"date and time("2012-12-24T23:59:00z")"#,
        (2012, 12, 24),
        (23, 59, 0, 0),
    );
}

#[test]
fn _0015() {
    te_date_time_offset(
        false,
        &scope!(),
        r#"date and time("2016-12-24T23:59:00-08:00")"#,
        (2016, 12, 24),
        (23, 59, 0, 0),
        -28800,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"date and time("2012-12-24T23:59:00") in [date and time("2012-12-24T23:59:00")..date and time("2012-12-24T23:59:00")]"#,
        true,
    );
}

#[test]
fn _0017() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00"))"#,
        "2016-12-24T23:59:00",
    );
}

#[test]
fn _0018() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00Z"))"#,
        "2016-12-24T23:59:00Z",
    );
}

#[test]
fn _0019() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00z"))"#,
        "2016-12-24T23:59:00Z",
    );
}

#[test]
fn _0020() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00-08:00"))"#,
        "2016-12-24T23:59:00-08:00",
    );
}

#[test]
fn _0021() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00+02:12"))"#,
        "2016-12-24T23:59:00+02:12",
    );
}

#[test]
fn _0022() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00+14:00"))"#,
        "2016-12-24T23:59:00+14:00",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00+14:01"))"#,
        r#""#,
    );
}

#[test]
fn _0024() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00-14:00"))"#,
        "2016-12-24T23:59:00-14:00",
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00-14:01"))"#,
        r#""#,
    );
}

#[test]
fn _0026() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00@Etc/UTC"))"#,
        "2016-12-24T23:59:00@Etc/UTC",
    );
}

#[test]
fn _0027() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("2016-12-24T23:59:00@Europe/Warsaw"))"#,
        "2016-12-24T23:59:00@Europe/Warsaw",
    );
}

#[test]
fn _0028() {
    te_null(
        false,
        &scope!(),
        r#"string(date and time("999999999-12-31T23:59:59.999999999@Europe/Paris"))"#,
        "",
    );
}

#[test]
fn _0029() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("262142-12-31T23:59:59.999999999@Europe/Paris"))"#,
        "262142-12-31T23:59:59.999999999@Europe/Paris",
    );
}

#[test]
fn _0030() {
    te_string(
        false,
        &scope!(),
        r#"string(date and time("-262143-01-01T00:00:00.1@Europe/Paris"))"#,
        "-262143-01-01T00:00:00.1@Europe/Paris",
    );
}

#[test]
fn _0031() {
    let scope = &te_scope(r#"{dateTimeString:"2016-12-24T23:59:00-08:00"}"#);
    te_string(
        false,
        scope,
        r#"string(date and time(dateTimeString))"#,
        "2016-12-24T23:59:00-08:00",
    );
}

#[test]
fn _0032() {
    te_null(
        false,
        &scope!(),
        r#"date and time(null)"#,
        "[core::date and time] invalid argument type, expected string, actual type is Null",
    );
}

#[test]
fn _0033() {
    te_null(
        false,
        &scope!(),
        r#"date and time("2012-13-24T23:59:00")"#,
        "[core::date and time] invalid date or date and time '2012-13-24T23:59:00'",
    );
}

#[test]
fn _0034() {
    te_null(
        false,
        &scope!(),
        r#"date and time("2012-12-24T13:65:00")"#,
        "[core::date and time] invalid date or date and time '2012-12-24T13:65:00'",
    );
}

#[test]
fn _0035() {
    te_null(
        false,
        &scope!(),
        r#"date and time("2016-12-24T23:59:00+25:00")"#,
        "[core::date and time] invalid date or date and time '2016-12-24T23:59:00+25:00'",
    );
}

#[test]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#"date and time("2016-12-24T23:59:00-27:30")"#,
        "[core::date and time] invalid date or date and time '2016-12-24T23:59:00-27:30'",
    );
}

#[test]
fn _0037() {
    te_null(
        false,
        &scope!(),
        r#"date and time("2017-12-31T13:20:00@xyz/abc")"#,
        "[core::date and time] invalid date or date and time '2017-12-31T13:20:00@xyz/abc'",
    );
}

#[test]
fn _0038() {
    te_date_time_offset(
        false,
        &scope!(),
        r#"date and time("2018-12-10T10:30:00.0001+05:00:01")"#,
        (2018, 12, 10),
        (10, 30, 0, 100_000),
        18001,
    )
}

#[test]
fn _0039() {
    te_date_time_offset(
        false,
        &scope!(),
        r#"date and time("2018-12-10T10:30:40.001+05:00")"#,
        (2018, 12, 10),
        (10, 30, 40, 1_000_000),
        18000,
    )
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        r#"date and time(10,"16:21:57")"#,
        "[core::date and time] invalid argument type, expected date and time or date, actual type is number",
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        r#"date and time(date("2021-01-24"),16)"#,
        "[core::date and time] invalid argument type, expected time, actual type is number",
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#"date and time(date and time("2021-01-24T18:12:45"),true)"#,
        "[core::date and time] invalid argument type, expected time, actual type is boolean",
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"date and time()"#,
        "expected 1,2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#"date and time("","","","")"#,
        "expected 1,2 parameters, actual number of parameters is 4",
    );
}

#[test]
fn _0045() {
    te_null(
        false,
        &scope!(),
        r#"date and time(f: "2012-12-24")"#,
        r#"invalid parameters in named::bif_date_and_time"#,
    );
}
