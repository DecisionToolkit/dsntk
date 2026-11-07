use super::*;

from_examples!(DMN_2_0004);

#[test]
fn _0001() {
    let ctx = context(r#"{Age: 18,RiskCategory: "Medium",isAffordable: true}"#);
    let invocable = "Approval Status";
    let expected = r#""Approved""#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Age: 17,RiskCategory: "Medium",isAffordable: true}"#);
    let invocable = "Approval Status";
    let expected = r#""Declined""#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Age: 18,RiskCategory: "High",isAffordable: true}"#);
    let invocable = "Approval Status";
    let expected = r#""Declined""#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}
