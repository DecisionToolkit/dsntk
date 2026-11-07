use super::*;

from_examples!(DMN_3_0077);

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
