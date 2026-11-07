use super::*;

from_examples!(DMN_3_0011);

#[test]
fn _0001() {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literalNestedList",
        &ctx,
        r#"[["a", "b"], ["b", "c"]]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "remove1",
        &ctx,
        r#"["a", "c"]"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "insert3",
        &ctx,
        r#"[["o"], ["a", "b", "c"], ["p", "q"]]"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "insert2",
        &ctx,
        r#"[["a", "b"], ["a", "b", "c"], ["b", "c"]]"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "remove2",
        &ctx,
        r#"[["a", "b"]]"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(
        r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "insert1",
        &ctx,
        r#"["a", "x", "b", "c"]"#,
    );
}
