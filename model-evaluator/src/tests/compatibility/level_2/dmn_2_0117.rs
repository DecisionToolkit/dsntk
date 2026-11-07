use super::*;

from_examples!(DMN_2_0117);

#[test]
fn _0001() {
    let ctx = context(r#"{Age: 19,RiskCategory: "Low",isAffordable: true}"#);
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
    let ctx = context(r#"{Age: 17,RiskCategory: "High",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"{Rate: "Standard", Status: "Declined"}"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Age: 19,RiskCategory: "Medium",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval",
        &ctx,
        r#"{Rate: "Standard", Status: "Approved"}"#,
    );
}
