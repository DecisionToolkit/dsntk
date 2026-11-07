use super::*;

from_examples!(DMN_3_0016);
static_context!(CTX, "{}");

#[test]
fn _0001() {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "priceTable1",
        &ctx,
        r#"[{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "everyGtTen1",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "everyGtTen2",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "someGtTen1",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "someGtTen2",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "everyGtTen3",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0007() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_014",
        &CTX,
        r#"false"#,
    );
}

#[test]
fn _0008() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_014a",
        &CTX,
        r#"true"#,
    );
}
