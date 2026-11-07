use super::*;

from_examples!(DMN_3_0066);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "decision001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"false"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "decision002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "decision003_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::not] invalid argument type, expected boolean, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "decision003_b";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::not] invalid argument type, expected boolean, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "decision003_c";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::not] invalid argument type, expected boolean, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "decision003_d";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::not] invalid argument type, expected boolean, actual type is string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
