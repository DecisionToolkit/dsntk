use super::*;

from_examples!(DMN_3_0064);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision001",
        &ctx,
        r#"true"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision002",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision003_a",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision003_b",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision003_c",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision004",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision005",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision006_a",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision006_b",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision006_c",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision007_a",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision007_b",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision007_c",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision008_a",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision008_b",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision008_c",
        &ctx,
        r#"false"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision009_a",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision009_b",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision009_c",
        &ctx,
        r#"null"#,
    );
}
