use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"overlaps([1..5],[3..8])"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"overlaps([3..8],[1..5])"#, true);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"overlaps([1..5],[6..8])"#, false);
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps(range1: [1..5], range2: [3..8])"#,
        true,
    );
}

#[test]
fn _0005() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps(range2: [3..8], range1: [1..5])"#,
        true,
    );
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps([date("2022-10-01")..date("2022-11-30")], [date("2022-11-16")..date("2022-12-31")])"#,
        true,
    );
}

#[test]
fn _0007() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps([date("2022-10-01")..date("2022-11-30")], [date("2022-12-01")..date("2022-12-31")])"#,
        false,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"overlaps([date("2022-10-01")..date("2022-11-30")], [time("10:11:12")..time("10:11:12")])"#,
        "[core::overlaps] invalid argument type, expected range<date>, actual type is range<time>",
    );
}

#[test]
fn _0009() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps([date and time("2022-10-01T00:00:00")..date and time("2022-11-30T23:59:59")], [date and time("2022-11-16T00:00:00")..date and time("2022-12-31T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"overlaps([date and time("2022-10-01T00:00:00")..date and time("2022-11-30T23:59:59")], [time("00:00:00")..time("23:59:59")])"#,
        "[core::overlaps] invalid argument type, expected range<date and time>, actual type is range<time>",
    );
}

#[test]
fn _0011() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps([time("00:00:01")..time("21:00:00")], [time("12:00:01")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"overlaps([time("00:00:00")..time("23:59:59")], [date and time("2022-10-01T00:00:00")..date and time("2022-11-30T23:59:59")])"#,
        "[core::overlaps] invalid argument type, expected range<time>, actual type is range<date and time>",
    );
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps([duration("P5D")..duration("P10D")], [duration("P1D")..duration("P20D")])"#,
        true,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"overlaps([duration("P5D")..duration("P10D")], [duration("P1Y")..duration("P20Y")])"#,
        "[core::overlaps] invalid argument type, expected range<days and time duration>, actual type is range<years and months duration>",
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps([duration("P5Y")..duration("P10Y")], [duration("P1Y")..duration("P20Y")])"#,
        true,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"overlaps([duration("P1Y")..duration("P20Y")], [duration("P5D")..duration("P10D")])"#,
        "[core::overlaps] invalid argument type, expected range<years and months duration>, actual type is range<days and time duration>",
    );
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#"overlaps()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#"overlaps([1..5],[3..8],[9..12])"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"overlaps(range1: [1..5],r2: [3..8])"#,
        "parameter 'range2' not found",
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"overlaps(r1: [1..5],range2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"overlaps(r1: [1..5], r2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#"overlaps([1..5], [date("2022-12-28")..date("2022-12-31")])"#,
        "[core::overlaps] invalid argument type, expected range<number>, actual type is range<date>",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"overlaps(1, [date("2022-12-28")..date("2022-12-31")])"#,
        "[core::overlaps] invalid argument type, expected range<scalar>, actual type is number",
    );
}
