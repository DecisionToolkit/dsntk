use super::*;

from_examples!(DMN_2_0002);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{Monthly Salary: 10000}"#);
    let invocable = "Yearly Salary";
    let expected = "120000";
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
