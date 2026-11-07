use super::*;

from_examples!(DMN_3_0031);

static_context!(CTX, r#"{inputA: 10, inputB: 5}"#);

#[test]
fn _0001() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "fn invocation positional parameters",
        &CTX,
        r#"{divisionResultPositional: 2, multiplicationResultPositional: 50, sumResult: 15}"#,
    );
}

#[test]
fn _0002() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "fn invocation named parameters",
        &CTX,
        r#"{divisionResultNamed: 2, multiplicationResultNamed: 50, subResult: 5, subResultMixed: -5}"#,
    );
}

#[test]
fn _0003() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "fn invocation complex parameters",
        &CTX,
        r#"{circumference: 94.24776, functionInvocationInParameter: 200, functionInvocationLiteralExpressionInParameter: 500}"#,
    );
}
