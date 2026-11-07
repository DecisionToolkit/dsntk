use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"meets([1..10],[10..20])"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"meets([1..10),[10..20])"#, false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"meets([1..10],(10..20])"#, false);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"meets([1..10],[9..20])"#, false);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"meets([1..10],[11..20])"#, false);
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"meets(range1: [1..10], range2: [10..20])"#,
        true,
    );
}

#[test]
fn _0007() {
    te_bool(
        false,
        &scope!(),
        r#"meets(range2: [1..10], range1: [10..20])"#,
        false,
    );
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"meets(range2: [10..20], range1: [1..10])"#,
        true,
    );
}

#[test]
fn _0009() {
    te_bool(
        false,
        &scope!(),
        r#"meets([date("2022-12-01")..date("2022-12-10")],[date("2022-12-10")..date("2022-12-31")])"#,
        true,
    );
}

#[test]
fn _0010() {
    te_bool(
        false,
        &scope!(),
        r#"meets([date("2022-12-01")..date("2022-12-10")),[date("2022-12-10")..date("2022-12-31")])"#,
        false,
    );
}

#[test]
fn _0011() {
    te_bool(
        false,
        &scope!(),
        r#"meets([date and time("2022-12-01T00:00:00")..date and time("2022-12-10T00:00:00")],[date and time("2022-12-10T00:00:00")..date and time("2022-12-31T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#"meets([date and time("2022-12-01T00:00:00")..date and time("2022-12-10T00:00:01")],[date and time("2022-12-10T00:00:00")..date and time("2022-12-31T23:59:59")])"#,
        false,
    );
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"meets([time("00:00:00")..time("12:00:00")],[time("12:00:00")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#"meets([time("00:00:00")..time("12:00:00")],[time("11:59:59")..time("23:59:59")])"#,
        false,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#"meets([duration("P1D")..duration("P10D")],[duration("P10D")..duration("P12D")])"#,
        true,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"meets([duration("P1D")..duration("P10D")],[duration("P11D")..duration("P12D")])"#,
        false,
    );
}

#[test]
fn _0017() {
    te_bool(
        false,
        &scope!(),
        r#"meets([duration("P1Y")..duration("P10Y")],[duration("P10Y")..duration("P12Y")])"#,
        true,
    );
}

#[test]
fn _0018() {
    te_bool(
        false,
        &scope!(),
        r#"meets([duration("P1Y")..duration("P10Y")],(duration("P10Y")..duration("P12Y")])"#,
        false,
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"meets()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"meets(20,[1..20],10)"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"meets(range1: [1..5],r2: [3..8])"#,
        "parameter 'range2' not found",
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#"meets(r1: [1..5],range2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"meets(r1: [1..5], r2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0024() {
    te_null(
        false,
        &scope!(),
        r#"meets(time("00:00:00"),[time("11:59:59")..time("23:59:59")])"#,
        "[core::meets] invalid argument type, expected range of scalars, actual type is time",
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"meets(["a".."b"],[time("11:59:59")..time("23:59:59")])"#,
        "[core::meets] invalid argument type, expected range of scalars, actual type is range<string>",
    );
}
