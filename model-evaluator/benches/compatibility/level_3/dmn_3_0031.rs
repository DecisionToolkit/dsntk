use super::*;

from_examples!(DMN_3_0031);
static CTX: LazyLock<FeelContext> = LazyLock::new(|| context(r#"{inputA: 10, inputB: 5}"#));

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "fn invocation positional parameters";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"{divisionResultPositional: 2, multiplicationResultPositional: 50, sumResult: 15}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "fn invocation named parameters";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"{divisionResultNamed: 2, multiplicationResultNamed: 50, subResult: 5, subResultMixed: -5}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "fn invocation complex parameters";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"{circumference: 94.24776, functionInvocationInParameter: 200, functionInvocationLiteralExpressionInParameter: 500}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}
