use super::*;

from_examples!(DMN_3_0092);

static_context!(CTX, r#"{}"#);

fn eq(b: &mut Bencher, invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, ctx)
    );
}

#[bench]
fn _0001(b: &mut Bencher) {
    eq(b, "decision_001_1", &CTX, r#"3"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision_002_1", &CTX, r#"4"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision_003_1", &CTX, r#"5"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision_004_1", &CTX, r#"6"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision_005_1", &CTX, r#"20"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "decision_006_1", &CTX, r#"30"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
    let ctx = context(r#"{input_007_1: 20}"#);
    eq(b, "decision_007_1", &ctx, r#"100"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(b, "decision_008_1", &CTX, r#"6"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "decision_009_1", &CTX, r#"200"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(b, "decision_010_1", &CTX, r#"120"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(b, "decision_010_1_a", &CTX, r#"120"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
    let ctx = context(r#"{input_011_1: 10}"#);
    eq(b, "decision_011_1", &ctx, r#"5000"#);
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(b, "decision_012_1", &CTX, r#"5000"#);
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "decision_013_1", &CTX, r#"5000"#);
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(b, "decision_014_1", &CTX, r#"25"#);
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(b, "decision_015_1", &CTX, r#"18"#);
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(
        b,
        "decision_016_1",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 2)"#,
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    let ctx = context(r#"{input_017_1: "a"}"#);
    eq(b, "decision_017_1", &ctx, r#"["a", "a", "z", "z"]"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
    eq(b, "decision_018", &CTX, r#"["a", "a", "z", "z"]"#);
}
