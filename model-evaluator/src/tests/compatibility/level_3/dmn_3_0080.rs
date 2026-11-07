use super::*;

from_examples!(DMN_3_0080);
static_context!(CTX, "{}");

#[test]
fn _0001() {
    let invocable = "decision_001";
    let expected = "null(expected 2 parameters, actual number of parameters is 0)";
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
    let invocable = "decision_002";
    let expected = "null(expected 2 parameters, actual number of parameters is 1)";
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
    let invocable = "decision_003";
    let expected = "null(expected 2 parameters, actual number of parameters is 3)";
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
    let invocable = "decision_004";
    let expected = r#""foo""#;
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
    let invocable = "decision_005";
    let expected =
        "null([core::get value] invalid argument type, expected context, actual type is string)";
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
    let invocable = "decision_006";
    let expected =
        "null([core::get value] invalid argument type, expected string, actual type is number)";
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
    let invocable = "decision_007";
    let expected = r#""foo""#;
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
    let invocable = "decision_008";
    let expected = "null(parameter 'key' not found)";
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
fn _0009() {
    let invocable = "decision_009";
    let expected =
        "null([core::get value] invalid argument type, expected context, actual type is Null)";
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
fn _0010() {
    let invocable = "decision_010";
    let expected =
        "null([core::get value] invalid argument type, expected string, actual type is Null)";
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
fn _0011() {
    let invocable = "decision_011";
    let expected =
        "null([core::get value] invalid argument type, expected context, actual type is Null)";
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
fn _0012() {
    let invocable = "decision_012";
    let expected = "null";
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
fn _0013() {
    let ctx = context(r#"{ input_001: "a" }"#);
    let invocable = "decision_013";
    let expected = r#""foo""#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{ input_001: 12 }"#);
    let invocable = "decision_013";
    let expected =
        "null([core::get value] invalid argument type, expected string, actual type is number)";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}
