use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"met by([10..20],[1..10])"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"met by([10..20],[1..10))"#, false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"met by((10..20],[1..10])"#, false);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"met by([9..20],[1..10])"#, false);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"met by([11..20],[1..10])"#, false);
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"met by(range1: [10..20], range2: [1..10])"#,
        true,
    );
}

#[test]
fn _0007() {
    te_bool(
        false,
        &scope!(),
        r#"met by(range2: [10..20], range1: [1..10])"#,
        false,
    );
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"met by(range2: [1..10], range1: [10..20])"#,
        true,
    );
}

#[test]
fn _0009() {
    te_bool(
        false,
        &scope!(),
        r#"met by([date("2022-12-31")..date("2023-01-15")],[date("2022-12-10")..date("2022-12-31")])"#,
        true,
    );
}

#[test]
fn _0010() {
    te_bool(
        false,
        &scope!(),
        r#"met by((date("2022-12-31")..date("2023-01-15")),[date("2022-12-10")..date("2022-12-31")])"#,
        false,
    );
}

#[test]
fn _0011() {
    te_bool(
        false,
        &scope!(),
        r#"met by([date and time("2022-12-31T23:59:59")..date and time("2022-12-10T00:00:00")],[date and time("2022-12-10T00:00:00")..date and time("2022-12-31T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#"met by([date and time("2022-12-31T23:59:58")..date and time("2022-12-10T00:00:00")],[date and time("2022-12-10T00:00:00")..date and time("2022-12-31T23:59:59")])"#,
        false,
    );
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"met by([time("12:00:00")..time("22:00:00")],[time("10:00:00")..time("12:00:00")])"#,
        true,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#"met by([time("11:30:00")..time("22:00:00")],[time("10:00:00")..time("12:00:00")])"#,
        false,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#"met by([duration("P12D")..duration("P15D")],[duration("P10D")..duration("P12D")])"#,
        true,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"met by([duration("P12DT1H")..duration("P15D")],[duration("P11D")..duration("P12D")])"#,
        false,
    );
}

#[test]
fn _0017() {
    te_bool(
        false,
        &scope!(),
        r#"met by([duration("P12Y")..duration("P15Y")],[duration("P10Y")..duration("P12Y")])"#,
        true,
    );
}

#[test]
fn _0018() {
    te_bool(
        false,
        &scope!(),
        r#"met by([duration("P1Y")..duration("P10Y")],(duration("P10Y")..duration("P12Y")])"#,
        false,
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"met by()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"met by([1..20],20,10)"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"met by(range1: [1..5],r2: [3..8])"#,
        "parameter 'range2' not found",
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#"met by(r1: [1..5],range2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"met by(r1: [1..5], r2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0024() {
    te_null(
        false,
        &scope!(),
        r#"met by(time("00:00:00"),[time("11:59:59")..time("23:59:59")])"#,
        "[core::meets] invalid argument type, expected range of scalars, actual type is time",
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"met by([time("11:59:59")..time("23:59:59")],time("00:00:00"))"#,
        "[core::meets] invalid argument type, expected range of scalars, actual type is time",
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#"met by(["a".."b"],[time("11:59:59")..time("23:59:59")])"#,
        "[core::meets] invalid argument type, expected range of scalars, actual type is range<string>",
    );
}

#[test]
fn _0027() {
    te_null(
        false,
        &scope!(),
        r#"met by([time("11:59:59")..time("23:59:59")],["a".."b"])"#,
        "[core::meets] invalid argument type, expected range<time>, actual type is range<string>",
    );
}

#[test]
fn _0028() {
    te_null(
        false,
        &scope!(),
        r#"met by([date("2022-12-31")..date("2023-01-15")],[1..12])"#,
        "[core::meets] invalid argument type, expected range<date>, actual type is range<number>",
    );
}

#[test]
fn _0029() {
    te_null(
        false,
        &scope!(),
        r#"met by([date and time("2022-12-31T23:59:59")..date and time("2022-12-10T00:00:00")],[1..2])"#,
        "[core::meets] invalid argument type, expected range<date and time>, actual type is range<number>",
    );
}

#[test]
fn _0030() {
    te_null(
        false,
        &scope!(),
        r#"met by([time("12:00:00")..time("22:00:00")],[1..2])"#,
        "[core::meets] invalid argument type, expected range<time>, actual type is range<number>",
    );
}

#[test]
fn _0031() {
    te_null(
        false,
        &scope!(),
        r#"met by([10..20],[duration("P1D")..duration("P2D")])"#,
        "[core::meets] invalid argument type, expected range<number>, actual type is range<days and time duration>",
    );
}

#[test]
fn _0032() {
    te_null(
        false,
        &scope!(),
        r#"met by([duration("P1D")..duration("P2D")],[1..2])"#,
        "[core::meets] invalid argument type, expected range<days and time duration>, actual type is range<number>",
    );
}

#[test]
fn _0033() {
    te_null(
        false,
        &scope!(),
        r#"met by([duration("P1Y")..duration("P2Y")],[1..2])"#,
        "[core::meets] invalid argument type, expected range<years and months duration>, actual type is range<number>",
    );
}
