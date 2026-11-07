use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"includes([1..20],20)"#, true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"includes([1..20),20)"#, false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"includes([1..20],19)"#, true);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"includes([1..20],1)"#, true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"includes((1..20],1)"#, false);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), r#"includes([1..20],2)"#, true);
}

#[test]
fn _0007() {
    te_bool(
        false,
        &scope!(),
        r#"includes(range: [1..20], point: 20)"#,
        true,
    );
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), r#"includes([1..20],[1..20])"#, true);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), r#"includes((1..10],(1..5])"#, true);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), r#"includes([1..20],[5..15])"#, true);
}

#[test]
fn _0011() {
    te_bool(false, &scope!(), r#"includes([1..20],[1..21])"#, false);
}

#[test]
fn _0012() {
    te_bool(false, &scope!(), r#"includes([1..20],[0..20])"#, false);
}

#[test]
fn _0013() {
    te_bool(false, &scope!(), r#"includes([1..20],[1..20))"#, true);
}

#[test]
fn _0014() {
    te_bool(false, &scope!(), r#"includes([1..20],(1..20])"#, true);
}

#[test]
fn _0015() {
    te_bool(false, &scope!(), r#"includes((1..20],[1..20])"#, false);
}

#[test]
fn _0016() {
    te_bool(false, &scope!(), r#"includes([1..20),[1..20])"#, false);
}

#[test]
fn _0017() {
    te_bool(
        false,
        &scope!(),
        r#"includes(range1: [1..20], range2: [1..20])"#,
        true,
    );
}

//

#[test]
fn _0018() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date("2022-12-11")..date("2022-12-31")],date("2022-12-11"))"#,
        true,
    );
}

#[test]
fn _0019() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date("2022-12-11")..date("2022-12-30")],date("2022-12-31"))"#,
        false,
    );
}

#[test]
fn _0020() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date("2022-12-11")..date("2022-12-31")],[date("2022-12-12")..date("2022-12-30")])"#,
        true,
    );
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date("2022-12-11")..date("2022-12-31")),[date("2022-12-11")..date("2022-12-31")])"#,
        false,
    );
}

//

#[test]
fn _0022() {
    te_bool(
        false,
        &scope!(),
        r#"includes([time("00:00:00")..time("23:59:59")],time("00:00:00"))"#,
        true,
    );
}

#[test]
fn _0023() {
    te_bool(
        false,
        &scope!(),
        r#"includes([time("01:00:00")..time("23:59:59")],time("00:59:59"))"#,
        false,
    );
}

#[test]
fn _0024() {
    te_bool(
        false,
        &scope!(),
        r#"includes([time("00:00:00")..time("23:59:59")],[time("00:00:00")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"includes([time("00:00:00")..time("23:59:00")),[time("00:00:00")..time("23:59:01")])"#,
        false,
    );
}

//

#[test]
fn _0026() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date and time("2022-12-01T00:00:00")..date and time("2022-12-31T23:59:59")],date and time("2022-12-01T00:00:00"))"#,
        true,
    );
}

#[test]
fn _0027() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date and time("2022-12-11T00:00:00")..date and time("2022-12-30T23:59:59")],date and time("2022-12-31T00:00:00"))"#,
        false,
    );
}

#[test]
fn _0028() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date and time("2022-12-11T00:00:00")..date and time("2022-12-31T23:59:59")],[date and time("2022-12-11T00:00:00")..date and time("2022-12-30T00:00:00")])"#,
        true,
    );
}

#[test]
fn _0029() {
    te_bool(
        false,
        &scope!(),
        r#"includes([date and time("2022-12-11T00:00:00")..date and time("2022-12-31T23:59:00")),[date and time("2022-12-11T00:00:00")..date and time("2022-12-31T23:59:01")])"#,
        false,
    );
}

//

#[test]
fn _0030() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1D")..duration("P10D")],duration("P5D"))"#,
        true,
    );
}

#[test]
fn _0031() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1D")..duration("P2D")],duration("P2DT1H"))"#,
        false,
    );
}

#[test]
fn _0032() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1D")..duration("P3D")],[duration("P1D")..duration("P3D")])"#,
        true,
    );
}

#[test]
fn _0033() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1D")..duration("P3D")),[duration("P1D")..duration("P3DT1M")])"#,
        false,
    );
}

#[test]
fn _0034() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1Y")..duration("P10Y")],duration("P5Y"))"#,
        true,
    );
}

#[test]
fn _0035() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1Y")..duration("P2Y")],duration("P2Y1M"))"#,
        false,
    );
}

#[test]
fn _0036() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1Y")..duration("P3Y")],[duration("P1Y")..duration("P3Y")])"#,
        true,
    );
}

#[test]
fn _0037() {
    te_bool(
        false,
        &scope!(),
        r#"includes([duration("P1Y")..duration("P3Y")),[duration("P1Y")..duration("P3Y1M")])"#,
        false,
    );
}

#[test]
fn _0038() {
    te_null(
        false,
        &scope!(),
        r#"includes()"#,
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0039() {
    te_null(
        false,
        &scope!(),
        r#"includes([1..20],20,10)"#,
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        r#"includes(range: [1..20],p: 20)"#,
        "[named::includes] invalid named parameters",
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        r#"includes(r: [1..20],point: 20)"#,
        "[named::includes] invalid named parameters",
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#"includes(r: [1..20],p: 20)"#,
        "[named::includes] invalid named parameters",
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"includes(range1: [1..20],r2: [1..20])"#,
        "[named::includes] invalid named parameters",
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#"includes(r1: [1..20],range2: [1..20])"#,
        "[named::includes] invalid named parameters",
    );
}

#[test]
fn _0045() {
    te_null(
        false,
        &scope!(),
        r#"includes(r1: [1..20],r2: [1..20])"#,
        "[named::includes] invalid named parameters",
    );
}

#[test]
fn _0046() {
    te_null(
        false,
        &scope!(),
        r#"includes([1..20],"a")"#,
        "[core::includes] invalid argument type, expected number or range<number>, actual type is string",
    );
}

#[test]
fn _0047() {
    te_null(
        false,
        &scope!(),
        r#"includes([1..20],["a".."b"])"#,
        "[core::includes] invalid argument type, expected range<number>, actual type is range<string>",
    );
}

#[test]
fn _0048() {
    te_null(
        false,
        &scope!(),
        r#"includes(["a".."b"],[1..20])"#,
        "[core::includes] invalid argument type, expected scalar or range<scalar>, actual type is range<string>",
    );
}

#[test]
fn _0049() {
    te_null(
        false,
        &scope!(),
        r#"includes("a",[1..20])"#,
        "[core::includes] invalid argument type, expected range<scalar>, actual type is string",
    );
}

#[test]
fn _0050() {
    te_null(
        false,
        &scope!(),
        r#"includes([date("2022-12-01")..date("2022-12-31")],1)"#,
        "[core::includes] invalid argument type, expected date or range<date>, actual type is number",
    );
}

#[test]
fn _0051() {
    te_null(
        false,
        &scope!(),
        r#"includes([date("2022-12-01")..date("2022-12-31")],[1..31])"#,
        "[core::includes] invalid argument type, expected range<date>, actual type is range<number>",
    );
}

#[test]
fn _0052() {
    te_null(
        false,
        &scope!(),
        r#"includes([time("00:00:00")..time("23:59:59")],1)"#,
        "[core::includes] invalid argument type, expected time or range<time>, actual type is number",
    );
}

#[test]
fn _0053() {
    te_null(
        false,
        &scope!(),
        r#"includes([time("00:00:00")..time("23:59:59")],[1..31])"#,
        "[core::includes] invalid argument type, expected range<time>, actual type is range<number>",
    );
}

#[test]
fn _0054() {
    te_null(
        false,
        &scope!(),
        r#"includes([date and time("2022-12-01T00:00:00")..date and time("2022-12-31T23:59:59")],1)"#,
        "[core::includes] invalid argument type, expected date and time or range<date and time>, actual type is number",
    );
}

#[test]
fn _0055() {
    te_null(
        false,
        &scope!(),
        r#"includes([date and time("2022-12-01T00:00:00")..date and time("2022-12-31T23:59:59")],[1..31])"#,
        "[core::includes] invalid argument type, expected range<date and time>, actual type is range<number>",
    );
}

#[test]
fn _0056() {
    te_null(
        false,
        &scope!(),
        r#"includes([duration("P1D")..duration("P10D")],1)"#,
        "[core::includes] invalid argument type, expected days and time duration or range<days and time duration>, actual type is number",
    );
}

#[test]
fn _0057() {
    te_null(
        false,
        &scope!(),
        r#"includes([duration("P1D")..duration("P10D")],[1..31])"#,
        "[core::includes] invalid argument type, expected range<days and time duration>, actual type is range<number>",
    );
}

#[test]
fn _0058() {
    te_null(
        false,
        &scope!(),
        r#"includes([duration("P1Y")..duration("P10Y")],1)"#,
        "[core::includes] invalid argument type, expected years and months duration or range<years and months duration>, actual type is number",
    );
}

#[test]
fn _0059() {
    te_null(
        false,
        &scope!(),
        r#"includes([duration("P1Y")..duration("P10Y")],[1..31])"#,
        "[core::includes] invalid argument type, expected range<years and months duration>, actual type is range<number>",
    );
}
