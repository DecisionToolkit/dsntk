use super::*;

from_examples!(DMN_3_0012);

#[test]
fn _0001() {
    let ctx = context(r#"{ list1: ["a", "b", "c"], list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "listContainsList",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{ string1: "OK", list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "listContains",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{ list1: ["a", "b", "c"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "count1",
        &ctx,
        r#"3"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{ numList: [6, 14, -3] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "min1",
        &ctx,
        r#"-3"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{ numList: [6, 14, -3] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "sum1",
        &ctx,
        r#"17"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{ numList: [6, 14, -3] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "mean1",
        &ctx,
        r#"5.666666666666666666666666666666667"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{ num1: 11, num2: 2, num3: 10 }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "mean2",
        &ctx,
        r#"7.666666666666666666666666666666667"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{ list1: ["a", "b", "c"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "sublist12",
        &ctx,
        r#"["a", "b"]"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{ list1: ["a", "b", "c"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "sublistLast",
        &ctx,
        r#"["c"]"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{ num1: 11, num2: 2, num3: 10, numList: [6, 14, -3] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "append1",
        &ctx,
        r#"[6, 14, -3, 11, 2]"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{ list1: ["a", "b", "c"], list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "concatenate1",
        &ctx,
        r#"["a", "b", "c", "x", "y", "z"]"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{ string1: "OK", list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "insertBefore1",
        &ctx,
        r#"["x", "OK", "y", "z"]"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{ list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "remove2nd",
        &ctx,
        r#"["x", "z"]"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{ list1: ["a", "b", "c"], list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "reverse1",
        &ctx,
        r#"["z", "y", "x", "c", "b", "a"]"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{ list1: ["a", "b", "c"], string1: "x" }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "appendListItem",
        &ctx,
        r#"["a", "b", "c", "x"]"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{ string1: "OK", list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "indexOf1",
        &ctx,
        r#"[]"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{ string1: "OK", list1: ["a", "b", "c"], list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "union1",
        &ctx,
        r#"["x", "OK", "y", "z", "a", "b", "c"]"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{ string1: "OK", list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "distinctVals",
        &ctx,
        r#"["x", "OK", "y", "z"]"#,
    );
}

#[test]
fn _0020() {
    let ctx = context(r#"{ list1: ["a", "b", "c"], list2: ["x", "y", "z"] }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "flatten1",
        &ctx,
        r#"["a", "b", "c", "x", "y", "z"]"#,
    );
}
