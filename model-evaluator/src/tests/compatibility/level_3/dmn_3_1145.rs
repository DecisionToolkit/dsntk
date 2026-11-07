use super::*;

from_examples!(DMN_3_1145);

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
    eq("decision001", &CTX, "{a: 1, b: 2}");
}

#[test]
fn _0002() {
    eq("decision002", &CTX, "{a: 1}");
}

#[test]
fn _0003() {
    eq("decision003", &CTX, "null(context: duplicated key: a)");
}

#[test]
fn _0004() {
    eq("decision004", &CTX, "true");
}

#[test]
fn _0005() {
    eq("decision005", &CTX, "{a: 1}");
}

#[test]
fn _0006() {
    eq(
        "decision006",
        &CTX,
        "null(context: no 'key' entry in context {value: 1})",
    );
}

#[test]
fn _0007() {
    eq(
        "decision007",
        &CTX,
        "null(context: 'key' entry is not a string, actual type is Null)",
    );
}

#[test]
fn _0008() {
    eq(
        "decision008",
        &CTX,
        r#"null(context: no 'value' entry in context {key: "a"})"#,
    );
}

#[test]
fn _0009() {
    eq("decision009", &CTX, "{a: null}");
}

#[test]
fn _0010() {
    eq("decision010", &CTX, r#"{"": 1}"#);
}

#[test]
fn _0011() {
    eq("decision011", &CTX, "null");
}

#[test]
fn _0012() {
    eq(
        "decision012",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 0)",
    );
}

#[test]
fn _0013() {
    eq(
        "decision013",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 2)",
    );
}

#[test]
fn _0014() {
    eq("decision014_a", &CTX, "{a: 1}");
}

#[test]
fn _0015() {
    eq("decision014_b", &CTX, "{a: 1}");
}

#[test]
fn _0016() {
    eq("decision015", &CTX, "null(parameter 'entries' not found)");
}

#[test]
fn _0017() {
    eq(
        "decision016",
        &CTX,
        "null([core::context] invalid argument type, expected list or context, actual type is string)",
    );
}

#[test]
fn _0018() {
    eq("decision017", &CTX, "{a: 1}");
}
