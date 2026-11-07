use super::*;

from_examples!(DMN_3_0084);

static_context!(CTX, r#"{}"#);

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "decision_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[2, 3, 4]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "decision_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[5, 6, 6, 7, 7, 8]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "decision_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable_name = "decision_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[2, 3, 4]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable_name = "decision_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[4, 3, 2]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable_name = "decision_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[-1, 0, 1]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "decision_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[1, 0, -1]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "decision_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[1]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let invocable_name = "decision_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[2, 3, 4]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let invocable_name = "decision_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"[1, 1, 2, 6, 24]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    let invocable_name = "decision_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[false, true]",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    let invocable_name = "decision_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1, 2, 3, 4]",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    let invocable_name = "decision_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[[1, 2], [3, 4]]",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    let invocable_name = "decision_017";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1980-01-01, 1980-01-02, 1980-01-03]",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0015(b: &mut Bencher) {
    let invocable_name = "decision_018";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1980-01-03, 1980-01-02, 1980-01-01]",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    let invocable_name = "decision_019";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: string)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    let invocable_name = "decision_019_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: string)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    let invocable_name = "decision_020";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: date and time)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    let invocable_name = "decision_020_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: date and time)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0020(b: &mut Bencher) {
    let invocable_name = "decision_021";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: time)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    let invocable_name = "decision_021_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: time)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0022(b: &mut Bencher) {
    let invocable_name = "decision_022";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: days and time duration)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0023(b: &mut Bencher) {
    let invocable_name = "decision_022_a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> invalid type of interval start: days and time duration)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0024(b: &mut Bencher) {
    let invocable_name = "decision_023";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1, 2]",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0025(b: &mut Bencher) {
    let invocable_name = "decision_024";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "[1980-01-01, 1980-01-02, 1980-01-03]",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0026(b: &mut Bencher) {
    let invocable_name = "decision_025";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        "null(<FeelEvaluatorError> range start must be less or equal then range end)",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}
