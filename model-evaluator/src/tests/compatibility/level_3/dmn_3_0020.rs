use super::*;

from_examples!(DMN_3_0020);

#[test]
fn _0001() {
    let ctx = context(r#"{Age: 16,Years of Service: 1}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Total Vacation Days",
        &ctx,
        r#"27"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{Age: 25,Years of Service: 5}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Total Vacation Days",
        &ctx,
        r#"22"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{Age: 25,Years of Service: 20}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Total Vacation Days",
        &ctx,
        r#"24"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{Age: 44,Years of Service: 30}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Total Vacation Days",
        &ctx,
        r#"30"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{Age: 50,Years of Service: 20}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Total Vacation Days",
        &ctx,
        r#"24"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{Age: 50,Years of Service: 30}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Total Vacation Days",
        &ctx,
        r#"30"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{Age: 60,Years of Service: 20}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Total Vacation Days",
        &ctx,
        r#"30"#,
    );
}
