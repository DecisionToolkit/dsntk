use super::*;

from_examples!(DMN_2_0108);

#[test]
fn _0001() {
    let ctx = context(r#"{Age: 19,RiskCategory: "Medium",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"{Rate: "Best", Status: "Approved"}"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Age: 13,RiskCategory: "Medium",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"{Rate: "Standard", Status: "Approved"}"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Age: 10,RiskCategory: "Low",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"{Rate: "Standard", Status: "Declined"}"#,
    );
}
