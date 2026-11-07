use super::*;

from_examples!(DMN_3_0002);

#[test]
fn _0001() {
    let ctx = context(r#"{A: "banana", B: "a", NumC: 2}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Basic",
        &ctx,
        r#"{containsB: true, containsX: false, endsWithB: true, endsWithX: false, lowercase: "a", startsWithB: false, startsWithX: false, stringlength: 6, substringAfterB: "nana", substringBeforeB: "b", substringC1: "a", uppercase: "BANANA"}"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{A: "banana"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Matches",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{A: "banana"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Replace",
        &ctx,
        r#"{AanplusStarstar: "b**a", Aao: "bonono", encloseVowels: "b[a]n[a]n[a]"}"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{NumC: 2}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Constructor",
        &ctx,
        r#""2""#,
    );
}
