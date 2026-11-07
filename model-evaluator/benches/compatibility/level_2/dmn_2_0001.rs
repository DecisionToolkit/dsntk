use super::*;

from_examples!(DMN_2_0001);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{Full Name: "John Doe"}"#);
    let invocable = "Greeting Message";
    let expected = r#""Hello John Doe""#;
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
