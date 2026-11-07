use super::*;

from_examples!(DMN_3_0035);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{B Value: 83,G Value: 65,R Value: 0}"#);
    let invocable_name = "Profile of Color";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r##"{CMYK notation: {C: 100, K: 67, M: 22, Y: 0}, Hex notation: "#004153", RGB notation: {B: 83, G: 65, R: 0}}"##,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{B Value: 0,G Value: 0,R Value: 0}"#);
    let invocable_name = "Profile of Color";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r##"{CMYK notation: {C: 0, K: 100, M: 0, Y: 0}, Hex notation: "#000000", RGB notation: {B: 0, G: 0, R: 0}}"##,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(r#"{B Value: 0,G Value: 0,R Value: 204}"#);
    let invocable_name = "Profile of Color";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r##"{CMYK notation: {C: 0, K: 20, M: 100, Y: 100}, Hex notation: "#CC0000", RGB notation: {B: 0, G: 0, R: 204}}"##,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
