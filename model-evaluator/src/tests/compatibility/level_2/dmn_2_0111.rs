use super::*;

from_examples!(DMN_2_0111);

#[test]
fn _0001() {
    let ctx = context(r#"{age: 19}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Advertisement",
        &ctx,
        r#""Cars""#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{age: 13}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Advertisement",
        &ctx,
        r#""Videogames""#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{age: 5}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Advertisement",
        &ctx,
        r#""Toys""#,
    );
}
