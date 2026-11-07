use super::*;

from_examples!(DMN_3_0088);

#[test]
fn _0001() {
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Evaluation DS",
        &context(
            r#"{Grade: "A",Student's name: "John Doe",Teacher's Evaluation: "A very motivated, hard-working student!"}"#,
        ),
        r#""John Doe is Graduated with merit with grade: A and evaluation: A very motivated, hard-working student!""#,
    );
}
