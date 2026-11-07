use super::*;

from_examples!(DMN_3_0004);

const APPLICANT_DATA: &str = r#"
  {
    ApplicantData:  {
      Age:  35,
      EmploymentStatus:  "EMPLOYED",
      ExistingCustomer:  true,
      MaritalStatus:  "M",
      Monthly:  {
        Expenses:  2000,
        Income:  6000,
        Repayments:  0
      }
    },
    BureauData:  {
      Bankrupt:  false,
      CreditScore:  649
    },
    RequestedProduct:  {
      Amount:  350000,
      ProductType:  "STANDARD LOAN",
      Rate:  0.0395,
      Term:  360
    },
    SupportingDocuments:  "YES"
  }
"#;

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Adjudication";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""ACCEPT""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "ApplicationRiskScore";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"130"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Pre-bureauRiskCategory";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""LOW""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "BureauCallType";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""MINI""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Post-bureauRiskCategory";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""LOW""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "RequiredMonthlyInstallment";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"1680.8803256086347968"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Pre-bureauAffordability";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Eligibility";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""ELIGIBLE""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Strategy";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""BUREAU""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Post-bureauAffordability";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"true"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    let ctx = context(APPLICANT_DATA);
    let invocable_name = "Routing";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""ACCEPT""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
