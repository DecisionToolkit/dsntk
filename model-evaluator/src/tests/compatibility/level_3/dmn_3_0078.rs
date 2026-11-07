use super::*;

from_examples!(DMN_3_0078);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_001",
        &ctx,
        r#"null([division] division by zero)"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_002",
        &ctx,
        r#"null([division] division by zero)"#,
    );
}
