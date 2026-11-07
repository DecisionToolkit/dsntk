use super::*;

from_examples!(DMN_2_0112);

#[test]
fn _0001() {
    let ctx = context(r#"{Age: 19}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"["Best", "Standard"]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Age: 13}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"["Standard"]"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Age: 10}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"["Standard"]"#,
    );
}
