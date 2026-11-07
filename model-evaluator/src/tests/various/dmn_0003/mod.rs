use super::super::*;

const DMN_0003: &str = include_str!("_0003.dmn");

model_evaluator!(DMN_0003);

const MODEL_NAMESPACE: &str = "https://dsntk.io/services";
const MODEL_NAME: &str = "services";

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "input",
        &ctx,
        r#"27"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "output",
        &ctx,
        r#"27"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "ds",
        &ctx,
        r#"27"#,
    );
}
