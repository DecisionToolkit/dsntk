use super::*;

from_examples!(DMN_2_0004);

#[bench]
fn _0001(b: &mut Bencher) {
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
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
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
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
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
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &ctx)
    );
}
