use super::super::*;

#[test]
fn _0001() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(0)", true);
}

#[test]
fn _0002() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(0.0)", true);
}

#[test]
fn _0003() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(2)", true);
}

#[test]
fn _0004() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(1)", false);
}

#[test]
fn _0005() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(-2)", true);
}

#[test]
fn _0006() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(-1)", false);
}

#[test]
fn _0007() {
    let scope = &te_scope("{}");
    te_null(
        false,
        scope,
        "even()",
        "expected 1 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0008() {
    let scope = &te_scope("{}");
    te_null(
        false,
        scope,
        "even(4,4)",
        "expected 1 parameters, actual number of parameters is 2",
    );
}

#[test]
fn _0009() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(number:4)", true);
}

#[test]
fn _0010() {
    let scope = &te_scope("{}");
    te_null(false, scope, "even(n:4)", r#"parameter 'number' not found"#);
}

#[test]
fn _0011() {
    let scope = &te_scope("{}");
    te_null(false, scope, "even(null)", "even");
}

#[test]
fn _0012() {
    let scope = &te_scope("{}");
    te_null(false, scope, r#"even("4")"#, "even");
}

#[test]
fn _0013() {
    let scope = &te_scope("{}");
    te_null(false, scope, "even(true)", "even");
}

#[test]
fn _0014() {
    let scope = &te_scope("{}");
    te_null(false, scope, "even(false)", "even");
}

#[test]
fn _0015() {
    let scope = &te_scope("{}");
    te_null(false, scope, r#"even(duration("P4D"))"#, "even");
}

#[test]
fn _0016() {
    let scope = &te_scope("{}");
    te_null(false, scope, r#"even(duration("P4Y"))"#, "even");
}

#[test]
fn _0017() {
    let scope = &te_scope("{}");
    te_null(false, scope, r#"even(date("2018-12-06"))"#, "even");
}

#[test]
fn _0018() {
    let scope = &te_scope("{}");
    te_null(false, scope, r#"even(time("00:00:00"))"#, "even");
}

#[test]
fn _0019() {
    let scope = &te_scope("{}");
    te_null(
        false,
        scope,
        r#"even(date and time("2018-12-06T00:00:00"))"#,
        "even",
    );
}

#[test]
fn _0020() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(2.35)", false);
}

#[test]
fn _0021() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(-2.35)", false);
}

#[test]
fn _0022() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(1.78)", false);
}

#[test]
fn _0023() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(-1.78)", false);
}

#[test]
fn _0024() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(2.0000)", true);
}

#[test]
fn _0025() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(-2.0000)", true);
}

#[test]
fn _0026() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "even(2.4)", false);
}
