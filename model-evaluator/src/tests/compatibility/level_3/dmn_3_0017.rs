use super::*;

from_examples!(DMN_3_0017);

#[test]
fn _0001() {
    let ctx = context(r#"{structA: {name: "widget",price: 20}}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "priceGt10",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{numB: 9,numC: 10,structA: {name: "widget",price: 20}}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "priceInRange",
        &ctx,
        r#""Not in range""#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{dateD: @"2016-11-01"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "dateCompare1",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{dateD: @"2016-11-01",dateE: @"2016-11-02"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "dateCompare2",
        &ctx,
        r#"false"#,
    );
}
