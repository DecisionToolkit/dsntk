use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_bool(false, &scope!(), "not(true)", false);
}

#[test]
fn _00011() {
    te_bool(false, &scope!(), "not(negand: true)", false);
}

#[test]
fn _0002() {
    te_bool(false, &scope!(), " not  (  true  ) ", false);
}

#[test]
fn _0003() {
    te_bool(false, &scope!(), "not(false)", true);
}

#[test]
fn _0004() {
    te_bool(false, &scope!(), " not  \n (  \t  false \r  ) \n  ", true);
}

#[test]
fn _0005() {
    let scope = &te_scope("{ On time: true, Too late: false }");
    te_bool(false, scope, "not(On time)", false);
}

#[test]
fn _0006() {
    let scope = &te_scope("{ On time: true, Too late: false }");
    te_bool(false, scope, "not(Too late)", true);
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        "not()",
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        "not(true,false)",
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        "not(n: true)",
        r#"parameter 'negand' not found"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        "not(12)",
        r#"[core::not] invalid argument type, expected boolean, actual type is number"#,
    );
}
