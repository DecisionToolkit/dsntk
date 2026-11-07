use super::*;

from_examples!(DMN_3_0088);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(
        r#"{Grade: "A",Student's name: "John Doe",Teacher's Evaluation: "A very motivated, hard-working student!"}"#,
    );
    let invocable_name = "Evaluation DS";
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""John Doe is Graduated with merit with grade: A and evaluation: A very motivated, hard-working student!""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
