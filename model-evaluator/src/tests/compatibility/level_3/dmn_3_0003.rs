use super::*;

from_examples!(DMN_3_0003);

#[test]
fn _0001() {
    let ctx = context(
        r#"{Loans: [{amount: 200000,rate: .041,term: 360},{amount: 20000,rate: .049,term: 60}]}"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "MonthlyPayment",
        &ctx,
        r#"[966.3967422049753602329651244861695, 376.5090706325024247283858289020851]"#,
    );
}
