use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_none(false, &scope!(), r#"2 in ()"#);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"2 in []"#, false);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"2 in 2"#, true);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"2 in 3"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#" "a" in "a" "#, true);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#" "a" in "a" "#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#" "a" in "b" "#, false);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in @"2023-02-06" "#, true);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in @"2023-02-07" "#, false);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:12" in @"2023-02-06T10:11:12" "#, true);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:12" in @"2023-02-06T10:11:13" "#, false);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#" @"10:11:12" in @"10:11:12" "#, true);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#" @"10:11:12" in @"10:11:13" "#, false);
}

#[test]
fn _0014() {
  te_bool(false, &scope!(), r#" @"P1D" in @"P1D" "#, true);
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#" @"P1D" in @"P2D" "#, false);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#" @"P1Y" in @"P1Y" "#, true);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#" @"P1Y" in @"P2Y" "#, false);
}

#[test]
fn _0018() {
  te_bool(false, &scope!(), r#" null in null "#, true);
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#" null in @"P2Y" "#, false);
}

#[test]
fn _0020() {
  te_bool(false, &scope!(), r#"2 in [1..5]"#, true);
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#"99 in <=100"#, true);
}

#[test]
fn _0022() {
  let scope = &te_scope("{ a: 100.0, b: 99.0, c: 101.0}");
  te_bool(false, scope, "(b) in <=100", true);
  te_bool(false, scope, "b in <=100", true);
  te_bool(false, scope, "(a) in <=100", true);
  te_bool(false, scope, "a in <=100", true);
  te_bool(false, scope, "(b) in <100", true);
  te_bool(false, scope, "b in <100", true);
  te_bool(false, scope, "(a) in <100", false);
  te_bool(false, scope, "(a) in >=100", true);
  te_bool(false, scope, "(b) in >=100", false);
  te_bool(false, scope, "b in >=100", false);
  te_bool(false, scope, "(a) in >100", false);
  te_bool(false, scope, "a in >100", false);
  te_bool(false, scope, "(a) in (1,2,3,4,5)", false);
}

#[test]
fn _0023() {
  te_bool(false, &scope!(), "99 in >=100", false);
}

#[test]
fn _0024() {
  te_bool(false, &scope!(), "100 in >=100", true);
}

#[test]
fn _0025() {
  te_bool(false, &scope!(), "99 in <100", true);
}

#[test]
fn _0026() {
  te_bool(false, &scope!(), "101 in >=100", true);
}

#[test]
fn _0027() {
  te_bool(false, &scope!(), "100 in <100", false);
}

#[test]
fn _0028() {
  te_bool(false, &scope!(), "100 in <=100", true);
}

#[test]
fn _0029() {
  te_bool(false, &scope!(), "101 in <=100", false);
}

#[test]
fn _0030() {
  te_bool(false, &scope!(), "101 in >100", true);
}

#[test]
fn _0031() {
  te_bool(false, &scope!(), "100 in >100", false);
}

#[test]
fn _0032() {
  te_bool(false, &scope!(), "2 in (2)", true);
}

#[test]
fn _0033() {
  te_bool(false, &scope!(), "2 in (3)", false);
}

#[test]
fn _0034() {
  te_bool(false, &scope!(), "2 in (1,2,3,4,5)", true);
}

#[test]
fn _0035() {
  te_bool(false, &scope!(), "7 in (1,2,3,4,5)", false);
}

#[test]
fn _0036() {
  te_bool(false, &scope!(), "2 in (<3)", true);
}

#[test]
fn _0037() {
  te_bool(false, &scope!(), "6 in (>5)", true);
}

#[test]
fn _0038() {
  te_bool(false, &scope!(), "6 in (>3,>5)", true);
}

#[test]
fn _0039() {
  te_bool(false, &scope!(), "2 in (<3,>5)", true);
}

#[test]
fn _0040() {
  te_bool(false, &scope!(), "3 in (<3,>5)", false);
}

#[test]
fn _0041() {
  te_bool(false, &scope!(), "4.12 in (<3,>5)", false);
}

#[test]
fn _0042() {
  te_bool(false, &scope!(), "5 in (<3,>5)", false);
}

#[test]
fn _0043() {
  te_bool(false, &scope!(), "2 in (>5,<3)", true);
}

#[test]
fn _0044() {
  te_bool(false, &scope!(), "5 in (>5,<3)", false);
}

#[test]
fn _0045() {
  te_bool(false, &scope!(), "4.5 in (>5,<3)", false);
}

#[test]
fn _0046() {
  te_bool(false, &scope!(), "3 in (>5,<3)", false);
}

#[test]
fn _0047() {
  te_bool(false, &scope!(), "2 in (<=3)", true);
}

#[test]
fn _0048() {
  te_bool(false, &scope!(), "2 in (<=3,>=5)", true);
}

#[test]
fn _0049() {
  te_bool(false, &scope!(), "3 in (<=3,>=5)", true);
}

#[test]
fn _0050() {
  te_bool(false, &scope!(), "5 in (<=3,>=5)", true);
}

#[test]
fn _0051() {
  te_bool(false, &scope!(), "4 in (<=3,>=5)", false);
}

#[test]
fn _0052() {
  te_bool(false, &scope!(), "2 in (>=5,<=3)", true);
}

#[test]
fn _0053() {
  te_bool(false, &scope!(), "3 in (>=5,<=3)", true);
}

#[test]
fn _0054() {
  te_bool(false, &scope!(), "5 in (>=5,<=3)", true);
}

#[test]
fn _0055() {
  te_bool(false, &scope!(), "4 in (>=5,<=3)", false);
}

#[test]
fn _0056() {
  te_bool(false, &scope!(), "not(4 in (1,3))", true);
}

#[test]
fn _0057() {
  te_bool(false, &scope!(), "not(5.25 in (1.32,2.45,4.12,5.25))", false);
}

#[test]
fn _0058() {
  te_bool(false, &scope!(), "5 in (<=5)", true);
}

#[test]
fn _0059() {
  te_bool(false, &scope!(), "5 in ((5..10])", false);
}

#[test]
fn _0060() {
  te_bool(false, &scope!(), "5 in ([5..10])", true);
}

#[test]
fn _0061() {
  te_bool(false, &scope!(), "5 in (4,5,6)", true);
}

#[test]
fn _0062() {
  te_bool(false, &scope!(), "5 in (<5,>5)", false);
}

#[test]
fn _0063() {
  te_bool(false, &scope!(), "1 in [2,3,1]", true);
}

#[test]
fn _0064() {
  te_bool(false, &scope!(), r#""k" in ["j".."l"]"#, true);
}

#[test]
fn _0065() {
  te_bool(false, &scope!(), r#""b" in [["f".."h"], ["a".."c"]]"#, true);
}

#[test]
fn _0066() {
  te_bool(false, &scope!(), r#""a" in <= "b""#, true);
}

#[test]
fn _0067() {
  te_bool(false, &scope!(), r#"true in [false, 2, 3]"#, false);
}

#[test]
fn _0068() {
  te_bool(false, &scope!(), r#"true in true"#, true);
}

#[test]
fn _0069() {
  te_bool(
    false,
    &scope!(),
    r#"date("2018-12-08") in [date("2018-12-08"),date("2018-12-09"),date("2018-12-10")]"#,
    true,
  );
}

#[test]
fn _0070() {
  te_bool(false, &scope!(), r#"date("2018-12-04") in <= date("2018-12-05")"#, true);
}

#[test]
fn _0071() {
  te_bool(false, &scope!(), r#"[1,2,3] in [[1,2,3,4], [1,2,3]]"#, true);
}

#[test]
fn _0072() {
  te_bool(false, &scope!(), r#"[1,2,2] in [[1,2,3,4], [1,2,3]]"#, false);
}

#[test]
fn _0073() {
  te_bool(false, &scope!(), r#"{a: "foo"} in [{b: "bar"}, {a: "foo"}]"#, true);
}

#[test]
fn _0074() {
  te_bool(false, &scope!(), r#"duration("P11Y") in [duration("P8Y"),duration("P9Y"),duration("P10Y")]"#, false);
}

#[test]
fn _0075() {
  te_bool(false, &scope!(), r#" "k" in <= "k" "#, true);
}

#[test]
fn _0076() {
  te_bool(false, &scope!(), r#" "k" in <= "i" "#, false);
}

#[test]
fn _0077() {
  te_bool(false, &scope!(), r#" "k" in < "z" "#, true);
}

#[test]
fn _0078() {
  te_bool(false, &scope!(), r#" "k" in < "k" "#, false);
}

#[test]
fn _0079() {
  te_bool(false, &scope!(), r#" "k" in >= "k" "#, true);
}

#[test]
fn _0080() {
  te_bool(false, &scope!(), r#" "k" in >= "l" "#, false);
}

#[test]
fn _0081() {
  te_bool(false, &scope!(), r#" "k" in > "i" "#, true);
}

#[test]
fn _0082() {
  te_bool(false, &scope!(), r#" "k" in > "k" "#, false);
}

#[test]
fn _0083() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in <= @"2023-02-06" "#, true);
}

#[test]
fn _0084() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in <= @"2023-02-05" "#, false);
}

#[test]
fn _0085() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in < @"2023-02-07" "#, true);
}

#[test]
fn _0086() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in < @"2023-02-05" "#, false);
}

#[test]
fn _0087() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in >= @"2023-02-06" "#, true);
}

#[test]
fn _0088() {
  te_bool(false, &scope!(), r#" @"2023-02-05" in >= @"2023-02-06" "#, false);
}

#[test]
fn _0089() {
  te_bool(false, &scope!(), r#" @"2023-02-06" in > @"2023-02-05" "#, true);
}

#[test]
fn _0090() {
  te_bool(false, &scope!(), r#" @"2023-02-05" in > @"2023-02-06" "#, false);
}

#[test]
fn _0091() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:23" in <= @"2023-02-06T10:11:23" "#, true);
}

#[test]
fn _0092() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:23" in <= @"2023-02-06T10:11:22" "#, false);
}

#[test]
fn _0093() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:23" in < @"2023-02-06T10:11:24" "#, true);
}

#[test]
fn _0094() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:23" in < @"2023-02-06T10:11:23" "#, false);
}

#[test]
fn _0095() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:23" in >= @"2023-02-06T10:11:23" "#, true);
}

#[test]
fn _0096() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:21" in >= @"2023-02-06T10:11:22" "#, false);
}

#[test]
fn _0097() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:23" in > @"2023-02-06T10:11:22" "#, true);
}

#[test]
fn _0098() {
  te_bool(false, &scope!(), r#" @"2023-02-06T10:11:23" in > @"2023-02-06T10:11:23" "#, false);
}

#[test]
fn _0099() {
  te_bool(false, &scope!(), r#" @"10:11:23" in <= @"10:11:23" "#, true);
}

#[test]
fn _0100() {
  te_bool(false, &scope!(), r#" @"10:11:23" in <= @"10:11:22" "#, false);
}

#[test]
fn _0101() {
  te_bool(false, &scope!(), r#" @"10:11:23" in < @"10:11:24" "#, true);
}

#[test]
fn _0102() {
  te_bool(false, &scope!(), r#" @"10:11:23" in < @"10:11:23" "#, false);
}

#[test]
fn _0103() {
  te_bool(false, &scope!(), r#" @"10:11:23" in >= @"10:11:23" "#, true);
}

#[test]
fn _0104() {
  te_bool(false, &scope!(), r#" @"10:11:21" in >= @"10:11:22" "#, false);
}

#[test]
fn _0105() {
  te_bool(false, &scope!(), r#" @"10:11:23" in > @"10:11:22" "#, true);
}

#[test]
fn _0106() {
  te_bool(false, &scope!(), r#" @"10:11:23" in > @"10:11:23" "#, false);
}

#[test]
fn _0107() {
  te_bool(false, &scope!(), r#" @"P2D" in <= @"P2D" "#, true);
}

#[test]
fn _0108() {
  te_bool(false, &scope!(), r#" @"P2D" in <= @"P1D" "#, false);
}

#[test]
fn _0109() {
  te_bool(false, &scope!(), r#" @"P2D" in < @"P3D" "#, true);
}

#[test]
fn _0110() {
  te_bool(false, &scope!(), r#" @"P2D" in < @"P2D" "#, false);
}

#[test]
fn _0111() {
  te_bool(false, &scope!(), r#" @"P2D" in >= @"P2D" "#, true);
}

#[test]
fn _0112() {
  te_bool(false, &scope!(), r#" @"P2D" in >= @"P3D" "#, false);
}

#[test]
fn _0113() {
  te_bool(false, &scope!(), r#" @"P2D" in > @"P1D" "#, true);
}

#[test]
fn _0114() {
  te_bool(false, &scope!(), r#" @"P2D" in > @"P2D" "#, false);
}

#[test]
fn _0115() {
  te_bool(false, &scope!(), r#" @"P2Y" in <= @"P2Y" "#, true);
}

#[test]
fn _0116() {
  te_bool(false, &scope!(), r#" @"P2Y" in <= @"P1Y" "#, false);
}

#[test]
fn _0117() {
  te_bool(false, &scope!(), r#" @"P2Y" in < @"P3Y" "#, true);
}

#[test]
fn _0118() {
  te_bool(false, &scope!(), r#" @"P2Y" in < @"P2Y" "#, false);
}

#[test]
fn _0119() {
  te_bool(false, &scope!(), r#" @"P2Y" in >= @"P2Y" "#, true);
}

#[test]
fn _0120() {
  te_bool(false, &scope!(), r#" @"P2Y" in >= @"P3Y" "#, false);
}

#[test]
fn _0121() {
  te_bool(false, &scope!(), r#" @"P2Y" in > @"P1Y" "#, true);
}

#[test]
fn _0122() {
  te_bool(false, &scope!(), r#" @"P2Y" in > @"P2Y" "#, false);
}

#[test]
fn _0123() {
  te_bool(
    false,
    &scope!(),
    r#"duration("P11Y") in [[duration("P5Y") .. duration("P7Y")], [duration("P10Y") .. duration("P12Y")]]"#,
    true,
  );
}

#[test]
fn _0124() {
  te_bool(false, &scope!(), r#"duration("P11Y") in > duration("P10Y")"#, true);
}

#[test]
fn _0125() {
  te_null(false, &scope!(), r#" true in <= @"P2D" "#, "eval_in_unary_less_or_equal");
}

#[test]
fn _0126() {
  te_null(false, &scope!(), r#" true in < @"P2D" "#, "eval_in_unary_less");
}

#[test]
fn _0127() {
  te_null(false, &scope!(), r#" true in >= @"P2D" "#, "eval_in_unary_greater_or_equal");
}

#[test]
fn _0128() {
  te_null(false, &scope!(), r#" true in > @"P2D" "#, "eval_in_unary_greater");
}

#[test]
fn _0129() {
  te_null(false, &scope!(), r#" 1 in (function() 1) "#, "unexpected argument type in 'in' operator: function<>->Any");
}

#[test]
fn _0130() {
  te_bool(false, &scope!(), r#" null in [1,2,null,3] "#, true);
}

#[test]
fn _0131() {
  te_bool(false, &scope!(), r#" 5 in [1,2,null,3] "#, false);
}

#[test]
fn _0132() {
  te_bool(false, &scope!(), r#" @"10:11:12" in [@"10:11:10", @"10:11:11", @"10:11:12", @"10:11:13"] "#, true);
}

#[test]
fn _0133() {
  te_bool(false, &scope!(), r#" @"10:11:18" in [@"10:11:10", @"10:11:11", @"10:11:12" ,@"10:11:13"] "#, false);
}

#[test]
fn _0134() {
  te_bool(
    false,
    &scope!(),
    r#" @"2023-02-06T10:11:12" in [@"2023-02-06T10:11:10", @"2023-02-06T10:11:11", @"2023-02-06T10:11:12", @"2023-02-06T10:11:13"] "#,
    true,
  );
}

#[test]
fn _0135() {
  te_bool(
    false,
    &scope!(),
    r#" @"2023-02-06T10:11:18" in [@"2023-02-06T10:11:10", @"2023-02-06T10:11:11", @"2023-02-06T10:11:12" ,@"2023-02-06T10:11:13"] "#,
    false,
  );
}

#[test]
fn _0136() {
  te_bool(false, &scope!(), r#" @"P3D" in [@"P1D", @"P2D", @"P3D", @"P4D"] "#, true);
}

#[test]
fn _0137() {
  te_bool(false, &scope!(), r#" @"P5D" in [@"P1D", @"P2D", @"P3D", @"P4D"] "#, false);
}

#[test]
fn _0138() {
  te_bool(false, &scope!(), r#"[1,2,3] in [1]"#, false);
}

#[test]
fn _0139() {
  te_null(false, &scope!(), r#" 10 in [(function() 1)] "#, "unexpected type in list: function<>->Any");
}

#[test]
fn _0140() {
  te_bool(false, &scope!(), r#"10 in =10"#, true);
}

#[test]
fn _0141() {
  te_bool(false, &scope!(), r#"9 in =10"#, false);
}

#[test]
fn _0142() {
  te_bool(false, &scope!(), r#"9 in !=10"#, true);
}

#[test]
fn _0143() {
  te_bool(false, &scope!(), r#"10 in !=10"#, false);
}

#[test]
fn _0144() {
  te_bool(false, &scope!(), r#"10 in [=10]"#, true);
}

#[test]
fn _0145() {
  te_bool(false, &scope!(), r#"9 in [=10]"#, false);
}

#[test]
fn _0146() {
  te_bool(false, &scope!(), r#"9 in [!=10]"#, true);
}

#[test]
fn _0147() {
  te_bool(false, &scope!(), r#"10 in [9, !=10]"#, false);
}

#[test]
fn _0148() {
  te_bool(false, &scope!(), r#"5 in (=4,=5,=6)"#, true);
}

#[test]
fn _0149() {
  te_bool(false, &scope!(), r#"5 in (!=4,!=5,!=6)"#, false);
}

#[test]
fn _0150() {
  te_bool(false, &scope!(), r#"5 in (!=4,=5,!=6)"#, true);
}

#[test]
fn _0151() {
  te_bool(false, &scope!(), r#"5 in (!=5,=5)"#, true);
}

#[test]
fn _0152() {
  te_bool(false, &scope!(), r#"4 in (!=5)"#, true);
}

#[test]
fn _0153() {
  te_bool(false, &scope!(), r#"5 in (!=5)"#, false);
}

#[test]
fn _0154() {
  te_bool(false, &scope!(), r#"5 in (=5)"#, true);
}

#[test]
fn _0155() {
  te_bool(false, &scope!(), r#"4 in (=5)"#, false);
}
