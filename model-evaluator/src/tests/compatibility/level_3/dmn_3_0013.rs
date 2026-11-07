use super::*;

from_examples!(DMN_3_0013);

#[test]
fn _0001() {
    let ctx = context(
        r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "sort1",
        &ctx,
        r#"[5, 4, 3, 1]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(
        r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "sort2",
        &ctx,
        r#"[{col1: 1, col2: 0, col3: 1, col4: 1}, {col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}]"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(
        r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "sort3",
        &ctx,
        r#"["10", "8", "9", "A", "Aa", "a"]"#,
    );
}
