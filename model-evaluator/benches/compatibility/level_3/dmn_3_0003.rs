use super::*;

from_examples!(DMN_3_0003);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(
        r#"{Loans: [{amount: 200000,rate: .041,term: 360},{amount: 20000,rate: .049,term: 60}]}"#,
    );
    let invocable_name = "MonthlyPayment";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"[966.3967422049753602329651244861695, 376.5090706325024247283858289020851]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
