use super::*;

from_examples!(DMN_3_0013);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(
        r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
    );
    let invocable_name = "sort1";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"[5, 4, 3, 1]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(
        r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
    );
    let invocable_name = "sort2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"[{col1: 1, col2: 0, col3: 1, col4: 1}, {col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(
        r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
    );
    let invocable_name = "sort3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"["10", "8", "9", "A", "Aa", "a"]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
