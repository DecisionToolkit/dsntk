use super::*;

from_examples!(DMN_2_0113);

#[test]
fn _0001() {
    let ctx = context(r#"{Age: 17}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval Status",
        &ctx,
        r#"["Approved", "Declined"]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Age: 19}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval Status",
        &ctx,
        r#"["Approved", "Approved"]"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Age: 10}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval Status",
        &ctx,
        r#"["Approved", "Declined"]"#,
    );
}
