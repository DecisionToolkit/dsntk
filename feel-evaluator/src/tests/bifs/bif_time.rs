use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_time(
        false,
        &scope!(),
        r#"time(11,59,45,null)"#,
        FeelTime::local(11, 59, 45, 0),
    );
}

#[test]
fn _0002() {
    te_time(
        false,
        &scope!(),
        r#"time(hour: 11, minute: 59, second: 45, offset: null)"#,
        FeelTime::local(11, 59, 45, 0),
    );
}

#[test]
fn _0003() {
    te_time(
        false,
        &scope!(),
        r#"time(hour: 11, minute: 59, second: 45)"#,
        FeelTime::local(11, 59, 45, 0),
    );
}

#[test]
fn _0004() {
    let scope = &te_scope(r#"{Hours:12,Minutes:59,Seconds:1.3,Timezone:@"-PT1H"}"#);
    te_time(
        false,
        scope,
        r#"time(Hours,Minutes,Seconds,Timezone)"#,
        FeelTime::offset_opt(12, 59, 1, 300_000_000, -3600).unwrap(),
    );
}

#[test]
fn _0005() {
    let scope = &te_scope(
        r#"{Hours:12,Minutes:59,Seconds:1.9999999999999999999999999999999999999999999999999,Timezone:@"-PT1H"}"#,
    );
    te_time(
        false,
        scope,
        r#"time(Hours,Minutes,Seconds,Timezone)"#,
        FeelTime::offset_opt(12, 59, 2, 0, -3600).unwrap(),
    );
}

#[test]
fn _0006() {
    let scope = &te_scope(r#"{Hours:12.999,Minutes:59.999,Seconds:1.0003,Timezone:@"-PT1H3S"}"#);
    te_time(
        false,
        scope,
        r#"time(Hours,Minutes,Seconds,Timezone)"#,
        FeelTime::offset_opt(12, 59, 1, 300_000, -3603).unwrap(),
    );
}

#[test]
fn _0007() {
    te_time(
        false,
        &scope!(),
        r#"time("23:59:00")"#,
        FeelTime::local(23, 59, 0, 0),
    );
}

#[test]
fn _0008() {
    te_time(
        false,
        &scope!(),
        r#"time("23:59:59")"#,
        FeelTime::local(23, 59, 59, 0),
    );
}

#[test]
fn _0009() {
    te_time(
        false,
        &scope!(),
        r#"time("24:00:00")"#,
        FeelTime::local(0, 0, 0, 0),
    );
}

#[test]
fn _0010() {
    te_time(
        false,
        &scope!(),
        r#"  time  (  "23:59:00"         )   "#,
        FeelTime::local(23, 59, 0, 0),
    );
}

#[test]
fn _0011() {
    te_time(
        false,
        &scope!(),
        r#"time("23:59:00Z")"#,
        FeelTime::utc(23, 59, 0, 0),
    );
}

#[test]
fn _0012() {
    te_time(
        false,
        &scope!(),
        r#"time("23:59:00z")"#,
        FeelTime::utc(23, 59, 0, 0),
    );
}

#[test]
fn _0013() {
    te_time(
        false,
        &scope!(),
        r#"time("11:22:33-00:00")"#,
        FeelTime::utc(11, 22, 33, 0),
    );
}

#[test]
fn _0014() {
    te_time(
        false,
        &scope!(),
        r#"time(from: "11:22:33-00:00")"#,
        FeelTime::utc(11, 22, 33, 0),
    );
}

#[test]
fn _0015() {
    te_time(
        false,
        &scope!(),
        r#"time("11:22:33+00:00")"#,
        FeelTime::utc(11, 22, 33, 0),
    );
}

#[test]
fn _0016() {
    te_time(
        false,
        &scope!(),
        r#"time(time("11:00:00"))"#,
        FeelTime::local(11, 0, 0, 0),
    );
}

#[test]
fn _0017() {
    te_time(
        false,
        &scope!(),
        r#"time(from: time("11:00:00"))"#,
        FeelTime::local(11, 0, 0, 0),
    );
}

#[test]
fn _0018() {
    te_time(
        false,
        &scope!(),
        r#"time(date and time("2019-12-06T18:34:12"))"#,
        FeelTime::local(18, 34, 12, 0),
    );
}

#[test]
fn _0019() {
    te_time(
        false,
        &scope!(),
        r#"time(date and time("2019-12-06T11:00:00Z"))"#,
        FeelTime::utc(11, 0, 0, 0),
    );
}

#[test]
fn _0020() {
    te_time(
        false,
        &scope!(),
        r#"time(from: date and time("2019-12-06T11:00:00Z"))"#,
        FeelTime::utc(11, 0, 0, 0),
    );
}

#[test]
fn _0021() {
    te_time(
        false,
        &scope!(),
        r#"time(date and time("2019-12-06T11:00:00z"))"#,
        FeelTime::utc(11, 0, 0, 0),
    );
}

#[test]
fn _0022() {
    te_time(
        false,
        &scope!(),
        r#"time(date("2019-12-06"))"#,
        FeelTime::utc(0, 0, 0, 0),
    );
}

#[test]
fn _0023() {
    te_bool(
        false,
        &scope!(),
        r#"time("12:21:12") in [time("12:21:12")..time("12:21:12")]"#,
        true,
    );
}

#[test]
fn _0024() {
    te_bool(
        false,
        &scope!(),
        r#"time("12:21:11") in [time("12:21:12")..time("12:21:12")]"#,
        false,
    );
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"time("12:21:13") in [time("12:21:12")..time("12:21:12")]"#,
        false,
    );
}

#[test]
fn _0026() {
    te_bool(
        false,
        &scope!(),
        r#"time("12:21:12") in (time("12:21:11")..time("12:21:13"))"#,
        true,
    );
}

#[test]
fn _0027() {
    te_null(false, &scope!(), r#"time("22:63:12")"#, "time_1");
}

#[test]
fn _0028() {
    te_null(false, &scope!(), r#"time("22:10:12+15:00")"#, "time_1");
}

#[test]
fn _0029() {
    te_null(false, &scope!(), r#"time("22:10:12-15:00")"#, "time_1");
}

#[test]
fn _0030() {
    te_null(
        false,
        &scope!(),
        r#"time(24,59,45,null)"#,
        "[core::time_4] hour must be 0..23, current value is: 24",
    );
}

#[test]
fn _0031() {
    te_null(
        false,
        &scope!(),
        r#"time(23,60,45,null)"#,
        "[core::time_4] minute must be 0..59, current value is: 60",
    );
}

#[test]
fn _0032() {
    te_null(
        false,
        &scope!(),
        r#"time(24,59,45,null)"#,
        "[core::time_4] hour must be 0..23, current value is: 24",
    );
}

#[test]
fn _0033() {
    te_null(
        false,
        &scope!(),
        r#"time(23,60,45,null)"#,
        "[core::time_4] minute must be 0..59, current value is: 60",
    );
}

#[test]
fn _0034() {
    te_null(
        false,
        &scope!(),
        r#"time(23,59,60,null)"#,
        "[core::time_4] second must be 0..59, current value is: 60",
    );
}

#[test]
fn _0035() {
    te_null(
        false,
        &scope!(),
        r#"time(-12,12,12,null)"#,
        "[core::time_4] hour must be 0..23, current value is: -12",
    );
}

#[test]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#"time(12,-12,12,null)"#,
        "[core::time_4] minute must be 0..59, current value is: -12",
    );
}

#[test]
fn _0037() {
    te_null(
        false,
        &scope!(),
        r#"time(12,12,-12,null)"#,
        "[core::time_4] second must be 0..59, current value is: -12",
    );
}

#[test]
fn _0038() {
    te_time(
        false,
        &scope!(),
        r#"time(11,59,45)"#,
        FeelTime::local(11, 59, 45, 0),
    );
}

#[test]
fn _0039() {
    te_null(
        false,
        &scope!(),
        r#"time()"#,
        r#"expected 1,3,4 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        r#"time(12,12)"#,
        r#"expected 1,3,4 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        r#"time(12,12,12,12,12)"#,
        r#"expected 1,3,4 parameters, actual number of parameters is 5"#,
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#"time(f: "11:22:33-00:00")"#,
        r#"invalid parameters in bif time"#,
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"time(h: 11, minute: 59, second: 45, offset: null)"#,
        r#"invalid parameters in bif time"#,
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#"time(hour: 11, m: 59, second: 45, offset: null)"#,
        r#"invalid parameters in bif time"#,
    );
}

#[test]
fn _0045() {
    te_null(
        false,
        &scope!(),
        r#"time(hour: 11, minute: 59, s: 45, offset: null)"#,
        r#"invalid parameters in bif time"#,
    );
}

#[test]
fn _0046() {
    te_null(
        false,
        &scope!(),
        r#"time("11", 59, 45, null)"#,
        r#"[core::time_4] hour must be a number, current type is: string"#,
    );
}

#[test]
fn _0047() {
    te_null(
        false,
        &scope!(),
        r#"time(11, "59", 45, null)"#,
        r#"[core::time_4] minutes must be a number, current type is: string"#,
    );
}

#[test]
fn _0048() {
    te_null(
        false,
        &scope!(),
        r#"time(11, 59, "45", null)"#,
        r#"[core::time_4] seconds must be a number, current type is: string"#,
    );
}

#[test]
fn _0049() {
    te_null(
        false,
        &scope!(),
        r#"time(23,59,45,"10")"#,
        "expected days and time duration or null, current offset type is string",
    );
}

#[test]
fn _0050() {
    te_null(
        false,
        &scope!(),
        r#"time(24,59,3)"#,
        r#"hour must be 0..23, current value is 24"#,
    );
}

#[test]
fn _0051() {
    te_null(
        false,
        &scope!(),
        r#"time(12,61,3)"#,
        r#"minute must be 0..59, current value is 61"#,
    );
}

#[test]
fn _0052() {
    te_null(
        false,
        &scope!(),
        r#"time(11,59,300)"#,
        r#"second must be 0..59, current value is 300"#,
    );
}

#[test]
fn _0053() {
    te_null(
        false,
        &scope!(),
        r#"time("12",45,13)"#,
        r#"hour must be a number, current type is string"#,
    );
}

#[test]
fn _0054() {
    te_null(
        false,
        &scope!(),
        r#"time(12,"45",13)"#,
        r#"minutes must be a number, current type is string"#,
    );
}

#[test]
fn _0055() {
    te_null(
        false,
        &scope!(),
        r#"time(12,45,"13")"#,
        r#"seconds must be a number, current type is string"#,
    );
}

#[test]
#[should_panic]
fn _0056() {
    te_null(
        false,
        &scope!(),
        r#"time(23,59,45,@"P3000000000000000000D")"#,
        "",
    );
}
