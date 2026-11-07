use super::*;

from_examples!(DMN_3_0034);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision A 1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve A: "A"}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision A 2.1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve A 1: {resolve A: "A"}}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision A 2.2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve A 1: {resolve A: "A"}}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision A 3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision B 1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve A: "A", resolve B: "B"}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision B 2.1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve B 1: {resolve A: "A", resolve B: "B"}}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision B 2.2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve B 1: {resolve A: "A", resolve B: "B"}}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision B 3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 2.1: {resolve B 1: {resolve A: "A", resolve B: "B"}}, resolve B 2.2: {resolve B 1: {resolve A: "A", resolve B: "B"}}}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision C 1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 3: {resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 2.1: {resolve B 1: {resolve A: "A", resolve B: "B"}}, resolve B 2.2: {resolve B 1: {resolve A: "A", resolve B: "B"}}}, resolve C: "C"}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision C 2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""BKM I # BKM II # BKM III # decision C 2 # BKM IV # BKM III # decision C 2""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision C 3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""BKM II # BKM III # decision C 3 # BKM IV # BKM III # decision C 3""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    let invocable_name = "decision C 4";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{resolve C 3: "BKM II # BKM III # decision C 3 # BKM IV # BKM III # decision C 3"}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
