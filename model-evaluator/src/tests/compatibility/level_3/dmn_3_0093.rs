use super::*;

from_examples!(DMN_3_0093);

static_context!(CTX, "{}");

fn eq(invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
}

#[test]
fn _0001() {
    eq("test_001", &CTX, r#"null(invalid @ literal: foo)"#);
}

#[test]
fn _0002() {
    eq("date_001", &CTX, r#"true"#);
}

#[test]
fn _0003() {
    eq("date_002", &CTX, r#"2019-03-31"#);
}

#[test]
fn _0004() {
    eq("datetime_001", &CTX, r#"true"#);
}

#[test]
fn _0005() {
    eq("datetime_002", &CTX, r#"2018-12-08T10:30:11"#);
}

#[test]
fn _0006() {
    eq("datetime_003", &CTX, r#"true"#);
}

#[test]
fn _0007() {
    eq(
        "datetime_004",
        &CTX,
        r#""2018-12-08T10:30:11@Australia/Melbourne""#,
    );
}

#[test]
fn _0008() {
    eq("datetime_005", &CTX, r#"true"#);
}

#[test]
fn _0009() {
    eq("datetime_006", &CTX, r#"2018-12-08T10:30:11+11:00"#);
}

#[test]
fn _0010() {
    eq("time_001", &CTX, r#"true"#);
}

#[test]
fn _0011() {
    eq("time_002", &CTX, r#"10:30:11"#);
}

#[test]
fn _0012() {
    eq("time_003", &CTX, r#"true"#);
}

#[test]
fn _0013() {
    eq("time_004", &CTX, r#"10:30:11+11:00"#);
}

#[test]
fn _0014() {
    eq("time_005", &CTX, r#"true"#);
}

#[test]
fn _0015() {
    eq("time_006", &CTX, r#""10:30:11@Australia/Melbourne""#);
}

#[test]
fn _0016() {
    eq("dt_duration_001", &CTX, r#"true"#);
}

#[test]
fn _0017() {
    eq("dt_duration_002", &CTX, r#"P10D"#);
}

#[test]
fn _0018() {
    eq("ym_duration_001", &CTX, r#"true"#);
}

#[test]
fn _0019() {
    eq("ym_duration_002", &CTX, r#"P10Y"#);
}
