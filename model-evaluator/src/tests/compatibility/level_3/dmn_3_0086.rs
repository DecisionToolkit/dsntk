use super::*;
use dsntk_examples::*;

static MODEL_EVALUATOR: LazyLock<Arc<ModelEvaluator>> =
    LazyLock::new(|| build_model_evaluators(&[DMN_3_0086_IMPORT, DMN_3_0086]));

const MODEL_NAMESPACE: &str = "https://dsntk.io/3_0086";
const MODEL_NAME: &str = "Import BKM and have a Decision Ctx with DT";

#[test]
fn _0001() {
    let ctx = context(r#" { A Person: { age: 21, name: "John Doe"}} "#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "A Decision Ctx with DT",
        &ctx,
        r#""Hello John Doe!""#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#" { A Person: {age: 47,name: "John Doe"}} "#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "A Decision Ctx with DT",
        &ctx,
        r#""Respectfully, Hello John Doe!""#,
    );
}
