use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_value(false, &scope!(), r#"@"P1Y0M""#, r#"duration("P1Y")"#);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), r#"date("2021-02-10").year"#, 2021, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), r#"date("2021-02-10").month"#, 2, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), r#"date("2021-02-10").day"#, 10, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), r#"date("2021-02-10").weekday"#, 3, 0);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), r#"date and time("2018-12-10T10:30:01").year"#, 2018, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), r#"date and time("2018-12-10T10:30:01").month"#, 12, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), r#"date and time("2018-12-10T10:30:01").day"#, 10, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), r#"date and time("2018-12-10T11:30:01").hour"#, 11, 0);
}

#[test]
fn _0010() {
  te_number(false, &scope!(), r#"date and time("2018-12-10T11:30:01").hour"#, 11, 0);
}

#[test]
fn _0011() {
  te_number(false, &scope!(), r#"date and time("2018-12-10T11:30:01").minute"#, 30, 0);
}

#[test]
fn _0012() {
  te_number(false, &scope!(), r#"date and time("2018-12-10T11:30:01").second"#, 1, 0);
}

#[test]
fn _0013() {
  te_number(false, &scope!(), r#"date and time("2021-02-10").hour"#, 0, 0);
}

#[test]
fn _0014() {
  te_number(false, &scope!(), r#"date and time("2021-02-10").minute"#, 0, 0);
}

#[test]
fn _0015() {
  te_number(false, &scope!(), r#"date and time("2021-02-10").second"#, 0, 0);
}

#[test]
fn _0016() {
  te_value(false, &scope!(), r#"date and time("2018-12-10T10:30:00+05:00").time offset"#, r#"@"PT5H""#);
}

#[test]
fn _0017() {
  te_null(
    false,
    &scope!(),
    r#"date and time("2018-12-10T10:30:00").time offset"#,
    "could not retrieve time offset for date and time",
  );
}

#[test]
fn _0018() {
  te_string(false, &scope!(), r#"date and time("2018-12-10T10:30:00@Etc/UTC").timezone"#, r#"Etc/UTC"#);
}

#[test]
fn _0019() {
  te_null(
    false,
    &scope!(),
    r#"date and time("2018-12-10T10:30:00").timezone"#,
    "could not retrieve timezone for date and time",
  );
}

#[test]
fn _0020() {
  te_number(false, &scope!(), r#"time("08:45:27").hour"#, 8, 0);
}

#[test]
fn _0021() {
  te_number(false, &scope!(), r#"time("08:45:27").minute"#, 45, 0);
}

#[test]
fn _0022() {
  te_number(false, &scope!(), r#"time("08:45:27").second"#, 27, 0);
}

#[test]
fn _0023() {
  te_value(false, &scope!(), r#"time("08:45:27-05:00").time offset"#, r#"@"-PT5H""#);
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#"time("08:45:27").time offset"#, "could not retrieve time offset for time");
}

#[test]
fn _0025() {
  te_string(false, &scope!(), r#"time("08:45:27@Etc/UTC").timezone"#, r#"Etc/UTC"#);
}

#[test]
fn _0026() {
  te_null(false, &scope!(), r#"time("08:45:27").timezone"#, "could not retrieve timezone for time");
}

#[test]
fn _0027() {
  te_number(false, &scope!(), r#"duration("P1Y2M").years"#, 1, 0);
}

#[test]
fn _0028() {
  te_number(false, &scope!(), r#"duration("P2M").years"#, 0, 0);
}

#[test]
fn _0029() {
  te_number(false, &scope!(), r#"duration("P2M").months"#, 2, 0);
}

#[test]
fn _0030() {
  te_number(false, &scope!(), r#"duration("P1Y").months"#, 0, 0);
}

#[test]
fn _0031() {
  te_null(false, &scope!(), r#"duration("P1Y").days"#, "no such property in years and months duration: days");
}

#[test]
fn _0032() {
  te_null(false, &scope!(), r#"duration("P1Y").hours"#, "no such property in years and months duration: hours");
}

#[test]
fn _0033() {
  te_null(false, &scope!(), r#"duration("P1Y").minutes"#, "no such property in years and months duration: minutes");
}

#[test]
fn _0034() {
  te_null(false, &scope!(), r#"duration("P1Y").seconds"#, "no such property in years and months duration: seconds");
}

#[test]
fn _0035() {
  te_null(false, &scope!(), r#"duration("P1D").years"#, "no such property in days and time duration: years");
}

#[test]
fn _0036() {
  te_null(false, &scope!(), r#"duration("P1D").months"#, "no such property in days and time duration: months");
}

#[test]
fn _0037() {
  te_number(false, &scope!(), r#"duration("P1D").days"#, 1, 0);
}

#[test]
fn _0038() {
  te_number(false, &scope!(), r#"duration("PT2H").days"#, 0, 0);
}

#[test]
fn _0039() {
  te_number(false, &scope!(), r#"duration("PT2H").hours"#, 2, 0);
}

#[test]
fn _0040() {
  te_number(false, &scope!(), r#"duration("P1D").hours"#, 0, 0);
}

#[test]
fn _0041() {
  te_number(false, &scope!(), r#"duration("P1DT3H").hours"#, 3, 0);
}

#[test]
fn _0042() {
  te_number(false, &scope!(), r#"duration("PT2M").minutes"#, 2, 0);
}

#[test]
fn _0043() {
  te_number(false, &scope!(), r#"duration("P1D").minutes"#, 0, 0);
}

#[test]
fn _0044() {
  te_number(false, &scope!(), r#"duration("PT5S").seconds"#, 5, 0);
}

#[test]
fn _0045() {
  te_number(false, &scope!(), r#"duration("P1D").seconds"#, 0, 0);
}

#[test]
fn _0046() {
  te_number(false, &scope!(), r#"duration("P3DT15H47M13S").days"#, 3, 0);
}

#[test]
fn _0047() {
  te_number(false, &scope!(), r#"duration("P3DT15H47M13S").hours"#, 15, 0);
}

#[test]
fn _0048() {
  te_number(false, &scope!(), r#"duration("P3DT15H47M13S").minutes"#, 47, 0);
}

#[test]
fn _0049() {
  te_number(false, &scope!(), r#"duration("P3DT15H47M13S").seconds"#, 13, 0);
}

#[test]
fn _0050() {
  te_number(false, &scope!(), r#" @"262142-12-31".weekday"#, 1, 0);
}

#[test]
fn _0051() {
  te_number(false, &scope!(), r#" @"-262143-01-01".weekday"#, 4, 0);
}

#[test]
fn _0052() {
  te_null(false, &scope!(), r#" @"2023-02-06".weekdays"#, "no such property in date: weekdays");
}

#[test]
fn _0053() {
  te_null(false, &scope!(), r#" @"262143-01-01T00:00:00".weekday"#, "invalid @ literal: 262143-01-01T00:00:00");
}

#[test]
fn _0054() {
  te_null(false, &scope!(), r#" @"-262144-12-31T23:59:59".weekday"#, "invalid @ literal: -262144-12-31T23:59:59");
}

#[test]
fn _0055() {
  te_number(false, &scope!(), r#" @"2023-02-06T10:11:12".weekday"#, 1, 0);
}

#[test]
fn _0056() {
  te_null(false, &scope!(), r#" @"2023-02-06T10:11:12".weekdays"#, "no such property in date and time: weekdays");
}

#[test]
fn _0057() {
  te_null(false, &scope!(), r#" @"10:11:12".weekdays"#, "no such property in time: weekdays");
}

#[test]
fn _0058() {
  te_null(false, &scope!(), r#" false.weekdays "#, "unexpected type: boolean, for property: weekdays");
}

#[test]
fn _0059() {
  te_number(false, &scope!(), r#" [1..10].start "#, 1, 0);
}

#[test]
fn _0060() {
  te_bool(false, &scope!(), r#" [1..10].start included "#, true);
}

#[test]
fn _0061() {
  te_bool(false, &scope!(), r#" (1..10].start included "#, false);
}

#[test]
fn _0062() {
  te_number(false, &scope!(), r#" [1..10].end "#, 10, 0);
}

#[test]
fn _0063() {
  te_bool(false, &scope!(), r#" [1..10].end included "#, true);
}

#[test]
fn _0064() {
  te_bool(false, &scope!(), r#" (1..10).end included "#, false);
}

#[test]
fn _0065() {
  te_null(false, &scope!(), r#" [1..10].spread "#, "no such property in range: spread");
}

#[test]
fn _0066() {
  te_number(false, &scope!(), r#" >1.start "#, 1, 0);
}

#[test]
fn _0067() {
  te_number(false, &scope!(), r#"(>10).start"#, 10, 0);
}

#[test]
fn _0068() {
  te_number(false, &scope!(), r#"(=10).start"#, 10, 0);
}

#[test]
fn _0069() {
  te_null(false, &scope!(), r#" >1.end "#, "no such property in unary greater: end");
}

#[test]
fn _0070() {
  te_bool(false, &scope!(), r#" >1.start included "#, false);
}

#[test]
fn _0071() {
  te_bool(false, &scope!(), r#" >1.end included "#, false);
}

#[test]
fn _0072() {
  te_number(true, &scope!(), r#" >=1.start "#, 1, 0);
}

#[test]
fn _0073() {
  te_null(false, &scope!(), r#" >=1.end "#, "no such property in unary greater or equal: end");
}

#[test]
fn _0074() {
  te_bool(false, &scope!(), r#" >=1.start included "#, true);
}

#[test]
fn _0075() {
  te_bool(false, &scope!(), r#" >=1.end included "#, false);
}

#[test]
fn _0076() {
  te_null(false, &scope!(), r#" <1.start "#, "no such property in unary less: start");
}

#[test]
fn _0077() {
  te_number(false, &scope!(), r#" <1.end "#, 1, 0);
}

#[test]
fn _0078() {
  te_bool(false, &scope!(), r#" <1.start included "#, false);
}

#[test]
fn _0079() {
  te_bool(false, &scope!(), r#" <1.end included "#, false);
}

#[test]
fn _0080() {
  te_null(false, &scope!(), r#" <=1.start "#, "no such property in unary less or equal: start");
}

#[test]
fn _0081() {
  te_number(false, &scope!(), r#" <=1.end "#, 1, 0);
}

#[test]
fn _0082() {
  te_bool(false, &scope!(), r#" <=1.start included "#, false);
}

#[test]
fn _0083() {
  te_bool(false, &scope!(), r#" <=1.end included "#, true);
}

#[test]
fn _0084() {
  te_bool(false, &scope!(), "(>= 10).end included", false);
}

#[test]
fn _0085() {
  te_bool(false, &scope!(), "(> (1+3)).end included", false);
}
