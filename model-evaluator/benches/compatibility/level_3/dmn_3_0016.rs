use super::*;

from_examples!(DMN_3_0016);
static_context!(CTX, "{}");

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    let invocable_name = "priceTable1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"[{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    let invocable_name = "everyGtTen1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"false"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    let invocable_name = "everyGtTen2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"false"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    let invocable_name = "someGtTen1";
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
fn _0005(b: &mut Bencher) {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    let invocable_name = "someGtTen2";
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
fn _0006(b: &mut Bencher) {
    let ctx = context(
        r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#,
    );
    let invocable_name = "everyGtTen3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"false"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "decision_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"false"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "decision_014a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}
