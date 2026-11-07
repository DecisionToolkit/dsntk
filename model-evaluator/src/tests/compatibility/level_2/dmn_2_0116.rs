use super::*;

from_examples!(DMN_2_0116);

#[test]
fn _0001() {
    let ctx = context(r#"{NumOfYears: 6}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Salary",
        &ctx,
        r#"4"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{NumOfYears: 2}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Salary",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{NumOfYears: 3}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Salary",
        &ctx,
        r#"2"#,
    );
}
