use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"before(1,10)"#, true);
}

#[test]
fn _0002() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("2022-01-31"),date("2022-02-01"))"#,
        true,
    );
}

#[test]
fn _0003() {
    te_bool(
        false,
        &scope!(),
        r#"before(time("10:11:12"),time("10:11:13"))"#,
        true,
    );
}

#[test]
fn _0004() {
    te_bool(
        false,
        &scope!(),
        r#"before(date and time("2022-01-31T10:11:12"),date and time("2022-01-31T10:11:13"))"#,
        true,
    );
}

#[test]
fn _0005() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1DT10H3M2S"),duration("P1DT10H3M3S"))"#,
        true,
    );
}

#[test]
fn _0006() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1Y"),duration("P1Y1M"))"#,
        true,
    );
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), r#"before(10,1)"#, false);
}

#[test]
fn _0008() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("2022-02-01"),date("2022-01-31"))"#,
        false,
    );
}

#[test]
fn _0009() {
    te_bool(
        false,
        &scope!(),
        r#"before(time("23:15:23"),time("23:15:22"))"#,
        false,
    );
}

#[test]
fn _0010() {
    te_bool(
        false,
        &scope!(),
        r#"before(date and time("2022-02-01T23:12:01"),date and time("2022-02-01T23:12:00"))"#,
        false,
    );
}

#[test]
fn _0011() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1DT2H2M"),duration("P1DT2H"))"#,
        false,
    );
}

#[test]
fn _0012() {
    te_bool(false, &scope!(), r#"before(10,10)"#, false);
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("1981-12-13"),date("1981-12-13"))"#,
        false,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#"before(time("12:13:14"),time("12:13:14"))"#,
        false,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#"before(date and time("2021-01-01T12:13:14"),date and time("2021-01-01T12:13:14"))"#,
        false,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1DT2H3M4S"),duration("P1DT2H3M4S"))"#,
        false,
    );
}

#[test]
fn _0017() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P2Y3M"),duration("P2Y3M"))"#,
        false,
    );
}

#[test]
fn _0018() {
    te_bool(false, &scope!(), r#"before(1,[1..10])"#, false);
}

#[test]
fn _0019() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("1960-11-15"),[date("1960-11-15")..date("1960-12-31")])"#,
        false,
    );
}

#[test]
fn _0020() {
    te_bool(
        false,
        &scope!(),
        r#"before(time("12:13:14"),[time("12:13:14")..time("13:00:00")])"#,
        false,
    );
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"before(date and time("1960-12-31T12:13:14"),[date and time("1960-12-31T12:13:14")..date and time("1960-12-31T13:00:00")])"#,
        false,
    );
}

#[test]
fn _0022() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1DT2H3M2S"),[duration("P1DT2H3M2S")..duration("P2DT2H3M2S")])"#,
        false,
    );
}

#[test]
fn _0023() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1Y4M"),[duration("P1Y4M")..duration("P1Y6M")])"#,
        false,
    );
}

#[test]
fn _0024() {
    te_bool(false, &scope!(), r#"before(1,(1..10])"#, true);
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("1960-11-15"),(date("1960-11-15")..date("1960-12-31")])"#,
        true,
    );
}

#[test]
fn _0026() {
    te_bool(
        false,
        &scope!(),
        r#"before(time("12:13:14"),(time("12:13:14")..time("13:00:00")])"#,
        true,
    );
}

#[test]
fn _0027() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1DT2H3M2S"),(duration("P1DT2H3M2S")..duration("P2DT2H3M2S")])"#,
        true,
    );
}

#[test]
fn _0028() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1Y4M"),(duration("P1Y4M")..duration("P1Y6M")])"#,
        true,
    );
}

#[test]
fn _0029() {
    te_bool(false, &scope!(), "before(1,[5..10])", true);
}

#[test]
fn _0030() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("1960-11-15"),[date("1960-11-17")..date("1960-12-31")])"#,
        true,
    );
}

#[test]
fn _0031() {
    te_bool(
        false,
        &scope!(),
        r#"before(time("12:01:02"),[time("12:01:03")..time("13:00:00")])"#,
        true,
    );
}

#[test]
fn _0032() {
    te_bool(
        false,
        &scope!(),
        r#"before(date and time("2021-02-16T12:01:02"),[date and time("2021-02-16T12:01:03")..date and time("2021-02-16T13:00:00")])"#,
        true,
    );
}

#[test]
fn _0033() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1DT2H3M1S"),[duration("P1DT2H3M2S")..duration("P2DT2H3M2S")])"#,
        true,
    );
}

#[test]
fn _0034() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1Y3M"),[duration("P1Y4M")..duration("P1Y6M")])"#,
        true,
    );
}

#[test]
fn _0035() {
    te_bool(false, &scope!(), r#"before([1..10],10)"#, false);
}

#[test]
fn _0036() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")],date("1960-12-31"))"#,
        false,
    );
}

#[test]
fn _0037() {
    te_bool(
        false,
        &scope!(),
        r#"before([time("13:18:56")..time("14:00:00")],time("13:59:59"))"#,
        false,
    );
}

#[test]
fn _0038() {
    te_bool(
        false,
        &scope!(),
        r#"before([date and time("2022-02-08T13:18:56")..date and time("2022-02-08T14:00:00")],date and time("2022-02-08T13:59:59"))"#,
        false,
    );
}

#[test]
fn _0039() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1D")..duration("P3D")],duration("P2D"))"#,
        false,
    );
}

#[test]
fn _0040() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1Y")..duration("P3Y")],duration("P2Y"))"#,
        false,
    );
}

#[test]
fn _0041() {
    te_bool(false, &scope!(), r#"before([1..10),10)"#, true);
}

#[test]
fn _0042() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")),date("1960-12-31"))"#,
        true,
    );
}

#[test]
fn _0043() {
    te_bool(
        false,
        &scope!(),
        r#"before([time("01:12:18")..time("15:12:30")),time("15:12:30"))"#,
        true,
    );
}

#[test]
fn _0044() {
    te_bool(
        false,
        &scope!(),
        r#"before([date and time("2020-07-23T01:12:18")..date and time("2020-07-23T15:12:30")),date and time("2020-07-23T15:12:30"))"#,
        true,
    );
}

#[test]
fn _0045() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1D")..duration("P10D")),duration("P10D"))"#,
        true,
    );
}

#[test]
fn _0046() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1Y")..duration("P4Y")),duration("P4Y"))"#,
        true,
    );
}

#[test]
fn _0047() {
    te_bool(false, &scope!(), r#"before([1..10],15)"#, true);
}

#[test]
fn _0048() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")],date("1961-02-12"))"#,
        true,
    );
}

#[test]
fn _0049() {
    te_bool(
        false,
        &scope!(),
        r#"before([time("01:12:18")..time("15:12:30")],time("15:12:31"))"#,
        true,
    );
}

#[test]
fn _0050() {
    te_bool(
        false,
        &scope!(),
        r#"before([date and time("2020-07-23T01:12:18")..date and time("2020-07-23T15:12:30")],date and time("2020-07-23T15:12:31"))"#,
        true,
    );
}

#[test]
fn _0051() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1D")..duration("P10D")],duration("P10DT1S"))"#,
        true,
    );
}

#[test]
fn _0052() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1Y")..duration("P4Y")],duration("P4Y1M"))"#,
        true,
    );
}

#[test]
fn _0053() {
    te_bool(false, &scope!(), r#"before([1..10],[15..20])"#, true);
}

#[test]
fn _0054() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")],[date("1961-01-01")..date("1960-05-31")])"#,
        true,
    );
}

#[test]
fn _0055() {
    te_bool(
        false,
        &scope!(),
        r#"before([time("01:00:00")..time("11:59:59")],[time("12:00:00")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0056() {
    te_bool(
        false,
        &scope!(),
        r#"before([date and time("2020-01-01T01:00:00")..date and time("2020-01-01T11:59:59")],[date and time("2020-01-01T12:00:00")..date and time("2020-01-01T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0057() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1D")..duration("P23D")],[duration("P23DT1S")..duration("P25D")])"#,
        true,
    );
}

#[test]
fn _0058() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1Y")..duration("P23Y")],[duration("P23Y1M")..duration("P25Y")])"#,
        true,
    );
}

#[test]
fn _0059() {
    te_bool(false, &scope!(), r#"before([1..10],[10..20])"#, false);
}

#[test]
fn _0060() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")],[date("1960-12-31")..date("1960-05-31")])"#,
        false,
    );
}

#[test]
fn _0061() {
    te_bool(
        false,
        &scope!(),
        r#"before([time("01:00:00")..time("12:00:00")],[time("12:00:00")..time("23:59:59")])"#,
        false,
    );
}

#[test]
fn _0062() {
    te_bool(
        false,
        &scope!(),
        r#"before([date and time("2020-01-01T01:00:00")..date and time("2020-01-01T12:00:00")],[date and time("2020-01-01T12:00:00")..date and time("2020-01-01T23:59:59")])"#,
        false,
    );
}

#[test]
fn _0063() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1D")..duration("P23D")],[duration("P23D")..duration("P25D")])"#,
        false,
    );
}

#[test]
fn _0064() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1Y")..duration("P23Y")],[duration("P23Y")..duration("P25Y")])"#,
        false,
    );
}

#[test]
fn _0065() {
    te_bool(false, &scope!(), r#"before([1..10),[10..20])"#, true);
}

#[test]
fn _0066() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")),[date("1960-12-31")..date("1960-05-31")])"#,
        true,
    );
}

#[test]
fn _0067() {
    te_bool(
        false,
        &scope!(),
        r#"before([time("01:00:00")..time("12:00:00")),[time("12:00:00")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0068() {
    te_bool(
        false,
        &scope!(),
        r#"before([date and time("2020-01-01T01:00:00")..date and time("2020-01-01T12:00:00")),[date and time("2020-01-01T12:00:00")..date and time("2020-01-01T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0069() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1D")..duration("P23D")),[duration("P23D")..duration("P25D")])"#,
        true,
    );
}

#[test]
fn _0070() {
    te_bool(
        false,
        &scope!(),
        r#"before([duration("P1Y")..duration("P23Y")),[duration("P23Y")..duration("P25Y")])"#,
        true,
    );
}

#[test]
fn _0071() {
    te_bool(false, &scope!(), r#"before([1..10],(10..20])"#, true);
}

#[test]
fn _0072() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")],(date("1960-12-31")..date("1960-05-31")])"#,
        true,
    );
}

#[test]
fn _0073() {
    te_bool(false, &scope!(), r#"before([1..10),(10..20])"#, true);
}

#[test]
fn _0074() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("1960-11-17")..date("1960-12-31")),(date("1960-12-31")..date("1960-05-31")])"#,
        true,
    );
}

#[test]
fn _0075() {
    te_bool(false, &scope!(), r#"before(point1:1,point2:10)"#, true);
}

#[test]
fn _0076() {
    te_bool(
        false,
        &scope!(),
        r#"before(point1:date("2022-01-31"),point2:date("2022-02-01"))"#,
        true,
    );
}

#[test]
fn _0077() {
    te_bool(
        false,
        &scope!(),
        r#"before(point1:time("10:11:12"),point2:time("10:11:13"))"#,
        true,
    );
}

#[test]
fn _0078() {
    te_bool(
        false,
        &scope!(),
        r#"before(point1:date and time("2022-01-31T10:11:12"),point2:date and time("2022-01-31T10:11:13"))"#,
        true,
    );
}

#[test]
fn _0079() {
    te_bool(
        false,
        &scope!(),
        r#"before(point1:duration("P1DT10H3M2S"),point2:duration("P1DT10H3M3S"))"#,
        true,
    );
}

#[test]
fn _0080() {
    te_bool(
        false,
        &scope!(),
        r#"before(point1:duration("P1Y"),point2:duration("P1Y1M"))"#,
        true,
    );
}

#[test]
fn _0081() {
    te_bool(false, &scope!(), r#"before(point2:10,point1:1)"#, true);
}

#[test]
fn _0082() {
    te_bool(
        false,
        &scope!(),
        r#"before(point2:date("2022-02-01"),point1:date("2022-01-31"))"#,
        true,
    );
}

#[test]
fn _0083() {
    te_bool(false, &scope!(), r#"before(point1:10,point2:1)"#, false);
}

#[test]
fn _0084() {
    te_bool(
        false,
        &scope!(),
        r#"before(point1:date("2022-02-01"),point2:date("2022-01-31"))"#,
        false,
    );
}

#[test]
fn _0085() {
    te_bool(false, &scope!(), r#"before(point1:10,point2:10)"#, false);
}

#[test]
fn _0086() {
    te_bool(
        false,
        &scope!(),
        r#"before(point1:date("2022-02-01"),point2:date("2022-02-01"))"#,
        false,
    );
}

#[test]
fn _0087() {
    te_bool(
        false,
        &scope!(),
        r#"before(range1:[1..10),range2:(10..20])"#,
        true,
    );
}

#[test]
fn _0088() {
    te_bool(
        false,
        &scope!(),
        r#"before(range1:[date("1960-11-17")..date("1960-12-31")),range2:(date("1960-12-31")..date("1960-05-31")])"#,
        true,
    );
}

#[test]
fn _0089() {
    te_bool(
        false,
        &scope!(),
        r#"before(range1:[time("01:00:00")..time("12:00:00")),range2:(time("12:00:00")..time("23:59:59")])"#,
        true,
    );
}

#[test]
fn _0090() {
    te_bool(
        false,
        &scope!(),
        r#"before(range1:[date and time("2020-01-01T01:00:00")..date and time("2020-01-01T12:00:00")),range2:(date and time("2020-01-01T12:00:00")..date and time("2020-01-01T23:59:59")])"#,
        true,
    );
}

#[test]
fn _0091() {
    te_bool(
        false,
        &scope!(),
        r#"before(range1:[duration("P1D")..duration("P23D")),range2:(duration("P23D")..duration("P25D")])"#,
        true,
    );
}

#[test]
fn _0092() {
    te_bool(
        false,
        &scope!(),
        r#"before(range1:[duration("P1Y")..duration("P23Y")),range2:(duration("P23Y")..duration("P25Y")])"#,
        true,
    );
}

#[test]
fn _0093() {
    te_bool(false, &scope!(), r#"before(point:1,range:(1..10])"#, true);
}

#[test]
fn _0094() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("1960-11-15"),(date("1960-11-15")..date("1960-12-31")])"#,
        true,
    );
}

#[test]
fn _0095() {
    te_bool(
        false,
        &scope!(),
        r#"before(time("12:13:14"),(time("12:13:14")..time("13:00:00")])"#,
        true,
    );
}

#[test]
fn _0096() {
    te_bool(
        false,
        &scope!(),
        r#"before(date and time("1960-12-31T12:13:14"),(date and time("1960-12-31T12:13:14")..date and time("1960-12-31T13:00:00")])"#,
        true,
    );
}

#[test]
fn _0097() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1DT2H3M2S"),(duration("P1DT2H3M2S")..duration("P2DT2H3M2S")])"#,
        true,
    );
}

#[test]
fn _0098() {
    te_bool(
        false,
        &scope!(),
        r#"before(duration("P1Y4M"),[duration("P1Y4M")..duration("P1Y6M")])"#,
        false,
    );
}

#[test]
fn _0099() {
    te_bool(false, &scope!(), r#"before(range:(1..10],point:1)"#, false);
}

#[test]
fn _0100() {
    te_bool(
        false,
        &scope!(),
        r#"before(range:[date("1960-11-17")..date("1960-12-31")],point:date("1960-12-31"))"#,
        false,
    );
}

#[test]
fn _0101() {
    te_bool(
        false,
        &scope!(),
        r#"before(range:[time("13:18:56")..time("14:00:00")],point:time("13:59:59"))"#,
        false,
    );
}

#[test]
fn _0102() {
    te_bool(
        false,
        &scope!(),
        r#"before(range:[date and time("2022-02-08T13:18:56")..date and time("2022-02-08T14:00:00")],point:date and time("2022-02-08T13:59:59"))"#,
        false,
    );
}

#[test]
fn _0103() {
    te_bool(
        false,
        &scope!(),
        r#"before(range:[duration("P1D")..duration("P3D")],point:duration("P2D"))"#,
        false,
    );
}

#[test]
fn _0104() {
    te_bool(
        false,
        &scope!(),
        r#"before(range:[duration("P1Y")..duration("P3Y")],point:duration("P2Y"))"#,
        false,
    );
}

#[test]
fn _0105() {
    te_bool(false, &scope!(), r#"before(range:[1..10),point:10)"#, true);
}

#[test]
fn _0106() {
    te_null(
        false,
        &scope!(),
        r#"before()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0107() {
    te_null(
        false,
        &scope!(),
        r#"before(1)"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0108() {
    te_null(
        false,
        &scope!(),
        r#"before(1,2,3)"#,
        r#"expected 2 parameters, actual number of parameters is 3"#,
    );
}

#[test]
fn _0109() {
    te_null(
        false,
        &scope!(),
        r#"before(p1: 1, point2: 2)"#,
        r#"[named::before] invalid named parameters"#,
    );
}

#[test]
fn _0110() {
    te_null(
        false,
        &scope!(),
        r#"before("abc","bcd")"#,
        r#"[core::before] invalid argument type, expected scalar or range of scalars, actual type is string"#,
    );
}

#[test]
fn _0111() {
    te_bool(
        false,
        &scope!(),
        r#"before(date("2022-01-31"),[date("2022-02-01")..date("2022-12-31")])"#,
        true,
    );
}

#[test]
fn _0112() {
    te_bool(
        false,
        &scope!(),
        r#"before([date("2021-01-01")..date("2021-12-31")],[date("2022-01-01")..date("2022-12-31")])"#,
        true,
    );
}
