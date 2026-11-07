use super::*;

from_examples!(DMN_3_1146);
static_context!(CTX, "{}");

#[test]
fn _0001() {
    let invocable = "decision001";
    let expected = "{a: 1}";
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
fn _0002() {
    let invocable = "decision002";
    let expected = r#"[{key: "a", value: 1}, {key: "b", value: 2}]"#;
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
fn _0003() {
    let invocable = "decision003";
    let expected = "{a: 2}";
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
fn _0004() {
    let invocable = "decision003_a";
    let expected = "{a: 1, b: 3, c: 3}";
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
fn _0005() {
    let invocable = "decision004";
    let expected = r#"{"": 1}"#;
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
fn _0006() {
    let invocable = "decision005";
    let expected = r#"null"#;
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
fn _0007() {
    let invocable = "decision006";
    let expected = r#"null"#;
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
fn _0008() {
    let invocable = "decision007";
    let expected = "{a: null}";
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
fn _0009() {
    let invocable = "decision008";
    let expected = "null(expected 3 parameters, actual number of parameters is 2)";
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
fn _0010() {
    let invocable = "decision009";
    let expected = "null(expected 3 parameters, actual number of parameters is 4)";
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
fn _0011() {
    let invocable = "decision010";
    let expected = "{a: 1}";
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
fn _0012() {
    let invocable = "decision011";
    let expected = "null(parameter 'key' or 'keys' not found)";
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
fn _0013() {
    let invocable = "decision012";
    let expected = "null([core::context put] invalid argument type, expected context, actual type is list<Null>)";
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
fn _0014() {
    let invocable = "decision013";
    let expected = "null([core::context put] invalid argument type, expected string or list<string>, actual type is number)";
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
fn _0015() {
    let invocable = "decision014";
    let expected = "{a: 2}";
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
fn _0016() {
    let invocable = "decision015";
    let expected = "{context01: {a: 1}, copied: {a: 2}}";
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
fn _0017() {
    let invocable = "decision016";
    let expected = "{copied: {a: 2}, original: {a: 1}}";
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
fn _0018() {
    let invocable = "nested001";
    let expected = "{x: 1, y: {a: 2}}";
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
fn _0019() {
    let invocable = "nested001_a";
    let expected = "{x: 1, y: {a: 0, b: 2}}";
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
fn _0020() {
    let invocable = "nested002";
    let expected = "null";
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
fn _0021() {
    let invocable = "nested003";
    let expected = "null";
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
fn _0023() {
    let invocable = "nested004";
    let expected = "null";
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
fn _0024() {
    let invocable = "nested005";
    let expected = "null";
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
fn _0025() {
    let invocable = "nested006";
    let expected = "{x: 1, y: {a: {b: {c: 2}}}}";
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
fn _0026() {
    let invocable = "nested007";
    let expected = "{x: 1, y: {a: 2}}";
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
fn _0027() {
    let invocable = "nested008";
    let expected = "null";
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
fn _0028() {
    let invocable = "nested009";
    let expected = "null";
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
fn _0029() {
    let invocable = "nested010";
    let expected = "{a: {b: 2}}";
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
fn _0030() {
    let invocable = "nested011";
    let expected = "{copied: {a: {b: 2}}, nestedContext01: {a: {b: 1}}}";
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
fn _0031() {
    let invocable = "nested012";
    let expected = "{copied: {a: {b: 2}}, original: {a: {b: 1}}}";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
}
