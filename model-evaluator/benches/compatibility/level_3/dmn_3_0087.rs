use super::*;

from_examples!(DMN_3_0087);
static CTX: LazyLock<FeelContext> = LazyLock::new(|| {
    context(
        r#"
    {
      Applicant data: {
        Age: 51,
        EmploymentStatus: "EMPLOYED",
        ExistingCustomer: false,
        MartitalStatus: "M",
        Monthly: {
          Expenses: 10000,
          Income: 100000,
          Repayments: 2500
        }
      },
      Bureau data: {
        Bankrupt: false,
        CreditScore: 600
      },
      Requested product: {
        Amount: 100000,
        ProductType: "STANDARD LOAN",
        Rate: 0.08,
        Term: 36
      }
    }"#,
    )
});

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "Strategy";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""THROUGH""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "Routing";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""ACCEPT""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "Application risk score";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"138"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable_name = "Pre-bureau risk category";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""VERY LOW""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable_name = "Bureau call type";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""NONE""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable_name = "Eligibility";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""ELIGIBLE""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "Post-bureau affordability";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "Strategy";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""THROUGH""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let invocable_name = "Routing";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""ACCEPT""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}
