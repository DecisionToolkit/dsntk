use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"is(100,100)"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"is(100,"abc")"#, false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"is(100,101)"#, false);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"is("abc","abc")"#, true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"is("abc",false)"#, false);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), r#"is("abc","abcd")"#, false);
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), r#"is(true,true)"#, true);
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), r#"is(false,false)"#, true);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), r#"is(true,false)"#, false);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), r#"is(true,1)"#, false);
}

#[test]
fn _0011() {
    te_bool(
        false,
        &scope!(),
        r#"is(date(2019,7,12),date("2019-07-12"))"#,
        true,
    );
}

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#"is(value1:date(2019,7,12),value2:date("2019-07-12"))"#,
        true,
    );
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"is(date(2019,7,11),date("2019-07-12"))"#,
        false,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#"is(value1:date(2019,7,11),value2:date("2019-07-12"))"#,
        false,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#"is(date and time("2019-07-12T10:12:13"),date and time("2019-07-12T10:12:13"))"#,
        true,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"is(date and time("2019-07-12T10:12:13"),date and time("2019-07-12T10:12:14"))"#,
        false,
    );
}

#[test]
fn _0017() {
    te_bool(
        false,
        &scope!(),
        r#"is(date and time("2019-07-12T10:12:13"),10)"#,
        false,
    );
}

#[test]
fn _0018() {
    te_bool(
        false,
        &scope!(),
        r#"is(time(12,13,14),time("12:13:14"))"#,
        true,
    );
}

#[test]
fn _0019() {
    te_bool(
        false,
        &scope!(),
        r#"is(value1:time(12,13,14),value2:time("12:13:14"))"#,
        true,
    );
}

#[test]
fn _0020() {
    te_bool(
        false,
        &scope!(),
        r#"is(time(12,13,15),time("12:13:14"))"#,
        false,
    );
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"is(value1:time(12,13,15),value2:time("12:13:14"))"#,
        false,
    );
}

#[test]
fn _0022() {
    te_bool(
        false,
        &scope!(),
        r#"is(time("23:00:50z"),time("23:00:50"))"#,
        false,
    );
}

#[test]
fn _0023() {
    te_bool(
        false,
        &scope!(),
        r#"is(time("23:00:50z"),time("23:00:50Z"))"#,
        true,
    );
}

#[test]
fn _0024() {
    te_bool(
        false,
        &scope!(),
        r#"is(time("23:00:50z"),time("23:00:50@Etc/UTC"))"#,
        true,
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"is(v1:time(12,13,15),value2:time("12:13:14"))"#,
        "parameter 'value1' not found",
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#"is(value1:time(12,13,15),v2:time("12:13:14"))"#,
        "parameter 'value2' not found",
    );
}

#[test]
fn _0027() {
    te_null(
        false,
        &scope!(),
        r#"is()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0028() {
    te_bool(false, &scope!(), r#"is(time(12,13,15))"#, false);
}

#[test]
fn _0029() {
    te_null(
        false,
        &scope!(),
        r#"is(time(12,13,15),time(12,13,15),time(12,13,15))"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0030() {
    te_bool(false, &scope!(), r#"is(10,time(12,13,15))"#, false);
}

#[test]
fn _0031() {
    te_bool(false, &scope!(), r#"is(time(12,13,15), 10)"#, false);
}

#[test]
fn _0032() {
    te_bool(false, &scope!(), r#"is(date(2012,11,15), 10)"#, false);
}

#[test]
fn _0033() {
    te_bool(false, &scope!(), r#"is(date(2012,11,15))"#, false);
}

#[test]
fn _0034() {
    te_bool(false, &scope!(), r#"is(value1: date(2012,11,15))"#, false);
}

#[test]
fn _0035() {
    te_null(
        false,
        &scope!(),
        r#"is(value1:time(12,13,14), value2:time("12:13:15"), value3:time("12:13:16"))"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#"is()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0037() {
    te_bool(
        false,
        &scope!(),
        r#"is(duration("P1D"),duration("P1D"))"#,
        true,
    );
}

#[test]
fn _0038() {
    te_bool(
        false,
        &scope!(),
        r#"is(duration("P1D"),duration("P1DT1S"))"#,
        false,
    );
}

#[test]
fn _0039() {
    te_bool(false, &scope!(), r#"is(duration("P1D"),10)"#, false);
}

#[test]
fn _0040() {
    te_bool(
        false,
        &scope!(),
        r#"is(duration("P1Y"),duration("P1Y"))"#,
        true,
    );
}

#[test]
fn _0041() {
    te_bool(
        false,
        &scope!(),
        r#"is(duration("P1Y"),duration("P1Y1M"))"#,
        false,
    );
}

#[test]
fn _0042() {
    te_bool(
        false,
        &scope!(),
        r#"is(duration("P1Y"),duration("P1D"))"#,
        false,
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"is(Null,duration("P1D"))"#,
        "[core::is] invalid argument type, expected scalar, actual type is Null",
    );
}

#[test]
fn _0044() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00@Australia/Melbourne", @"2021-04-02T23:00:00+11:00")"#,
        false,
    );
}

#[test]
fn _0045() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00Z", @"2021-04-02T23:00:00Z")"#,
        true,
    );
}

#[test]
fn _0046() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00Z", @"2021-04-02T23:00:00@Australia/Melbourne")"#,
        false,
    );
}

#[test]
fn _0047() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00Z", @"2021-04-02T23:00:00-01:00:00")"#,
        false,
    );
}

#[test]
fn _0048() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00+01:00:00", @"2021-04-02T23:00:00+01:00:00")"#,
        true,
    );
}

#[test]
fn _0049() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00+01:00:00", @"2021-04-02T23:00:00-01:00:00")"#,
        false,
    );
}

#[test]
fn _0050() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00@Europe/Warsaw", @"2021-04-02T23:00:00@Europe/Warsaw")"#,
        true,
    );
}

#[test]
fn _0051() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00@Europe/Warsaw", @"2021-04-02T23:00:00@Australia/Melbourne")"#,
        false,
    );
}

#[test]
fn _0052() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"2021-04-02T23:00:00@Australia/Melbourne", @"2021-04-02T23:00:00")"#,
        false,
    );
}

#[test]
fn _0053() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"262142-12-28T23:00:00Z", @"262142-12-28T23:00:00Z")"#,
        true,
    );
}

#[test]
fn _0054() {
    te_bool(
        false,
        &scope!(),
        r#"is(@"-262143-01-01T23:00:00Z", @"-262143-01-01T23:00:00Z")"#,
        true,
    );
}

#[test]
fn _0055() {
    te_null(
        false,
        &scope!(),
        r#"is(@"999999999-04-02T23:00:00Z", @"999999999-04-02T23:00:00Z")"#,
        "[core::is] invalid argument type, expected scalar, actual type is Null",
    );
}
