use super::*;

from_examples!(DMN_3_0039);

#[test]
fn _0001() {
    let ctx = context(
        r#"{Flu Symtoms:  ["fever", "cough", "sore throat", "runny nose"], Symptom:  "cough"}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Symptom Analysis",
        &ctx,
        r#"["cough is in the list of Cold symptoms", "cough is in the list of Flu symptoms"]"#,
    );
}

#[test]
fn _0002() {
    let ctx =
        context(r#"{Flu Symtoms: ["fever","cough","sore throat","runny nose"],Symptom: "fever"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Symptom Analysis",
        &ctx,
        r#"["fever is in the list of Flu symptoms"]"#,
    );
}
