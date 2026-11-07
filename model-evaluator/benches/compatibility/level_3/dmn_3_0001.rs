use super::*;

from_examples!(DMN_3_0001);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(
        r#"{Employees: [{dept: 10, id: 7792, name: "Clark"}, {dept: 10, id: 7934, name: "Miller"}, {dept: 20, id: 7976, name: "Adams"}, {dept: 20, id: 7902, name: "Ford"}, {dept: 30, id: 7900, name: "James"}]}"#,
    );
    let invocable_name = "Filter0001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"["Adams", "Ford"]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
