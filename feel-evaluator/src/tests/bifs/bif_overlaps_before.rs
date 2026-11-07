use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"overlaps before([1..5],[3..8])"#, true);
}

#[test]
fn _0002() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before([6..12],[3..8])"#,
        false,
    );
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before([1..5],[duration("P3D")..duration("P8D")])"#,
        "[core::overlaps before] invalid argument type, expected range<number>, actual type is range<days and time duration>",
    );
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before(range1: [1..5], range2: [3..8])"#,
        true,
    );
}

#[test]
fn _0005() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before(range2: [3..8], range1: [1..5])"#,
        true,
    );
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before([date("2022-11-01")..date("2022-12-11")], [date("2022-11-15")..date("2022-12-18")])"#,
        true,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before([date("2022-11-01")..date("2022-12-11")], [1..10])"#,
        "[core::overlaps before] invalid argument type, expected range<date>, actual type is range<number>",
    );
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before([date and time("2022-11-01T00:00:00")..date and time("2022-12-11T00:00:00")], [date and time("2022-11-15T00:00:00")..date and time("2022-12-18T00:00:00")])"#,
        true,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before([date and time("2022-11-01T00:00:00")..date and time("2022-12-11T00:00:00")], [1..10])"#,
        "[core::overlaps before] invalid argument type, expected range<date and time>, actual type is range<number>",
    );
}

#[test]
fn _0010() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before([time("00:00:00")..time("15:00:00")], [time("12:00:00")..time("23:00:00")])"#,
        true,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before([time("00:00:00")..time("15:00:00")], [1..10])"#,
        "[core::overlaps before] invalid argument type, expected range<time>, actual type is range<number>",
    );
}

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before([duration("P1D")..duration("P10D")], [duration("P3D")..duration("P18D")])"#,
        true,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before([duration("P1D")..duration("P10D")], [1..10])"#,
        "[core::overlaps before] invalid argument type, expected range<days and time duration>, actual type is range<number>",
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#"overlaps before([duration("P1Y")..duration("P10Y")], [duration("P3Y")..duration("P18Y")])"#,
        true,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before([duration("P1Y")..duration("P10Y")], [1..10])"#,
        "[core::overlaps before] invalid argument type, expected range<years and months duration>, actual type is range<number>",
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before([1..5],[3..8],[9..12])"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before(range1: [1..5],r2: [3..8])"#,
        "parameter 'range2' not found",
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before(r1: [1..5],range2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before(r1: [1..5], r2: [3..8])"#,
        "parameter 'range1' not found",
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"overlaps before(1, [3..8])"#,
        "[core::overlaps] invalid argument type, expected range<scalar>, actual type is number",
    );
}
