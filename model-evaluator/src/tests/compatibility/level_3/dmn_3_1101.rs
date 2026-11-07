use super::*;

from_examples!(DMN_3_1101);

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
    eq("decision001", &CTX, "1");
}

#[test]
fn _0002() {
    eq("decision002", &CTX, "-2");
}

#[test]
fn _0003() {
    eq("decision003", &CTX, "0");
}

#[test]
fn _0004() {
    eq("decision004", &CTX, "1.5");
}

#[test]
fn _0005() {
    eq("decision005", &CTX, "-1.6");
}

#[test]
fn _0006() {
    eq(
        "decision006",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[test]
fn _0007() {
    eq(
        "decision007",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[test]
fn _0008() {
    eq(
        "decision008",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[test]
fn _0009() {
    eq(
        "decision009",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[test]
fn _0010() {
    eq(
        "decision010",
        &CTX,
        "null(expected 1,2 parameters, actual number of parameters is 0)",
    );
}

#[test]
fn _0011() {
    eq(
        "decision011",
        &CTX,
        "null(expected 1,2 parameters, actual number of parameters is 3)",
    );
}

#[test]
fn _0012() {
    eq(
        "decision012",
        &CTX,
        "null(expected 1,2 parameters, actual number of parameters is 3)",
    );
}

#[test]
fn _0013() {
    eq("decision013", &CTX, "null(parameter 'scale' not found)");
}

#[test]
fn _0014() {
    eq(
        "decision014",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}

#[test]
fn _0015() {
    eq(
        "decision015",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}

#[test]
fn _0016() {
    eq(
        "decision016",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}

#[test]
fn _0017() {
    eq(
        "decision017",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}
