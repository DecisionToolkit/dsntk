use super::*;

from_examples!(DMN_3_0084);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
    let invocable_name = "decision_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[2, 3, 4]",
    );
}

#[test]
fn _0002() {
    let invocable_name = "decision_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[5, 6, 6, 7, 7, 8]",
    );
}

#[test]
fn _0003() {
    let invocable_name = "decision_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[]",
    );
}

#[test]
fn _0004() {
    let invocable_name = "decision_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[2, 3, 4]",
    );
}

#[test]
fn _0005() {
    let invocable_name = "decision_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[4, 3, 2]",
    );
}

#[test]
fn _0006() {
    let invocable_name = "decision_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[-1, 0, 1]",
    );
}

#[test]
fn _0007() {
    let invocable_name = "decision_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1, 0, -1]",
    );
}

#[test]
fn _0008() {
    let invocable_name = "decision_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1]",
    );
}

#[test]
fn _0009() {
    let invocable_name = "decision_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[2, 3, 4]",
    );
}

#[test]
fn _0010() {
    let invocable_name = "decision_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1, 1, 2, 6, 24]",
    );
}

#[test]
fn _0011() {
    let invocable_name = "decision_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[false, true]",
    );
}

#[test]
fn _0012() {
    let invocable_name = "decision_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1, 2, 3, 4]",
    );
}

#[test]
fn _0013() {
    let invocable_name = "decision_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[[1, 2], [3, 4]]",
    );
}

#[test]
fn _0014() {
    let invocable_name = "decision_017";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1980-01-01, 1980-01-02, 1980-01-03]",
    );
}

#[test]
fn _0015() {
    let invocable_name = "decision_018";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1980-01-03, 1980-01-02, 1980-01-01]",
    );
}

#[test]
fn _0016() {
    let invocable_name = "decision_019";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: string)",
    );
}

#[test]
fn _0017() {
    let invocable_name = "decision_019_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: string)",
    );
}

#[test]
fn _0018() {
    let invocable_name = "decision_020";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: date and time)",
    );
}

#[test]
fn _0019() {
    let invocable_name = "decision_020_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: date and time)",
    );
}

#[test]
fn _0020() {
    let invocable_name = "decision_021";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: time)",
    );
}

#[test]
fn _0021() {
    let invocable_name = "decision_021_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: time)",
    );
}

#[test]
fn _0022() {
    let invocable_name = "decision_022";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: days and time duration)",
    );
}

#[test]
fn _0023() {
    let invocable_name = "decision_022_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: days and time duration)",
    );
}

#[test]
fn _0024() {
    let invocable_name = "decision_023";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1, 2]",
    );
}

#[test]
fn _0025() {
    let invocable_name = "decision_024";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1980-01-01, 1980-01-02, 1980-01-03]",
    );
}

#[test]
fn _0026() {
    let invocable_name = "decision_025";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> range start must be less or equal then range end)",
    );
}
