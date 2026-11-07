use super::*;

from_examples!(DMN_2_0008);

#[test]
fn _0001() {
    let ctx = context(r#"{loan: {principal: 600000,rate: 0.0375,termMonths: 360}}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "payment",
        &ctx,
        r#"2778.693549432766768088520383236288"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{loan: {principal: 30000,rate: 0.0475,termMonths: 60}}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "payment",
        &ctx,
        r#"562.7073593732659271562143285576407"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{loan: {principal: 600000,rate: 0.0399,termMonths: 360}}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "payment",
        &ctx,
        r#"2861.03377700390163671626277960579"#,
    );
}
