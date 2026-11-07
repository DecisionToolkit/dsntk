use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"finishes(20,[1..20])"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"finishes(20,[1..20))"#, false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"finishes(19,[1..20])"#, false);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"finishes(21,[1..20])"#, false);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"finishes([1..20],[1..20])"#, true);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), r#"finishes([1..20],[1..20))"#, false);
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), r#"finishes([1..20),[1..20))"#, true);
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), r#"finishes([5..20],[1..20])"#, true);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), r#"finishes((1..20],[1..20])"#, true);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), r#"finishes((1..20],(1..20])"#, true);
}

#[test]
fn _0011() {
    te_bool(false, &scope!(), r#"finishes((1..20),(1..20])"#, false);
}

#[test]
fn _0012() {
    te_bool(false, &scope!(), r#"finishes([1..20],[1..21])"#, false);
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(point: 20,range: [1..20])"#,
        true,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(point: 20,range: [1..20))"#,
        false,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(point: 1,range: [1..20])"#,
        false,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(date("2022-12-10"),[date("2022-12-01")..date("2022-12-10")])"#,
        true,
    );
}

#[test]
fn _0017() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(date("2022-12-10"),[date("2022-12-01")..date("2022-12-10")))"#,
        false,
    );
}

#[test]
fn _0018() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([date("2022-11-03")..date("2022-12-10")],[date("2022-12-01")..date("2022-12-10")])"#,
        true,
    );
}

#[test]
fn _0019() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([date("2022-11-03")..date("2022-12-10")],[date("2022-12-01")..date("2022-12-10")))"#,
        false,
    );
}

#[test]
fn _0020() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(time("12:18:03"),[time("12:00:00")..time("12:18:03")])"#,
        true,
    );
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(time("12:18:03"),[time("12:00:00")..time("12:18:03")))"#,
        false,
    );
}

#[test]
fn _0022() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([time("11:23:420")..time("12:18:03")],[time("12:00:00")..time("12:18:03")])"#,
        true,
    );
}

#[test]
fn _0023() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([time("11:23:420")..time("12:18:03")],[time("12:00:00")..time("12:18:03")))"#,
        false,
    );
}

#[test]
fn _0024() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(date and time("2022-12-01T12:18:03"),[date and time("2022-12-01T12:00:00")..date and time("2022-12-01T12:18:03")])"#,
        true,
    );
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(date and time("2022-12-01T12:18:03"),[date and time("2022-12-01T12:00:00")..date and time("2022-12-01T12:18:03")))"#,
        false,
    );
}

#[test]
fn _0026() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([date and time("2022-12-01T11:00:00")..date and time("2022-12-01T12:18:03")],[date and time("2022-12-01T12:00:00")..date and time("2022-12-01T12:18:03")])"#,
        true,
    );
}

#[test]
fn _0027() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([date and time("2022-12-01T11:00:00")..date and time("2022-12-01T12:18:03")],[date and time("2022-12-01T12:00:00")..date and time("2022-12-01T12:18:03")))"#,
        false,
    );
}

#[test]
fn _0028() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(duration("P2DT1H1M1S"),[duration("P1DT1H1M1S")..duration("P2DT1H1M1S")])"#,
        true,
    );
}

#[test]
fn _0029() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(duration("P2DT1H1M1S"),[duration("P1DT1H1M1S")..duration("P2DT1H1M1S")))"#,
        false,
    );
}

#[test]
fn _0030() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([duration("P1DT0H1M1S")..duration("P2DT1H1M1S")],[duration("P1DT1H1M1S")..duration("P2DT1H1M1S")])"#,
        true,
    );
}

#[test]
fn _0031() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([duration("P1DT0H1M1S")..duration("P2DT1H1M1S")],[duration("P1DT1H1M1S")..duration("P2DT1H1M1S")))"#,
        false,
    );
}

#[test]
fn _0032() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(duration("P2Y"),[duration("P1Y")..duration("P2Y")])"#,
        true,
    );
}

#[test]
fn _0033() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(duration("P2Y"),[duration("P1Y")..duration("P2Y")))"#,
        false,
    );
}

#[test]
fn _0034() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([duration("P3Y")..duration("P5Y")],[duration("P1Y")..duration("P5Y")])"#,
        true,
    );
}

#[test]
fn _0035() {
    te_bool(
        false,
        &scope!(),
        r#"finishes([duration("P3Y")..duration("P5Y")],[duration("P1Y")..duration("P5Y")))"#,
        false,
    );
}

#[test]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#"finishes([1..20],20)"#,
        "[core::finishes] invalid argument type, expected range of scalars, actual type is number",
    );
}

#[test]
fn _0037() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(range: [1..20], point: 20)"#,
        true,
    );
}

#[test]
fn _0038() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(range1: [1..20],range2: [5..20])"#,
        true,
    );
}

#[test]
fn _0039() {
    te_null(
        false,
        &scope!(),
        r#"finishes()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        r#"finishes(20,[1..20],10)"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        r#"finishes(p: 20, range: [1..20])"#,
        "[named::finishes] invalid named parameters",
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#"finishes(point: 20, r: [1..20])"#,
        "[named::finishes] invalid named parameters",
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"finishes(p: 20, r: [1..20])"#,
        "[named::finishes] invalid named parameters",
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#"finishes(range1: [1..20], r2: [1..20])"#,
        "[named::finishes] invalid named parameters",
    );
}

#[test]
fn _0045() {
    te_bool(
        false,
        &scope!(),
        r#"finishes(range2: [1..20], range1: [1..20])"#,
        true,
    );
}

#[test]
fn _0046() {
    te_null(
        false,
        &scope!(),
        r#"finishes(r1: [1..20], range2: [1..20])"#,
        "[named::finishes] invalid named parameters",
    );
}

#[test]
fn _0047() {
    te_null(
        false,
        &scope!(),
        r#"finishes(r1: [1..20], r2: [1..20])"#,
        "[named::finishes] invalid named parameters",
    );
}

#[test]
fn _0048() {
    te_null(
        false,
        &scope!(),
        r#"finishes("alfa", [1..20])"#,
        "[core::finishes] invalid argument type, expected scalar or range of scalars, actual type is string",
    );
}
