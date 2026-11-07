use super::*;

from_examples!(DMN_2_0107);

#[test]
fn _0001() {
    let ctx = context(r#"{A: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionNot",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{A: false}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionNot",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{A: null}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionNot",
        &ctx,
        r#"null([core::not] invalid argument type, expected boolean, actual type is Null)"#,
    );
}
