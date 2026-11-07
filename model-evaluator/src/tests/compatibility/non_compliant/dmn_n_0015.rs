use super::*;

from_examples!(DMN_N_0015);

static_context!(CTX_1, r#"{ a: 10, b: 9, c: 1 }"#);
static_context!(CTX_2, r#"{ a: 10, b: 11, c: 1 }"#);

#[test]
fn _0001() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "and1",
        &CTX_1,
        r#"true"#,
    );
}

#[test]
fn _0002() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "or1",
        &CTX_1,
        r#"true"#,
    );
}

#[test]
fn _0003() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literalList",
        &CTX_1,
        r#"[true, true]"#,
    );
}

#[test]
fn _0004() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "and2",
        &CTX_1,
        r#"true"#,
    );
}

#[test]
fn _0005() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "or2",
        &CTX_1,
        r#"true"#,
    );
}

#[test]
fn _0006() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "and1",
        &CTX_2,
        r#"false"#,
    );
}

#[test]
fn _0007() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "or1",
        &CTX_2,
        r#"true"#,
    );
}

#[test]
fn _0008() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literalList",
        &CTX_2,
        r#"[false, true]"#,
    );
}

#[test]
fn _0009() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "and2",
        &CTX_2,
        r#"false"#,
    );
}

#[test]
fn _0010() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "or2",
        &CTX_2,
        r#"true"#,
    );
}
