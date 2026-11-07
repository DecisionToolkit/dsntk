use super::*;

from_examples!(DMN_3_1147);

static_context!(CTX, "{}");

fn eq(invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
}

#[test]
fn _0001() {
    eq("decision001", &CTX, "{a: 1}");
}

#[test]
fn _0002() {
    eq("decision001_a", &CTX, "true");
}

#[test]
fn _0003() {
    eq("decision002", &CTX, "{a: 1, b: 2}");
}

#[test]
fn _0004() {
    eq("decision003", &CTX, "{a: 2}");
}

#[test]
fn _0005() {
    eq("decision004", &CTX, "{a: {bb: 2}}");
}

#[test]
fn _0006() {
    eq("decision005", &CTX, "null");
}

#[test]
fn _0007() {
    eq(
        "decision006",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 0)",
    );
}

#[test]
fn _0008() {
    eq(
        "decision007",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 2)",
    );
}

#[test]
fn _0009() {
    eq("decision008", &CTX, "{a: 1}");
}

#[test]
fn _0010() {
    eq("decision009", &CTX, "null(parameter 'contexts' not found)");
}

#[test]
fn _0011() {
    eq(
        "decision010",
        &CTX,
        "null([core::context merge] invalid argument type, expected context, actual type is number)",
    );
}

#[test]
fn _0012() {
    eq(
        "decision011",
        &CTX,
        "null([core::context merge] invalid argument type, expected context, actual type is number)",
    );
}

#[test]
fn _0013() {
    eq("decision012", &CTX, "{a: 1}");
}

#[test]
fn _0014() {
    eq("decision013", &CTX, "{a: 1}");
}
