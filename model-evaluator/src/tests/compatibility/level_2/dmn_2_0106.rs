use super::*;

from_examples!(DMN_2_0106);

#[test]
fn _0001() {
    let ctx = context(r#"{A: true,B: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{A: true,B: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{A: false,B: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{A: false,B: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{A: null,B: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{A: null,B: true}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{A: true,B: false}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{A: true,B: false}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{A: false,B: false}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{A: false,B: false}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{A: null,B: false}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{A: null,B: false}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{A: true,B: null}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{A: true,B: null}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{A: false,B: null}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{A: false,B: null}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{A: null,B: null}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionAnd",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{A: null,B: null}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "DecisionOr",
        &ctx,
        r#"null"#,
    );
}
