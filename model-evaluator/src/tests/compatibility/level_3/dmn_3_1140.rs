use super::*;

from_examples!(DMN_3_1140);

static_context!(CTX, "{}");

fn eq(invocable: &str, expected: &str) {
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
fn _0001() {
    eq("decision001", r#""abc""#);
}

#[test]
fn _0002() {
    eq("decision002", r#""a and b and c""#);
}

#[test]
fn _0003() {
    eq("decision003", r#""abc""#);
}

#[test]
fn _0004() {
    eq("decision004", r#""abc""#);
}

#[test]
fn _0005() {
    eq("decision005_a", r#""a""#);
}

#[test]
fn _0006() {
    eq("decision005_b", r#""a""#);
}

#[test]
fn _0007() {
    eq("decision006_a", r#""ac""#);
}

#[test]
fn _0008() {
    eq("decision006_b", r#""aXc""#);
}

#[test]
fn _0009() {
    eq("decision007_a", r#""""#);
}

#[test]
fn _0010() {
    eq("decision007_b", r#""""#);
}

#[test]
fn _0011() {
    eq(
        "decision008",
        "null(expected 1,2 parameters, actual number of parameters is 0)",
    );
}

#[test]
fn _0012() {
    eq(
        "decision009",
        "null(expected 1,2 parameters, actual number of parameters is 3)",
    );
}

#[test]
fn _0013() {
    eq("decision010_a", r#""ac""#);
}

#[test]
fn _0014() {
    eq("decision010_b", r#""aXc""#);
}

#[test]
fn _0015() {
    eq("decision011_a", "null(parameter 'delimiter' not found)");
}

#[test]
fn _0016() {
    eq("decision011_b", "null(parameter 'list' not found)");
}

#[test]
fn _0017() {
    eq(
        "decision012_a",
        "null(string join: expected list or string, actual value type is Null)",
    );
}

#[test]
fn _0018() {
    eq(
        "decision012_b",
        "null(string join: expected list or string, actual value type is Null)",
    );
}

#[test]
fn _0019() {
    eq(
        "decision013",
        "null([core::string join] invalid argument type, expected string, actual type is number)",
    );
}

#[test]
fn _0020() {
    eq(
        "decision014",
        "null(string join: expected list or string, actual value type is number)",
    );
}

#[test]
fn _0021() {
    eq("decision015", r#""a""#);
}

#[test]
fn _0022() {
    eq("decision016", r#""a""#);
}
