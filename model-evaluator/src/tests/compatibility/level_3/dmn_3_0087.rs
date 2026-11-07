use super::*;

from_examples!(DMN_3_0087);

static_context!(
    CTX,
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
    }"#
);

#[test]
fn _0001() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Strategy",
        &CTX,
        r#""THROUGH""#,
    );
}

#[test]
fn _0002() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Routing",
        &CTX,
        r#""ACCEPT""#,
    );
}

#[test]
fn _0003() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Application risk score",
        &CTX,
        r#"138"#,
    );
}

#[test]
fn _0004() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Pre-bureau risk category",
        &CTX,
        r#""VERY LOW""#,
    );
}

#[test]
fn _0005() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Bureau call type",
        &CTX,
        r#""NONE""#,
    );
}

#[test]
fn _0006() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Eligibility",
        &CTX,
        r#""ELIGIBLE""#,
    );
}

#[test]
fn _0007() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Post-bureau affordability",
        &CTX,
        r#"true"#,
    );
}

#[test]
fn _0008() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Strategy",
        &CTX,
        r#""THROUGH""#,
    );
}

#[test]
fn _0009() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Routing",
        &CTX,
        r#""ACCEPT""#,
    );
}
