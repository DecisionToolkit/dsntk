use super::super::*;
use dsntk_feel::scope;

//----------------------------------------------------------------------------------------------------------------------
// NUMBER
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"0 in range("[1..3]")"#, false);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"1 in range("[1..3]")"#, true);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"2 in range("[1..3]")"#, true);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"3 in range("[1..3]")"#, true);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"4 in range("[1..3]")"#, false);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), r#"2 in range("(..1]")"#, false);
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), r#"1 in range("(..1]")"#, true);
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), r#"-2024 in range("(..1]")"#, true);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), r#"0 in range("[1..)")"#, false);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), r#"1 in range("[1..)")"#, true);
}

#[test]
fn _0011() {
    te_bool(false, &scope!(), r#"2024 in range("[1..)")"#, true);
}

//----------------------------------------------------------------------------------------------------------------------
// STRING
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0012() {
    te_bool(
        false,
        &scope!(),
        r#" "a" in range("[\"b\"..\"d\"]") "#,
        false,
    );
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#" "b" in range("[\"b\"..\"d\"]") "#,
        true,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#" "c" in range("[\"b\"..\"d\"]") "#,
        true,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#" "d" in range("[\"b\"..\"d\"]") "#,
        true,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#" "e" in range("[\"b\"..\"d\"]") "#,
        false,
    );
}

#[test]
fn _0017() {
    te_bool(false, &scope!(), r#" "e" in range("(..\"d\"]") "#, false);
}

#[test]
fn _0018() {
    te_bool(false, &scope!(), r#" "d" in range("(..\"d\"]") "#, true);
}

#[test]
fn _0019() {
    te_bool(false, &scope!(), r#" "a" in range("(..\"d\"]") "#, true);
}

#[test]
fn _0020() {
    te_bool(false, &scope!(), r#" "a" in range("[\"b\"..)") "#, false);
}

#[test]
fn _0021() {
    te_bool(false, &scope!(), r#" "b" in range("[\"b\"..)") "#, true);
}

#[test]
fn _0022() {
    te_bool(false, &scope!(), r#" "z" in range("[\"b\"..)") "#, true);
}

//----------------------------------------------------------------------------------------------------------------------
// DATE
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0023() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-05" in range("[@\"2024-01-06\"..@\"2024-01-08\"]") "#,
        false,
    );
}

#[test]
fn _0024() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-06" in range("[@\"2024-01-06\"..@\"2024-01-08\"]") "#,
        true,
    );
}

#[test]
fn _0025() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07" in range("[@\"2024-01-06\"..@\"2024-01-08\"]") "#,
        true,
    );
}

#[test]
fn _0026() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-08" in range("[@\"2024-01-06\"..@\"2024-01-08\"]") "#,
        true,
    );
}

#[test]
fn _0027() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-09" in range("[@\"2024-01-06\"..@\"2024-01-07\"]") "#,
        false,
    );
}

#[test]
fn _0028() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-08" in range("(..@\"2024-01-07\"]") "#,
        false,
    );
}

#[test]
fn _0029() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07" in range("(..@\"2024-01-07\"]") "#,
        true,
    );
}

#[test]
fn _0030() {
    te_bool(
        false,
        &scope!(),
        r#" @"2018-03-27" in range("(..@\"2024-01-07\"]") "#,
        true,
    );
}

#[test]
fn _0031() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-05" in range("[@\"2024-01-06\"..)") "#,
        false,
    );
}

#[test]
fn _0032() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-06" in range("[@\"2024-01-06\"..)") "#,
        true,
    );
}

#[test]
fn _0033() {
    te_bool(
        false,
        &scope!(),
        r#" @"2052-01-06" in range("[@\"2024-01-06\"..)") "#,
        true,
    );
}

//----------------------------------------------------------------------------------------------------------------------
// TIME
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0034() {
    te_bool(
        false,
        &scope!(),
        r#" @"10:00:00" in range( "[@\"11:00:00\"..@\"13:00:00\"]") "#,
        false,
    );
}

#[test]
fn _0035() {
    te_bool(
        false,
        &scope!(),
        r#" @"11:00:00" in range( "[@\"11:00:00\"..@\"13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0036() {
    te_bool(
        false,
        &scope!(),
        r#" @"12:00:00" in range( "[@\"11:00:00\"..@\"13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0037() {
    te_bool(
        false,
        &scope!(),
        r#" @"13:00:00" in range( "[@\"11:00:00\"..@\"13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0038() {
    te_bool(
        false,
        &scope!(),
        r#" @"14:00:00" in range( "[@\"11:00:00\"..@\"13:00:00\"]") "#,
        false,
    );
}

#[test]
fn _0039() {
    te_bool(
        false,
        &scope!(),
        r#" @"14:00:00" in range( "(..@\"13:00:00\"]") "#,
        false,
    );
}

#[test]
fn _0040() {
    te_bool(
        false,
        &scope!(),
        r#" @"13:00:00" in range( "(..@\"13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0041() {
    te_bool(
        false,
        &scope!(),
        r#" @"07:23:56" in range( "(..@\"13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0042() {
    te_bool(
        false,
        &scope!(),
        r#" @"10:00:00" in range( "[@\"11:00:00\"..)") "#,
        false,
    );
}

#[test]
fn _0043() {
    te_bool(
        false,
        &scope!(),
        r#" @"11:00:00" in range( "[@\"11:00:00\"..)") "#,
        true,
    );
}

#[test]
fn _0044() {
    te_bool(
        false,
        &scope!(),
        r#" @"18:32:21" in range( "[@\"11:00:00\"..)") "#,
        true,
    );
}

//----------------------------------------------------------------------------------------------------------------------
// DATE TIME
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0045() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T10:00:00" in range( "[@\"2024-01-07T11:00:00\"..@\"2024-01-07T13:00:00\"]") "#,
        false,
    );
}

#[test]
fn _0046() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T11:00:00" in range( "[@\"2024-01-07T11:00:00\"..@\"2024-01-07T13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0047() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T12:00:00" in range( "[@\"2024-01-07T11:00:00\"..@\"2024-01-07T13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0048() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T13:00:00" in range( "[@\"2024-01-07T11:00:00\"..@\"2024-01-07T13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0049() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T14:00:00" in range( "[@\"2024-01-07T11:00:00\"..@\"2024-01-07T13:00:00\"]") "#,
        false,
    );
}

#[test]
fn _0050() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T14:00:00" in range( "(..@\"2024-01-07T13:00:00\"]") "#,
        false,
    );
}

#[test]
fn _0051() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T13:00:00" in range( "(..@\"2024-01-07T13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0052() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T07:23:56" in range( "(..@\"2024-01-07T13:00:00\"]") "#,
        true,
    );
}

#[test]
fn _0053() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T10:00:00" in range( "[@\"2024-01-07T11:00:00\"..)") "#,
        false,
    );
}

#[test]
fn _0054() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T11:00:00" in range( "[@\"2024-01-07T11:00:00\"..)") "#,
        true,
    );
}

#[test]
fn _0055() {
    te_bool(
        false,
        &scope!(),
        r#" @"2024-01-07T18:32:21" in range( "[@\"2024-01-07T11:00:00\"..)") "#,
        true,
    );
}

//----------------------------------------------------------------------------------------------------------------------
// YEARS AND MONTHS DURATION
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0056() {
    te_bool(
        false,
        &scope!(),
        r#" @"P1Y" in range("[@\"P2Y\"..@\"P4Y\"]") "#,
        false,
    );
}

#[test]
fn _0057() {
    te_bool(
        false,
        &scope!(),
        r#" @"P2Y" in range("[@\"P2Y\"..@\"P4Y\"]") "#,
        true,
    );
}

#[test]
fn _0058() {
    te_bool(
        false,
        &scope!(),
        r#" @"P3Y" in range("[@\"P2Y\"..@\"P4Y\"]") "#,
        true,
    );
}

#[test]
fn _0059() {
    te_bool(
        false,
        &scope!(),
        r#" @"P4Y" in range("[@\"P2Y\"..@\"P4Y\"]") "#,
        true,
    );
}

#[test]
fn _0060() {
    te_bool(
        false,
        &scope!(),
        r#" @"P5Y" in range("[@\"P2Y\"..@\"P4Y\"]") "#,
        false,
    );
}

#[test]
fn _0061() {
    te_bool(
        false,
        &scope!(),
        r#" @"P5Y" in range("(..@\"P4Y\"]") "#,
        false,
    );
}

#[test]
fn _0062() {
    te_bool(
        false,
        &scope!(),
        r#" @"P4Y" in range("(..@\"P4Y\"]") "#,
        true,
    );
}

#[test]
fn _0063() {
    te_bool(
        false,
        &scope!(),
        r#" @"P1M" in range("(..@\"P4Y\"]") "#,
        true,
    );
}

#[test]
fn _0064() {
    te_bool(
        false,
        &scope!(),
        r#" @"P1Y" in range("[@\"P2Y\"..)") "#,
        false,
    );
}

#[test]
fn _0065() {
    te_bool(
        false,
        &scope!(),
        r#" @"P2Y" in range("[@\"P2Y\"..)") "#,
        true,
    );
}

#[test]
fn _0066() {
    te_bool(
        false,
        &scope!(),
        r#" @"P3Y2M" in range("[@\"P2Y\"..)") "#,
        true,
    );
}

//----------------------------------------------------------------------------------------------------------------------
// DAYS AND TIME DURATION
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn _0067() {
    te_bool(
        false,
        &scope!(),
        r#" @"P1D" in range("[@\"P2D\"..@\"P4D\"]") "#,
        false,
    );
}

#[test]
fn _0068() {
    te_bool(
        false,
        &scope!(),
        r#" @"P2D" in range("[@\"P2D\"..@\"P4D\"]") "#,
        true,
    );
}

#[test]
fn _0069() {
    te_bool(
        false,
        &scope!(),
        r#" @"P3D" in range("[@\"P2D\"..@\"P4D\"]") "#,
        true,
    );
}

#[test]
fn _0070() {
    te_bool(
        false,
        &scope!(),
        r#" @"P4D" in range("[@\"P2D\"..@\"P4D\"]") "#,
        true,
    );
}

#[test]
fn _0071() {
    te_bool(
        false,
        &scope!(),
        r#" @"P5D" in range("[@\"P2D\"..@\"P4D\"]") "#,
        false,
    );
}

#[test]
fn _0072() {
    te_bool(
        false,
        &scope!(),
        r#" @"P5D" in range("(..@\"P4D\"]") "#,
        false,
    );
}

#[test]
fn _0073() {
    te_bool(
        false,
        &scope!(),
        r#" @"P4D" in range("(..@\"P4D\"]") "#,
        true,
    );
}

#[test]
fn _0074() {
    te_bool(
        false,
        &scope!(),
        r#" @"PT1S" in range("(..@\"P4D\"]") "#,
        true,
    );
}

#[test]
fn _0075() {
    te_bool(
        false,
        &scope!(),
        r#" @"P1D" in range("[@\"P2D\"..)") "#,
        false,
    );
}

#[test]
fn _0076() {
    te_bool(
        false,
        &scope!(),
        r#" @"P2D" in range("[@\"P2D\"..)") "#,
        true,
    );
}

#[test]
fn _0077() {
    te_bool(
        false,
        &scope!(),
        r#" @"P3DT1S" in range("[@\"P2D\"..)") "#,
        true,
    );
}
