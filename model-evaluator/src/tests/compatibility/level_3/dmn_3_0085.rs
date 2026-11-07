use super::*;

from_examples!(DMN_3_0085);

const CTX: &str = "{}";

fn eqs(invocable: &str, input: &str, expected: &str) {
    let ctx = context(input);
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}

fn eqd(invocable: &str, input: &str, expected: &str) {
    let ctx = context(input);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
}

#[test]
fn _0001() {
    eqs("decision_001", CTX, r#""foo""#);
}

#[test]
fn _0002() {
    eqs("decisionService_001", CTX, r#""foo""#);
}

#[test]
fn _0003() {
    eqs(
        "decision_002",
        r#"{decision_002_input: "baz"}"#,
        r#""foo baz""#,
    );
}

#[test]
fn _0004() {
    eqs(
        "decisionService_002",
        r#"{decision_002_input: "baz"}"#,
        r#""foo baz""#,
    );
}

#[test]
fn _0005() {
    eqs("decision_002", CTX, r#""foo bar""#);
}

#[test]
fn _0006() {
    eqs(
        "decisionService_002",
        CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0007() {
    eqs(
        "decision_002",
        r#"{decision_002_input: null}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0008() {
    eqs(
        "decisionService_002",
        r#"{decision_002_input: null}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0009() {
    eqs(
        "decision_002",
        r#"{decision_002_input: 1234}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0010() {
    eqs(
        "decisionService_002",
        r#"{decision_002_input: 1234}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0011() {
    eqs(
        "decision_003",
        r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
        r#""A B C D""#,
    );
}

#[test]
fn _0012() {
    eqs(
        "decisionService_003",
        r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
        r#""A B C D""#,
    );
}

#[test]
fn _0013() {
    eqd("decision_004_1", CTX, r#""foo""#);
}

#[test]
fn _0014() {
    eqs("decisionService_005", CTX, r#""foo""#);
}

#[test]
fn _0015() {
    eqs(
        "decision_005_1",
        CTX,
        r#"null(invalid number of arguments)"#,
    );
}

#[test]
fn _0016() {
    eqs("decision_005_2", CTX, r#""foo""#);
}

#[test]
fn _0017() {
    eqs("decision_006_1", CTX, r#""foo bar""#);
}

#[test]
fn _0018() {
    eqs(
        "decision_007_1",
        CTX,
        r#"null(equal err 'null(after coercion)' =?= 'null')"#,
    );
}

#[test]
fn _0019() {
    eqs("decisionService_007", CTX, r#"true"#);
}

#[test]
fn _0020() {
    eqs(
        "decision_008_1",
        CTX,
        r#"null(invalid number of arguments)"#,
    );
}

#[test]
fn _0021() {
    eqs("decision_009_1", CTX, r#""foo bar""#);
}

#[test]
fn _0022() {
    eqs(
        "decision_010_1",
        CTX,
        r#"null(parameter with name decision_010_3 not found in arguments)"#,
    );
}

#[test]
fn _0023() {
    eqs("decision_011_1", CTX, r#""A B C D""#);
}

#[test]
fn _0024() {
    eqs("decision_012_1", CTX, r#""A B C D""#);
}

#[test]
fn _0025() {
    eqs(
        "decision_013_1",
        CTX,
        r#"{decisionService_013: "A B", decision_013_3: "D", inputData_013_1: null}"#,
    );
}

#[test]
fn _0026() {
    eqs(
        "decision_014_1",
        CTX,
        r#"{decisionService_014: "A B", decision_014_3: "D", inputData_014_1: null}"#,
    );
}
