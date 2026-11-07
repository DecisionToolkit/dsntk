use super::*;

from_examples!(DMN_3_1131);
static_context!(CTX, "{}");

#[test]
fn _0001() {
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
}

#[test]
fn _0002() {
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
}

#[test]
fn _0003() {
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
}

#[test]
fn _0004() {
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
}

#[test]
fn _0005() {
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
}

#[test]
fn _0006() {
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
}

#[test]
fn _0007() {
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
}

#[test]
fn _0008() {
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
}
