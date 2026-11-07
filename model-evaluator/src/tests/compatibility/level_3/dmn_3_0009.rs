use super::*;

from_examples!(DMN_3_0009);

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
        "append1",
        &ctx,
        r#"[["w", "x"], ["y"], ["z"], ["t"]]"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "append2",
        &ctx,
        r#"[["w", "x"], ["y"], ["z"], ["a", "b", "c"]]"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "append3",
        &ctx,
        r#"[["w", "x"], ["y"], ["z"], ["a", "b", "c"]]"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "append4",
        &ctx,
        r#"[["w", "x"], ["y"], ["z"], ["a", "b", "c"]]"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "flatten1",
        &ctx,
        r#"["w", "x", "y", "z", "t"]"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "flatten2",
        &ctx,
        r#"["w", "x", "y", "z", "a", "b", "c"]"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "flatten3",
        &ctx,
        r#"["w", "x", "y", "z", "a", "b", "c"]"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{nestedList: [["w", "x"], ["y"], ["z"]], simpleList: ["a", "b", "c"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "flatten4",
        &ctx,
        r#"["w", "x", "y", "z", "a", "b", "c"]"#,
    );
}
