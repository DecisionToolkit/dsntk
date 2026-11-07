use super::*;

from_examples!(DMN_3_0005);

#[test]
fn _0001() {
    let ctx = context(r#"{Loan: {amount: 600000,rate: 0.0375,term: 360},fee: 100}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "MonthlyPayment",
        &ctx,
        r#"2878.693549432766768088520383236288"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Loan: {amount: 30000,rate: 0.0475,term: 60},fee: 100}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "MonthlyPayment",
        &ctx,
        r#"662.7073593732659271562143285576407"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Loan: {amount: 600000,rate: 0.0399,term: 360},fee: 100}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "MonthlyPayment",
        &ctx,
        r#"2961.03377700390163671626277960579"#,
    );
}
