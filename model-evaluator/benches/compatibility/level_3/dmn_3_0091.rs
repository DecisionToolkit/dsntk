use super::*;

from_examples!(DMN_3_0091);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{ input_001: "input_001" }"#);
    let invocable = "decision_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_002",
        &ctx,
        r#""decision_001 input_001 bkm_001""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &ctx)
    );
}
