use super::*;

from_examples!(DMN_3_0034);

#[test]
fn _0001() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision A 1",
        &ctx,
        r#"{resolve A: "A"}"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision A 2.1",
        &ctx,
        r#"{resolve A 1: {resolve A: "A"}}"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision A 2.2",
        &ctx,
        r#"{resolve A 1: {resolve A: "A"}}"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision A 3",
        &ctx,
        r#"{resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision B 1",
        &ctx,
        r#"{resolve A: "A", resolve B: "B"}"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision B 2.1",
        &ctx,
        r#"{resolve B 1: {resolve A: "A", resolve B: "B"}}"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision B 2.2",
        &ctx,
        r#"{resolve B 1: {resolve A: "A", resolve B: "B"}}"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision B 3",
        &ctx,
        r#"{resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 2.1: {resolve B 1: {resolve A: "A", resolve B: "B"}}, resolve B 2.2: {resolve B 1: {resolve A: "A", resolve B: "B"}}}"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision C 1",
        &ctx,
        r#"{resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 3: {resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 2.1: {resolve B 1: {resolve A: "A", resolve B: "B"}}, resolve B 2.2: {resolve B 1: {resolve A: "A", resolve B: "B"}}}, resolve C: "C"}"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision C 2",
        &ctx,
        r#""BKM I # BKM II # BKM III # decision C 2 # BKM IV # BKM III # decision C 2""#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision C 3",
        &ctx,
        r#""BKM II # BKM III # decision C 3 # BKM IV # BKM III # decision C 3""#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision C 4",
        &ctx,
        r#"{resolve C 3: "BKM II # BKM III # decision C 3 # BKM IV # BKM III # decision C 3"}"#,
    );
}
