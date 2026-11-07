use super::*;

from_examples!(DMN_2_0009);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{Loan: {amount: 600000, rate: 0.0375, term: 360},fee: 100}"#);
    let invocable_name = "MonthlyPayment";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2878.693549432766768088520383236288"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{Loan: {amount: 30000, rate: 0.0475, term: 60}, fee: 100}"#);
    let invocable_name = "MonthlyPayment";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"662.7073593732659271562143285576407"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(r#"{Loan: {amount: 600000, rate: 0.0399, term: 360}, fee: 100}"#);
    let invocable_name = "MonthlyPayment";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2961.03377700390163671626277960579"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
