use super::*;

from_examples!(DMN_3_0001);

#[test]
fn _0001() {
    let ctx = context(
        r#"{Employees: [{dept: 10, id: 7792, name: "Clark"}, {dept: 10, id: 7934, name: "Miller"}, {dept: 20, id: 7976, name: "Adams"}, {dept: 20, id: 7902, name: "Ford"}, {dept: 30, id: 7900, name: "James"}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Filter0001",
        &ctx,
        r#"["Adams", "Ford"]"#,
    );
}
