use super::*;

from_examples!(DMN_N_0015);

static_context!(CTX_1, r#"{ a: 10, b: 9, c: 1 }"#);
static_context!(CTX_2, r#"{ a: 10, b: 11, c: 1 }"#);

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "and1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_1,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_1)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "or1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_1,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_1)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "literalList";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_1,
        r#"[true, true]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_1)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable_name = "and2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_1,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_1)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable_name = "or2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_1,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_1)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable_name = "and1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_2,
        r#"false"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_2)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "or1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_2,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_2)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "literalList";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_2,
        r#"[false, true]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_2)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let invocable_name = "and2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_2,
        r#"false"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_2)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let invocable_name = "or2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX_2,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX_2)
    );
}
