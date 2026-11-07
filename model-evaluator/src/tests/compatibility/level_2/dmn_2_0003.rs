use super::*;

from_examples!(DMN_2_0003);

#[test]
fn _0001() {
    let ctx = context(r#"{Employment Status: "EMPLOYED"}"#);
    let invocable = "Employment Status Statement";
    let expected = r#""You are EMPLOYED""#;
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
fn _0002() {
    let ctx = context(r#"{Employment Status: "RETIRED"}"#);
    let invocable = "Employment Status Statement";
    let expected = "null(expected string as a second argument in addition)";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}
