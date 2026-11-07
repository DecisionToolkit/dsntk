use super::*;

from_examples!(DMN_3_1140);

static_context!(CTX, "{}");

fn eq(b: &mut Bencher, invocable: &str, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0001(b: &mut Bencher) {
    eq(b, "decision001", r#""abc""#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", r#""a and b and c""#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision003", r#""abc""#);
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision004", r#""abc""#);
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision005_a", r#""a""#);
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "decision005_b", r#""a""#);
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(b, "decision006_a", r#""ac""#);
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(b, "decision006_b", r#""aXc""#);
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "decision007_a", r#""""#);
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(b, "decision007_b", r#""""#);
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(
        b,
        "decision008",
        "null(expected 1,2 parameters, actual number of parameters is 0)",
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(
        b,
        "decision009",
        "null(expected 1,2 parameters, actual number of parameters is 3)",
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(b, "decision010_a", r#""ac""#);
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "decision010_b", r#""aXc""#);
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(b, "decision011_a", "null(parameter 'delimiter' not found)");
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(b, "decision011_b", "null(parameter 'list' not found)");
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(
        b,
        "decision012_a",
        "null(string join: expected list or string, actual value type is Null)",
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    eq(
        b,
        "decision012_b",
        "null(string join: expected list or string, actual value type is Null)",
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    eq(
        b,
        "decision013",
        "null([core::string join] invalid argument type, expected string, actual type is number)",
    );
}

#[bench]
fn _0020(b: &mut Bencher) {
    eq(
        b,
        "decision014",
        "null(string join: expected list or string, actual value type is number)",
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    eq(b, "decision015", r#""a""#);
}

#[bench]
fn _0022(b: &mut Bencher) {
    eq(b, "decision016", r#""a""#);
}
