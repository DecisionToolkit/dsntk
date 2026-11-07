use super::*;

from_examples!(DMN_3_0091);

#[test]
fn _0001() {
    let ctx = context(r#"{ input_001: "input_001" }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_002",
        &ctx,
        r#""decision_001 input_001 bkm_001""#,
    );
}
