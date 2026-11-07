use super::*;

from_examples!(DMN_3_1131);
static_context!(CTX, "{}");

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable = "decision001";
    let expected = "null(expected built-in function name or function definition, actual is null(context has no value for key 'non_existing_function'))";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable = "decision002";
    let expected = "null(expected built-in function name or function definition, actual is null(context has no value for key 'null'))";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable = "decision003";
    let expected =
        r#"null(expected built-in function name or function definition, actual is "some_func")"#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable = "decision004";
    let expected =
        r#"null(expected built-in function name or function definition, actual is "abs")"#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable = "decision005";
    let expected =
        "null(expected built-in function name or function definition, actual is 2023-11-11)";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable = "decision006";
    let expected = "null(expected built-in function name or function definition, actual is 123)";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable = "decision007";
    let expected = "null(expected built-in function name or function definition, actual is null(context has no value for key 'true'))";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable = "decision008";
    let expected = "null(expected built-in function name or function definition, actual is null(context has no value for key 'false'))";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}
