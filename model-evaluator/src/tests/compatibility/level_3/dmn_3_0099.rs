use super::*;

from_examples!(DMN_3_0099);

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
    eq("decision_001", &CTX, r#"-10"#);
}

#[test]
fn _0002() {
    eq("decision_002", &CTX, r#"10"#);
}

#[test]
fn _0003() {
    eq("decision_003", &CTX, r#"-P1D"#);
}

#[test]
fn _0004() {
    eq("decision_003_a", &CTX, r#"P1D"#);
}

#[test]
fn _0005() {
    eq("decision_004", &CTX, r#"-P1Y"#);
}

#[test]
fn _0006() {
    eq("decision_004_a", &CTX, r#"P1Y"#);
}

#[test]
fn _0007() {
    eq(
        "decision_005",
        &CTX,
        r#"null(unexpected type in arithmetic negation: date)"#,
    );
}

#[test]
fn _0008() {
    eq(
        "decision_006",
        &CTX,
        r#"null(unexpected type in arithmetic negation: date and time)"#,
    );
}

#[test]
fn _0009() {
    eq(
        "decision_007",
        &CTX,
        r#"null(unexpected type in arithmetic negation: time)"#,
    );
}

#[test]
fn _0010() {
    eq(
        "decision_008",
        &CTX,
        r#"null(unexpected type in arithmetic negation: context<a: number>)"#,
    );
}

#[test]
fn _0011() {
    eq(
        "decision_009",
        &CTX,
        r#"null(unexpected type in arithmetic negation: string)"#,
    );
}

#[test]
fn _0012() {
    eq(
        "decision_010",
        &CTX,
        r#"null(unexpected type in arithmetic negation: list<number>)"#,
    );
}

#[test]
fn _0013() {
    eq(
        "decision_011",
        &CTX,
        r#"null(unexpected type in arithmetic negation: range<number>)"#,
    );
}

#[test]
fn _0014() {
    eq("decision_012", &CTX, r#"-10"#);
}
