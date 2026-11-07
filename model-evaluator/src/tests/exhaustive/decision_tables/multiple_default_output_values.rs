use super::*;

from_examples!(EXHAUSTIVE_DECISION_TABLES_MULTIPLE_DEFAULT_OUTPUT_VALUES);

static_context!(CTX_1, r#"{ Customer type: "Business", Order size: 9 }"#);
static_context!(CTX_2, r#"{ Customer type: "Business", Order size: 10 }"#);
static_context!(CTX_3, r#"{ Customer type: "Private", Order size: 100 }"#);
static_context!(CTX_4, r#"{ Customer type: "Private", Order size: 101 }"#);

fn eq(invocable_name: &str, input: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        input,
        expected,
    );
}

#[test]
fn _0001() {
    eq(
        "decision_0001",
        &CTX_1,
        r#"{Discount: 0.10, Priority: "Normal"}"#,
    );
}

#[test]
fn _0002() {
    eq(
        "decision_0001",
        &CTX_2,
        r#"{Discount: 0.15, Priority: "High"}"#,
    );
}

#[test]
fn _0003() {
    eq(
        "decision_0001",
        &CTX_3,
        r#"{Discount: 0.05, Priority: "Low"}"#,
    );
}

#[test]
fn _0004() {
    // No rule matches, so the default output values are returned as a context.
    eq(
        "decision_0001",
        &CTX_4,
        r#"{Discount: 0.06, Priority: "Normal"}"#,
    );
}
