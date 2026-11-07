use super::*;

const DMN_0001: &str = include_str!("_0001.dmn");

model_evaluator!(DMN_0001);

const MODEL_NAMESPACE: &str = "https://www.decision-toolkit.org/input-data";
const MODEL_NAME: &str = "Adder";

#[test]
fn _0001() {
    let ctx = context(r#"{a: 1, b: 2}"#);
    let invocable = "Add";
    let expected = r#"3"#;
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{a: "Hello ", b: "world!"}"#);
    let invocable = "Add";
    let expected = r#""Hello world!""#;
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}
