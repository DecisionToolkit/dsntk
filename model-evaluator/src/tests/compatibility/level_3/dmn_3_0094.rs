use super::*;

from_examples!(DMN_3_0094);

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
    eq("decision001", &CTX, r#"5832"#);
}

#[test]
fn _0002() {
    eq("decision002", &CTX, r#"null"#);
}

#[test]
fn _0003() {
    eq(
        "decision003",
        &CTX,
        r#"null(expected 1+ parameters, actual number of parameters is 0)"#,
    );
}

#[test]
fn _0004() {
    eq(
        "decision003_a",
        &CTX,
        r#"null([core::product] invalid argument type, expected number, actual type is Null)"#,
    );
}

#[test]
fn _0005() {
    eq(
        "decision004",
        &CTX,
        r#"null([core::product] invalid argument type, expected number, actual type is Null)"#,
    );
}

#[test]
fn _0006() {
    eq(
        "decision005",
        &CTX,
        r#"null([core::product] invalid argument type, expected number, actual type is string)"#,
    );
}

#[test]
fn _0007() {
    eq("decision006", &CTX, r#"18.75"#);
}

#[test]
fn _0008() {
    eq("decision007", &CTX, r#"24"#);
}

#[test]
fn _0009() {
    eq("decision008", &CTX, r#"6"#);
}

#[test]
fn _0010() {
    eq("decision008_a", &CTX, r#"6"#);
}

#[test]
fn _0011() {
    eq("decision011", &CTX, r#"5832"#);
}

#[test]
fn _0012() {
    eq("decision012", &CTX, r#"null(parameter 'list' not found)"#);
}

#[test]
fn _0013() {
    eq("decision013", &CTX, r#"null(parameter 'list' not found)"#);
}
