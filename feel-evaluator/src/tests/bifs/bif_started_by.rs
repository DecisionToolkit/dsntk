use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"started by([1..20],1)"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"started by((1..20],1)"#, false);
}

#[test]
fn _0003() {
    te_bool(
        false,
        &scope!(),
        r#"started by(range: [1..20],point: 1)"#,
        true,
    );
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"started by(range1: [1..20],range2: [1..10])"#,
        true,
    );
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"started by([1..20],[1..10])"#, true);
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"started by([1..20],["a".."z"])"#,
        "[core::started by] invalid argument type, expected range<number>, actual type is range<string>",
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"started by([1..20],duration("P1D"))"#,
        "[core::started by] invalid argument type, expected number or range<number>, actual type is days and time duration",
    );
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"started by([date("2022-01-01")..date("2022-02-01")],date("2022-01-01"))"#,
        true,
    );
}

#[test]
fn _0009() {
    te_bool(
        false,
        &scope!(),
        r#"started by([date("2022-01-01")..date("2022-02-01")],[date("2022-01-01")..date("2022-01-03")])"#,
        true,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"started by([date("2022-01-01")..date("2022-02-01")],20)"#,
        "[core::started by] invalid argument type, expected date or range<date>, actual type is number",
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"started by([date("2022-01-01")..date("2022-02-01")],[1..10])"#,
        "[core::started by] invalid argument type, expected range<date>, actual type is range<number>",
    );
}

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#"started by([date and time("2022-01-01T00:00:00")..date and time("2022-02-01T23:59:59")],date and time("2022-01-01T00:00:00"))"#,
        true,
    );
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"started by([date and time("2022-01-01T00:00:00")..date and time("2022-02-01T23:59:59")],[date and time("2022-01-01T00:00:00")..date and time("2022-01-01T01:00:00")])"#,
        true,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"started by([date and time("2022-01-01T00:00:00")..date and time("2022-02-01T23:59:59")],1)"#,
        "[core::started by] invalid argument type, expected date and time or range<date and time>, actual type is number",
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"started by([date and time("2022-01-01T00:00:00")..date and time("2022-02-01T23:59:59")],[1..10])"#,
        "[core::started by] invalid argument type, expected range<date and time>, actual type is range<number>",
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"started by([time("00:00:00")..time("23:59:59")],time("00:00:00"))"#,
        true,
    );
}

#[test]
fn _0017() {
    te_bool(
        false,
        &scope!(),
        r#"started by([time("00:00:00")..time("23:59:59")],[time("00:00:00")..time("00:00:01")])"#,
        true,
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#"started by([time("00:00:00")..time("23:59:59")],1)"#,
        "[core::started by] invalid argument type, expected time or range<time>, actual type is number",
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"started by([time("00:00:00")..time("23:59:59")],[1..10])"#,
        "[core::started by] invalid argument type, expected range<time>, actual type is range<number>",
    );
}

#[test]
fn _0020() {
    te_bool(
        false,
        &scope!(),
        r#"started by([duration("P1D")..duration("P10D")],duration("P1D"))"#,
        true,
    );
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"started by([duration("P1D")..duration("P10D")],[duration("P1D")..duration("P2D")])"#,
        true,
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#"started by([duration("P1D")..duration("P10D")],1)"#,
        "[core::started by] invalid argument type, expected days and time duration or range<days and time duration>, actual type is number",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"started by([duration("P1D")..duration("P10D")],[1..10])"#,
        "[core::started by] invalid argument type, expected range<days and time duration>, actual type is range<number>",
    );
}

#[test]
fn _0024() {
    te_bool(
        false,
        &scope!(),
        r#"started by([duration("P1Y")..duration("P10Y")],duration("P1Y"))"#,
        true,
    );
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"started by([duration("P1Y")..duration("P10Y")],[duration("P1Y")..duration("P2Y")])"#,
        true,
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#"started by([duration("P1Y")..duration("P10Y")],1)"#,
        "[core::started by] invalid argument type, expected years and months duration or range<years and months duration>, actual type is number",
    );
}

#[test]
fn _0027() {
    te_null(
        false,
        &scope!(),
        r#"started by([duration("P1Y")..duration("P10Y")],[1..10])"#,
        "[core::started by] invalid argument type, expected range<years and months duration>, actual type is range<number>",
    );
}

#[test]
fn _0028() {
    te_null(
        false,
        &scope!(),
        r#"started by()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0029() {
    te_null(
        false,
        &scope!(),
        r#"started by([1..20],20,10)"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0030() {
    te_null(
        false,
        &scope!(),
        r#"started by(range: [1..20],p: 20)"#,
        "[named::started by] invalid named parameters",
    );
}

#[test]
fn _0031() {
    te_null(
        false,
        &scope!(),
        r#"started by(r: [1..20],point: 20)"#,
        "[named::started by] invalid named parameters",
    );
}

#[test]
fn _0032() {
    te_null(
        false,
        &scope!(),
        r#"started by(r: [1..20],p: 20)"#,
        "[named::started by] invalid named parameters",
    );
}

#[test]
fn _0033() {
    te_null(
        false,
        &scope!(),
        r#"started by(range1: [1..20],r2: [1..20])"#,
        "[named::started by] invalid named parameters",
    );
}

#[test]
fn _0034() {
    te_null(
        false,
        &scope!(),
        r#"started by(r1: [1..20],range2: [1..20])"#,
        "[named::started by] invalid named parameters",
    );
}

#[test]
fn _0035() {
    te_null(
        false,
        &scope!(),
        r#"started by(r1: [1..20],r2: [1..20])"#,
        "[named::started by] invalid named parameters",
    );
}

#[test]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#"started by(["a".."z"],[1..20])"#,
        "[core::started by] invalid argument type, expected scalar or range<scalar>, actual type is range<string>",
    );
}

#[test]
fn _0037() {
    te_null(
        false,
        &scope!(),
        r#"started by("a",[1..20])"#,
        "[core::started by] invalid argument type, expected range<scalar>, actual type is string",
    );
}
