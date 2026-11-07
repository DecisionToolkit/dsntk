use super::*;

from_examples!(DMN_2_0110);

#[test]
fn _0001() {
    let ctx = context(r#"{Age: 17,RiskCategory: "High",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval Status",
        &ctx,
        r#"[{Approved/Declined: "Approved", Rate: "Standard"}, {Approved/Declined: "Declined", Rate: "Standard"}]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Age: 19,RiskCategory: "Low",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval Status",
        &ctx,
        r#"[{Approved/Declined: "Approved", Rate: "Basic"}]"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Age: 10,RiskCategory: "Low",isAffordable: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Approval Status",
        &ctx,
        r#"[{Approved/Declined: "Declined", Rate: "Standard"}]"#,
    );
}
