use super::*;

static MODEL_EVALUATOR: LazyLock<Arc<ModelEvaluator>> =
    LazyLock::new(|| build_model_evaluators(&[DMN_3_0086_IMPORT, DMN_3_0086]));

const MODEL_NAMESPACE: &str = "https://dsntk.io/3_0086";
const MODEL_NAME: &str = "Import BKM and have a Decision Ctx with DT";

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{A Person: {age: 21,name: "John Doe"}}"#);
    let invocable_name = "A Decision Ctx with DT";
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        invocable_name,
        &ctx,
        r#""Hello John Doe!""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(MODEL_NAMESPACE, MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{A Person: {age: 47,name: "John Doe"}}"#);
    let invocable_name = "A Decision Ctx with DT";
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        invocable_name,
        &ctx,
        r#""Respectfully, Hello John Doe!""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(MODEL_NAMESPACE, MODEL_NAME, invocable_name, &ctx)
    );
}
