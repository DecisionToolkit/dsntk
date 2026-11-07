use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"coincides(10,10)"#, true);
}

#[test]
fn _0002() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(date("2021-02-14"),date("2021-02-14"))"#,
        true,
    );
}

#[test]
fn _0003() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(time("12:13:14"),time("12:13:14"))"#,
        true,
    );
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(date and time("2021-02-14T12:13:14"),date and time("2021-02-14T12:13:14"))"#,
        true,
    );
}

#[test]
fn _0005() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(duration("P1DT3H2M5S"),duration("P1DT3H2M5S"))"#,
        true,
    );
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(duration("P1Y3M"),duration("P1Y3M"))"#,
        true,
    );
}

#[test]
fn _0007() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(point1: 10, point2: 10)"#,
        true,
    );
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(point2: 10, point1: 10)"#,
        true,
    );
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), r#"coincides(10,11)"#, false);
}

#[test]
fn _0010() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(date("2021-02-14"),date("2021-02-13"))"#,
        false,
    );
}

#[test]
fn _0011() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(time("12:13:14"),time("12:13:15"))"#,
        false,
    );
}

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(date and time("2021-02-14T12:13:14"),date and time("2021-02-14T12:13:15"))"#,
        false,
    );
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(duration("P1DT3H2M5S"),duration("P1DT3H2M6S"))"#,
        false,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(duration("P1Y3M"),duration("P1Y4M"))"#,
        false,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(point1: 10,point2: 11)"#,
        false,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(point2: 10,point1: 11)"#,
        false,
    );
}

#[test]
fn _0017() {
    te_bool(false, &scope!(), r#"coincides([1..10],[1..10])"#, true);
}

#[test]
fn _0018() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([date("2021-01-01")..date("2021-12-31")],[date("2021-01-01")..date("2021-12-31")])"#,
        true,
    );
}

#[test]
fn _0019() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([time("01:00:00")..time("03:00:00")],[time("01:00:00")..time("03:00:00")])"#,
        true,
    );
}

#[test]
fn _0020() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([date and time("2021-01-01T01:00:00")..date and time("2021-12-31T03:00:00")],[date and time("2021-01-01T01:00:00")..date and time("2021-12-31T03:00:00")])"#,
        true,
    );
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([duration("P1D")..duration("P3D")],[duration("P1D")..duration("P3D")])"#,
        true,
    );
}

#[test]
fn _0022() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([duration("P1Y")..duration("P3Y")],[duration("P1Y")..duration("P3Y")])"#,
        true,
    );
}

#[test]
fn _0023() {
    te_bool(false, &scope!(), r#"coincides([1..10],[2..10])"#, false);
}

#[test]
fn _0024() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([date("2021-01-01")..date("2021-12-30")],[date("2021-01-01")..date("2021-12-31")])"#,
        false,
    );
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([time("01:00:00")..time("03:00:01")],[time("01:00:00")..time("03:00:00")])"#,
        false,
    );
}

#[test]
fn _0026() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([date and time("2021-01-01T01:00:00")..date and time("2021-12-31T03:00:01")],[date and time("2021-01-01T01:00:00")..date and time("2021-12-31T03:00:00")])"#,
        false,
    );
}

#[test]
fn _0027() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([duration("P1D")..duration("P4D")],[duration("P1D")..duration("P3D")])"#,
        false,
    );
}

#[test]
fn _0028() {
    te_bool(
        false,
        &scope!(),
        r#"coincides([duration("P1Y")..duration("P4Y")],[duration("P1Y")..duration("P3Y")])"#,
        false,
    );
}

#[test]
fn _0029() {
    te_bool(false, &scope!(), r#"coincides([1..9],[1..10])"#, false);
}

#[test]
fn _0030() {
    te_bool(false, &scope!(), r#"coincides((1..10],(1..10])"#, true);
}

#[test]
fn _0031() {
    te_bool(false, &scope!(), r#"coincides((1..10],(2..10])"#, false);
}

#[test]
fn _0032() {
    te_bool(false, &scope!(), r#"coincides((1..9],(1..10])"#, false);
}

#[test]
fn _0033() {
    te_bool(false, &scope!(), r#"coincides([1..10),[1..10))"#, true);
}

#[test]
fn _0034() {
    te_bool(false, &scope!(), r#"coincides([1..10),[2..10))"#, false);
}

#[test]
fn _0035() {
    te_bool(false, &scope!(), r#"coincides([1..9),[1..10))"#, false);
}

#[test]
fn _0036() {
    te_bool(false, &scope!(), r#"coincides((1..10),(1..10))"#, true);
}

#[test]
fn _0037() {
    te_bool(false, &scope!(), r#"coincides((1..10),(2..10))"#, false);
}

#[test]
fn _0038() {
    te_bool(false, &scope!(), r#"coincides((1..9),(1..10))"#, false);
}

#[test]
fn _0039() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(range1: [1..10],range2: [1..10])"#,
        true,
    );
}

#[test]
fn _0040() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(range2: [1..10],range1: [1..10])"#,
        true,
    );
}

#[test]
fn _0041() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(range1: [1..9],range2: [1..10])"#,
        false,
    );
}

#[test]
fn _0042() {
    te_bool(
        false,
        &scope!(),
        r#"coincides(range2: [1..10],range1: [1..9])"#,
        false,
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"coincides(p1: 10, point2: 1)"#,
        r#"[named::coincides] invalid named parameters"#,
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#"coincides(point1: 10, p2: 1)"#,
        r#"[named::coincides] invalid named parameters"#,
    );
}

#[test]
fn _0045() {
    te_null(
        false,
        &scope!(),
        r#"coincides(p1: 10, p2: 1)"#,
        r#"[named::coincides] invalid named parameters"#,
    );
}

#[test]
fn _0046() {
    te_null(
        false,
        &scope!(),
        r#"coincides(r1: [1..10], range2: [1..10])"#,
        r#"[named::coincides] invalid named parameters"#,
    );
}

#[test]
fn _0047() {
    te_null(
        false,
        &scope!(),
        r#"coincides(range1: [1..10], r2: [1..10])"#,
        r#"[named::coincides] invalid named parameters"#,
    );
}

#[test]
fn _0048() {
    te_null(
        false,
        &scope!(),
        r#"coincides(r1: [1..10], r2: [1..10])"#,
        r#"[named::coincides] invalid named parameters"#,
    );
}

#[test]
fn _0049() {
    te_null(
        false,
        &scope!(),
        r#"coincides()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0050() {
    te_null(
        false,
        &scope!(),
        r#"coincides(1,2,3)"#,
        r#"expected 2 parameters, actual number of parameters is 3"#,
    );
}

#[test]
fn _0051() {
    te_null(
        false,
        &scope!(),
        r#"coincides(true,true)"#,
        r#"[core::coincides] invalid argument type, expected scalar or range of scalars, actual type is boolean"#,
    );
}

#[test]
fn _0052() {
    te_null(
        false,
        &scope!(),
        r#"coincides(null,true)"#,
        r#"[core::coincides] invalid argument type, expected scalar or range of scalars, actual type is Null"#,
    );
}
