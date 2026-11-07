use super::*;

from_examples!(DMN_3_1155);

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
    eq(b, "decision001", "[1, 4, 3]");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", "[1, 2, 4]");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision003", "null");
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision004", "null");
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision005", "null");
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(
        b,
        "decision006",
        "null([core::list replace] invalid argument type, expected list, actual type is Null)",
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(b, "decision007", "null");
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(b, "decision008", "[1, 2, null]");
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "decision009", "[5, 5, 7, 8]");
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(b, "decision010", "null");
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(b, "decision011", "[1, 4, 3]");
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(b, "decision011_a", "[1, 2, 4]");
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(b, "decision012", "[1, 4, 3]");
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "decision013", "[1, 4, 3]");
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(
        b,
        "decision014",
        "null(expected 3 parameters, actual number of parameters is 4)",
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(
        b,
        "decision015",
        "null(expected 3 parameters, actual number of parameters is 2)",
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(
        b,
        "decision016",
        "null(expected 3 parameters, actual number of parameters is 4)",
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    eq(
        b,
        "decision017",
        "null(list replace: matching function must accept exactly two arguments)",
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    eq(
        b,
        "decision018",
        "null(list replace: matching function must accept exactly two arguments)",
    );
}

#[bench]
fn _0020(b: &mut Bencher) {
    eq(
        b,
        "decision019",
        "null(list replace: matching function must return boolean value)",
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    eq(b, "decision020", "[5, 5, 5, 5]");
}

#[bench]
fn _0022(b: &mut Bencher) {
    eq(b, "decision021", "[5]");
}
