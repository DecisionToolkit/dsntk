use super::*;

from_examples!(DMN_3_0039);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(
        r#"{Flu Symtoms:  ["fever", "cough", "sore throat", "runny nose"], Symptom:  "cough"}"#,
    );
    let invocable_name = "Symptom Analysis";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"["cough is in the list of Cold symptoms", "cough is in the list of Flu symptoms"]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx =
        context(r#"{Flu Symtoms: ["fever","cough","sore throat","runny nose"],Symptom: "fever"}"#);
    let invocable_name = "Symptom Analysis";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"["fever is in the list of Flu symptoms"]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
