use super::*;

from_examples!(DMN_3_0002);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{A: "banana", B: "a", NumC: 2}"#);
    let invocable_name = "Basic";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{containsB: true, containsX: false, endsWithB: true, endsWithX: false, lowercase: "a", startsWithB: false, startsWithX: false, stringlength: 6, substringAfterB: "nana", substringBeforeB: "b", substringC1: "a", uppercase: "BANANA"}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{A: "banana"}"#);
    let invocable_name = "Matches";
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
    let ctx = context(r#"{A: "banana"}"#);
    let invocable_name = "Replace";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{AanplusStarstar: "b**a", Aao: "bonono", encloseVowels: "b[a]n[a]n[a]"}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(r#"{NumC: 2}"#);
    let invocable_name = "Constructor";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
