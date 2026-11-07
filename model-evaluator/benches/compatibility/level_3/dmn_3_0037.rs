use super::*;

from_examples!(DMN_3_0037);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{Person: {Children: 3,Gender: "Male",Name: "Bob"}}"#);
    let invocable_name = "Person's description";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""Bob is a dad of 3 children.""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{Person: {Children: 2,Gender: "Female",Name: "Alice"}}"#);
    let invocable_name = "Person's description";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""Alice is a mother of 2 children.""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
