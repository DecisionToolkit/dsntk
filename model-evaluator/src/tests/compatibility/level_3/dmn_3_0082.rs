use super::*;

from_examples!(DMN_3_0082);
static_context!(CTX, "{}");

#[test]
fn _0001() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_001",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0002() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_003",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0003() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_004",
        &CTX,
        r#"{age: 10, name: "foo", surname: "bar"}"#,
    );
}

#[test]
fn _0004() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_005",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0005() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_006",
        &CTX,
        r#"["foo"]"#,
    );
}

#[test]
fn _0006() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_006_a",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0007() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_007",
        &CTX,
        r#""foo""#,
    );
}

#[test]
fn _0008() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_007_a",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0009() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_008",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0010() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_bkm_001",
        &CTX,
        r#"true"#,
    );
}

#[test]
fn _0011() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_bkm_002",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0012() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_bkm_003",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0013() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_bkm_004_a",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0014() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_bkm_004_b",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0015() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_bkm_005",
        &CTX,
        r#"10"#,
    );
}

#[test]
fn _0016() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_bkm_005_a",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0017() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "invoke_001",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0018() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "invoke_002",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0019() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "invoke_004",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0020() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "invoke_005",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0021() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "invoke_006",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0022() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "invoke_007",
        &CTX,
        "null(argument not provided: arg)",
    );
}

#[test]
fn _0023() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "fd_001",
        &CTX,
        r#"10"#,
    );
}

#[test]
fn _0024() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "fd_002",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0025() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literal_001",
        &CTX,
        r#"10"#,
    );
}

#[test]
fn _0026() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literal_002",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0027() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literal_004",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0028() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literal_005",
        &CTX,
        r#"10"#,
    );
}

#[test]
fn _0029() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "literal_006",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0030() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "ds_invoke_002_with_number",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0031() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "ds_invoke_002_with_singleton_list",
        &CTX,
        r#""foo""#,
    );
}

#[test]
fn _0032() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decisionService_001",
        &CTX,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0033() {
    let ctx = context(r#"{decisionService_002_input_1: 10}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decisionService_002",
        &ctx,
        r#"null(after coercion)"#,
    );
}

#[test]
fn _0034() {
    let ctx = context(r#"{decisionService_002_input_1: ["foo"]}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "decision_ds_002",
        &ctx,
        r#""foo""#,
    );
}

#[test]
fn _0035() {
    let invocable = "decision_context_01";
    let expected = r#"{a: 1, b: 2}"#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
}

#[test]
fn _0036() {
    let invocable = "decision_context_02";
    let expected = r#"{a: 1, b: 2}"#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
}

#[test]
fn _0037() {
    let invocable = "decision_context_03";
    let expected = r#"{a: 1, b: 2}"#;
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
}
