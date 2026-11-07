use super::*;

from_examples!(DMN_3_0010);

#[test]
fn _0001() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literalSimpleList",
        &ctx,
        r#"["a", "b", "c"]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literalNestedList",
        &ctx,
        r#"[["w", "x"], ["y"], ["z"]]"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "concatenate1",
        &ctx,
        r#"["a", "b", "c", "a", "b", "c"]"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "concatenate2",
        &ctx,
        r#"["a", "b", "c", "w", "x", "y", "z"]"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "concatenate3",
        &ctx,
        r#"["a", "b", "c", "w", "x", "y", "z"]"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "concatenate4",
        &ctx,
        r#"[["a", "b", "c"], ["w", "x"], ["y"], ["z"]]"#,
    );
}
