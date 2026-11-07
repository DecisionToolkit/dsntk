use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), r#"1!=1"#, false);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), r#"1.25!=2.11"#, true);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), r#"1!=2"#, true);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), r#"(1+1)!=2"#, false);
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"(1+2)!=2"#, true);
}

#[test]
fn _0006() {
    te_bool(false, &scope!(), r#" ( 1 + 2 ) != 2"#, true);
}

#[test]
fn _0007() {
    te_bool(false, &scope!(), r#" ( 1 + 3 ) != 4"#, false);
}

#[test]
fn _0008() {
    te_bool(false, &scope!(), r#"(1+0.3)!=(5-3)"#, true);
}

#[test]
fn _0009() {
    te_bool(false, &scope!(), r#"(1*2)!=(10/5.1)"#, true);
}

#[test]
fn _0010() {
    te_bool(false, &scope!(), r#"true != false"#, true);
}

#[test]
fn _0011() {
    te_bool(false, &scope!(), r#" "alfa" != "beta" "#, true);
}

#[test]
fn _0012() {
    te_bool(false, &scope!(), r#" "alfa" != "alfa" "#, false);
}

#[test]
fn _0013() {
    te_bool(
        false,
        &scope!(),
        r#" @"2023-02-06" != @"2023-02-07" "#,
        true,
    );
}

#[test]
fn _0014() {
    te_bool(
        false,
        &scope!(),
        r#" @"2023-02-06" != @"2023-02-06" "#,
        false,
    );
}

#[test]
fn _0015() {
    te_bool(
        false,
        &scope!(),
        r#" @"2023-02-06T10:11:12" != @"2023-02-06T10:11:13" "#,
        true,
    );
}

#[test]
fn _0016() {
    te_bool(
        false,
        &scope!(),
        r#" @"2023-02-06T10:11:12" != @"2023-02-06T10:11:12" "#,
        false,
    );
}

#[test]
fn _0017() {
    te_bool(false, &scope!(), r#" @"10:11:12" != @"10:11:13" "#, true);
}

#[test]
fn _0018() {
    te_bool(false, &scope!(), r#" @"10:11:12" != @"10:11:12" "#, false);
}

#[test]
fn _0019() {
    te_bool(false, &scope!(), r#" @"P1D" != @"P2D" "#, true);
}

#[test]
fn _0020() {
    te_bool(false, &scope!(), r#" @"P1D" != @"P1D" "#, false);
}

#[test]
fn _0021() {
    te_bool(false, &scope!(), r#" @"P1Y" != @"P2Y" "#, true);
}

#[test]
fn _0022() {
    te_bool(false, &scope!(), r#" @"P1Y" != @"P1Y" "#, false);
}
