use super::*;

from_examples!(DMN_2_0114);

#[test]
fn _0001() {
    let ctx = context(r#"{NumOfYears: 5}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "CarInsurance",
        &ctx,
        r#"64.32"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{NumOfYears: 3}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "CarInsurance",
        &ctx,
        r#"98.83"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{NumOfYears: 4}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "CarInsurance",
        &ctx,
        r#"98.83"#,
    );
}
