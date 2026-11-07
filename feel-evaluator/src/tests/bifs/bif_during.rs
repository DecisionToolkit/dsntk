use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"during(10,[1..20])"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"during(1,[1..20])"#, true);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"during(1,(1..20])"#, false);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"during((1..5),(1..10])"#, true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"during((1..10),[1..10])"#, true);
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"during(point: 10, range: [1..20])"#,
        true,
    );
}

#[test]
fn _0007() {
    te_bool(
        false,
        &scope!(),
        r#"during(range: [1..20], point: 10)"#,
        true,
    );
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"during(date("2011-06-16"),[date("2011-01-01")..date("2011-12-31")])"#,
        true,
    );
}

#[test]
fn _0009() {
    te_bool(
        false,
        &scope!(),
        r#"during(time("12:13:14"),[time("00:00:00")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0010() {
    te_bool(
        false,
        &scope!(),
        r#"during(date and time("2011-06-16T12:13:14"),[date and time("2011-01-01T00:00:00")..date and time("2011-12-31T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0011() {
    te_bool(
        false,
        &scope!(),
        r#"during(duration("P1DT12H"),[duration("P1D")..duration("P2D")])"#,
        true,
    );
}

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#"during(duration("P1Y6M"),[duration("P1Y")..duration("P2Y")])"#,
        true,
    );
}

#[test]
fn _0013() {
    te_bool(false, &scope!(), r#"during(10,[10..20])"#, true);
}

#[test]
fn _0014() {
    te_bool(false, &scope!(), r#"during(20,[10..20])"#, true);
}

#[test]
fn _0015() {
    te_bool(false, &scope!(), r#"during(10,[20..10])"#, false);
}

#[test]
fn _0016() {
    te_bool(false, &scope!(), r#"during([1..20],[1..20])"#, true);
}

#[test]
fn _0017() {
    te_bool(false, &scope!(), r#"during((2..19),[1..20])"#, true);
}

#[test]
fn _0018() {
    te_bool(false, &scope!(), r#"during([2..19],(1..20))"#, true);
}

#[test]
fn _0019() {
    te_bool(
        false,
        &scope!(),
        r#"during(range1: [2..19], range2: (1..20))"#,
        true,
    );
}

#[test]
fn _0020() {
    te_bool(false, &scope!(), r#"during([0..20],[1..20])"#, false);
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"during(range1: [0..20],range2: [1..20])"#,
        false,
    );
}

#[test]
fn _0022() {
    te_bool(false, &scope!(), r#"during([1..21],[1..20])"#, false);
}

#[test]
fn _0023() {
    te_bool(false, &scope!(), r#"during([0..21],[1..20])"#, false);
}

#[test]
fn _0024() {
    te_bool(false, &scope!(), r#"during([1..12],[13..20])"#, false);
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"during([date("2011-02-01")..date("2011-11-30")],[date("2011-01-01")..date("2011-12-31")])"#,
        true,
    );
}

#[test]
fn _0026() {
    te_bool(
        false,
        &scope!(),
        r#"during([date("2010-12-31")..date("2011-11-30")],[date("2011-01-01")..date("2011-12-31")])"#,
        false,
    );
}

#[test]
fn _0027() {
    te_bool(
        false,
        &scope!(),
        r#"during([date and time("2011-02-01T00:00:01")..date and time("2011-12-31T23:59:58")],[date and time("2011-01-01T00:00:00")..date and time("2011-12-31T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0028() {
    te_bool(
        false,
        &scope!(),
        r#"during([date and time("2011-01-01T01:01:00")..date and time("2011-12-31T23:59:59")],[date and time("2011-01-01T01:01:01")..date and time("2011-12-31T23:59:59")])"#,
        false,
    );
}

#[test]
fn _0029() {
    te_bool(
        false,
        &scope!(),
        r#"during([time("00:00:01")..time("23:59:58")],[time("00:00:00")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0030() {
    te_bool(
        false,
        &scope!(),
        r#"during([time("00:00:00")..time("23:59:59")],[time("00:00:00")..time("23:59:58")])"#,
        false,
    );
}

#[test]
fn _0031() {
    te_bool(
        false,
        &scope!(),
        r#"during([duration("P1DT0H0M1S")..duration("P2DT0H0M0S")],[duration("P1DT0H0M0S")..duration("P2DT0H0M0S")])"#,
        true,
    );
}

#[test]
fn _0032() {
    te_bool(
        false,
        &scope!(),
        r#"during([duration("P1DT0H0M0S")..duration("P2DT0H0M0S")],[duration("P1DT0H0M1S")..duration("P2DT0H0M0S")])"#,
        false,
    );
}

#[test]
fn _0033() {
    te_bool(
        false,
        &scope!(),
        r#"during([duration("P2Y")..duration("P3Y")],[duration("P1Y")..duration("P4Y")])"#,
        true,
    );
}

#[test]
fn _0034() {
    te_bool(
        false,
        &scope!(),
        r#"during([duration("P2Y")..duration("P4Y")],[duration("P1Y")..duration("P2Y")])"#,
        false,
    );
}

#[test]
fn _0035() {
    te_bool(
        false,
        &scope!(),
        r#"during([duration("P1Y")..duration("P4Y1M")],[duration("P1Y")..duration("P4Y")])"#,
        false,
    );
}

#[test]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#"during()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0037() {
    te_null(
        false,
        &scope!(),
        r#"during(10,11,12)"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0038() {
    te_null(
        false,
        &scope!(),
        r#"during(p: 150, range: (100..200))"#,
        "[named::during] invalid named parameters",
    );
}

#[test]
fn _0039() {
    te_null(
        false,
        &scope!(),
        r#"during(point: 150, r: (100..200))"#,
        "[named::during] invalid named parameters",
    );
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        r#"during(p: 150, r: (100..200))"#,
        "[named::during] invalid named parameters",
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        r#"during(r1: [80..102], range2: (100..200))"#,
        "[named::during] invalid named parameters",
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#"during(range1: [80..102], r2: (100..200))"#,
        "[named::during] invalid named parameters",
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"during(r1: [80..102], r2: (100..200))"#,
        "[named::during] invalid named parameters",
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#"during(true,(100..200))"#,
        "[core::during] invalid argument type, expected scalar or range of scalars, actual type is boolean",
    );
}
