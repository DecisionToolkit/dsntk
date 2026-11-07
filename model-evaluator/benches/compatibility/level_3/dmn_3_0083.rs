use super::*;

from_examples!(DMN_3_0083);
static_context!(CTX, "{}");

fn bench(b: &mut Bencher, invocable: &str, expected: &str) {
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
    bench(b, "decision_001", "1");
}

#[bench]
fn _0002(b: &mut Bencher) {
    bench(b, "decision_001_a", "1");
}

#[bench]
fn _0003(b: &mut Bencher) {
    bench(b, "decision_001_b", "true");
}

#[bench]
fn _0004(b: &mut Bencher) {
    bench(b, "decision_002", "6");
}

#[bench]
fn _0005(b: &mut Bencher) {
    bench(b, "decision_003", "1");
}

#[bench]
fn _0006(b: &mut Bencher) {
    bench(b, "decision_003_a", "1");
}

#[bench]
fn _0007(b: &mut Bencher) {
    bench(b, "decision_004", "2");
}

#[bench]
fn _0008(b: &mut Bencher) {
    bench(b, "decision_004_a", "2");
}

#[bench]
fn _0009(b: &mut Bencher) {
    bench(b, "decision_005", "true");
}

#[bench]
fn _0010(b: &mut Bencher) {
    bench(b, "decision_005_a", "true");
}

#[bench]
fn _0011(b: &mut Bencher) {
    bench(b, "decision_006", r#"{🐎: "bar"}"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
    bench(b, "decision_007", r#"{🐎: "😀"}"#);
}

#[bench]
fn _0013(b: &mut Bencher) {
    bench(b, "decision_008", "true");
}

#[bench]
fn _0014(b: &mut Bencher) {
    bench(b, "endswith_001", "true");
}

#[bench]
fn _0015(b: &mut Bencher) {
    bench(b, "substring_001", r#""😀""#);
}

#[bench]
fn _0016(b: &mut Bencher) {
    bench(b, "substring_002", r#""🐎""#);
}

#[bench]
fn _0017(b: &mut Bencher) {
    bench(b, "substring_003", r#""🐎bar""#);
}

#[bench]
fn _0018(b: &mut Bencher) {
    bench(b, "substring_004", "true");
}
