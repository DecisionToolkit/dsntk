use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), "1=1", true);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), "100 = null", false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#" "a" = null "#, false);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), "1 = 1.000", true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), "1.276=1.276", true);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), ".54635=.54635", true);
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), "(1+1)=2", true);
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), "(1+2)=2", false);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), " ( 1 + 1 ) = 2", true);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), " ( 1 + 3 ) = 2", false);
}

#[test]
fn _0011() {
    te_bool(false, &scope!(), "(1+1)=(5-3)", true);
}

#[test]
fn _0012() {
    te_bool(false, &scope!(), "(1*2)=(10/5)", true);
}

#[test]
fn _0013() {
    te_bool(false, &scope!(), "true=true", true);
}

#[test]
fn _0014() {
    te_bool(false, &scope!(), "true=false", false);
}

#[test]
fn _0015() {
    te_bool(false, &scope!(), "false=true", false);
}

#[test]
fn _0016() {
    te_bool(false, &scope!(), "true=true", true);
}

#[test]
fn _0017() {
    te_bool(false, &scope!(), "true=null", false);
}

#[test]
fn _0018() {
    te_bool(false, &scope!(), r#"(10 = 10)"#, true);
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"10 = ({c1: {a: {c: "bar", b: "foo"}}})"#,
        r#"equal err '10' =?= '{c1: {a: {b: "foo", c: "bar"}}}'"#,
    );
}

#[test]
fn _0020() {
    te_bool(
        false,
        &scope!(),
        r#"{a: {c: "bar", b: "foo"}} = {a: {b: "foo", c: "bar"}}"#,
        true,
    );
}

#[test]
fn _0021() {
    te_bool(
        false,
        &scope!(),
        r#"{a: {c: "bar", b: "foo"}} = {a: {b: "foo", c: "bars"}}"#,
        false,
    );
}

#[test]
fn _0022() {
    te_bool(
        false,
        &scope!(),
        r#"{a: {b: "bar", c: "car"}} = {a: {b: "bar", c: "car", d: "dar"}}"#,
        false,
    );
}

#[test]
fn _0023() {
    te_bool(
        false,
        &scope!(),
        r#"{a: {b: "bar", c: "car"}} = {a: {b: "bar", c: null}}"#,
        false,
    );
}

#[test]
fn _0024() {
    te_null(
        false,
        &scope!(),
        r#"{a: {b: "bar", c: {x: "car"}}} = {a: {b: "bar", c: 15}}"#,
        r#"equal err '{a: {b: "bar", c: {x: "car"}}}' =?= '{a: {b: "bar", c: 15}}'"#,
    );
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#"{a: {b: "bar", c: "car"}} = null"#,
        false,
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#"{a: {b: "bar", c: "car"}} = 15"#,
        r#"equal err '{a: {b: "bar", c: "car"}}' =?= '15'"#,
    );
}

#[test]
fn _0027() {
    te_bool(false, &scope!(), r#"[1,2,3] = [1,2,3]"#, true);
}

#[test]
fn _0028() {
    te_bool(false, &scope!(), r#"[1,2,3] = [1,2,3,4]"#, false);
}

#[test]
fn _0029() {
    te_bool(false, &scope!(), r#"[1,2,3] = null"#, false);
}

#[test]
fn _0030() {
    te_null(
        false,
        &scope!(),
        r#"[1,2,3] = "a" "#,
        r#"equal err '[1, 2, 3]' =?= '"a"'"#,
    );
}

#[test]
fn _0031() {
    te_bool(false, &scope!(), r#" @"P0D" = @"-P0D" "#, true);
}

#[test]
fn _0032() {
    te_bool(
        false,
        &scope!(),
        r#" @"2002-04-02T23:00:00-04:00" = @"2002-04-03T02:00:00-01:00" "#,
        true,
    );
}

#[test]
fn _0033() {
    te_bool(
        false,
        &scope!(),
        r#" date and time("2018-12-08T00:00:00+00:00") = date and time("2018-12-08T00:00:00@Etc/UTC") "#,
        true,
    );
}

#[test]
fn _0034() {
    te_bool(
        false,
        &scope!(),
        r#" @"2018-12-08T00:00:00+01:00" = @"2018-12-08T00:00:00Z" "#,
        false,
    );
}

#[test]
fn _0035() {
    te_bool(
        false,
        &scope!(),
        r#" @"262142-01-01T23:59:59Z" = @"262142-01-01T23:59:59Z" "#,
        true,
    );
}

#[test]
fn _0036() {
    te_bool(
        false,
        &scope!(),
        r#" @"262142-01-01T23:59:58Z" = @"262142-01-01T23:59:59Z" "#,
        false,
    );
}

#[test]
fn _0037() {
    te_bool(false, &scope!(), r#" @"2018-12-08" = null "#, false);
}

#[test]
fn _0038() {
    te_null(
        false,
        &scope!(),
        r#" @"2018-12-08" = 2018 "#,
        r#"equal err '2018-12-08' =?= '2018'"#,
    );
}

#[test]
fn _0039() {
    te_bool(
        false,
        &scope!(),
        r#" @"2018-12-08T10:11:12" = null "#,
        false,
    );
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        r#" @"2018-12-08T10:11:12" = 2018 "#,
        r#"equal err '2018-12-08T10:11:12' =?= '2018'"#,
    );
}

#[test]
fn _0041() {
    te_bool(false, &scope!(), r#" @"10:11:12" = null "#, false);
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#" @"10:11:12" = 10 "#,
        r#"equal err '10:11:12' =?= '10'"#,
    );
}

#[test]
fn _0043() {
    te_bool(false, &scope!(), r#" @"P1D" = null "#, false);
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#" @"P1D" = "P1D" "#,
        r#"equal err 'P1D' =?= '"P1D"'"#,
    );
}

#[test]
fn _0045() {
    te_bool(false, &scope!(), r#" @"P1Y" = null "#, false);
}

#[test]
fn _0046() {
    te_null(
        false,
        &scope!(),
        r#" @"P1Y" = "P1Y" "#,
        r#"equal err 'P1Y' =?= '"P1Y"'"#,
    );
}

#[test]
fn _0047() {
    te_bool(false, &scope!(), r#" <= 10 = (null..10] "#, false);
}

#[test]
fn _0048() {
    te_null(
        false,
        &scope!(),
        r#" <= 10 = (null) "#,
        "equal err 'UnaryLessOrEqual(10)' =?= 'null'",
    );
}

#[test]
fn _0049() {
    te_bool(false, &scope!(), r#" <= 10 = [null..10] "#, false);
}

#[test]
fn _0050() {
    te_bool(false, &scope!(), r#" < 10 = (null..10) "#, false);
}

#[test]
fn _0051() {
    te_null(
        false,
        &scope!(),
        r#" < 10 = (null) "#,
        "equal err 'UnaryLess(10)' =?= 'null'",
    );
}

#[test]
fn _0052() {
    te_bool(false, &scope!(), r#" < 10 = (null..10] "#, false);
}

#[test]
fn _0053() {
    te_bool(false, &scope!(), r#" >= 10 = [10..null) "#, false);
}

#[test]
fn _0054() {
    te_null(
        false,
        &scope!(),
        r#" >= 10 = (null) "#,
        "equal err 'UnaryGreaterOrEqual(10)' =?= 'null'",
    );
}

#[test]
fn _0055() {
    te_bool(false, &scope!(), r#" >= 10 = [10..null] "#, false);
}

#[test]
fn _0056() {
    te_bool(false, &scope!(), r#" >= 10 = [10..null) "#, false);
}

#[test]
fn _0057() {
    te_bool(false, &scope!(), r#" >= 10 = [10..null] "#, false);
}

#[test]
fn _0058() {
    te_bool(false, &scope!(), r#" > 10 = (10..null) "#, false);
}

#[test]
fn _0059() {
    te_null(
        false,
        &scope!(),
        r#" > 10 = (null) "#,
        "equal err 'UnaryGreater(10)' =?= 'null'",
    );
}

#[test]
fn _0060() {
    te_bool(false, &scope!(), r#" > 10 = (10..null] "#, false);
}

#[test]
fn _0061() {
    te_bool(false, &scope!(), r#" [1..2] = [1..2] "#, true);
}

#[test]
fn _0062() {
    te_bool(false, &scope!(), r#" [1..3] = [1..2] "#, false);
}

#[test]
fn _0063() {
    te_bool(false, &scope!(), r#" [1..2] = [1..2) "#, false);
}

#[test]
fn _0064() {
    te_bool(false, &scope!(), r#" (1..2] = [1..2] "#, false);
}

#[test]
fn _0065() {
    te_bool(false, &scope!(), r#" (1..2] = null "#, false);
}

#[test]
fn _0066() {
    te_null(
        false,
        &scope!(),
        r#" (1..2] = 10 "#,
        r#"equal err '(1..2]' =?= '10'"#,
    );
}
