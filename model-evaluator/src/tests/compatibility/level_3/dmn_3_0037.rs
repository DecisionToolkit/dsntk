use super::*;

from_examples!(DMN_3_0037);

#[test]
fn _0001() {
    let ctx = context(r#"{Person: {Children: 3,Gender: "Male",Name: "Bob"}}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Person's description",
        &ctx,
        r#""Bob is a dad of 3 children.""#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Person: {Children: 2,Gender: "Female",Name: "Alice"}}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Person's description",
        &ctx,
        r#""Alice is a mother of 2 children.""#,
    );
}
