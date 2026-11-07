use super::*;

from_examples!(DMN_3_1156);

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
    eq(b, "decision001", "true");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision001_a", "true");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision001_b", "true");
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision001_c", "true");
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision001_d", "true");
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "decision001_e", "true");
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(b, "decision001_g", "true");
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(b, "decision001_h", "true");
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "decision001_i", "true");
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(b, "decision002", "true");
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(b, "decision003_a", "true");
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(b, "decision003_b", "true");
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(b, "decision003_c", "true");
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "decision003_d", "true");
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(b, "decision003_e", "true");
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(b, "decision004_e", "null(invalid range literal)");
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(b, "decision004_f", "null(invalid range literal)");
}

#[bench]
fn _0018(b: &mut Bencher) {
    eq(b, "decision005_a", "true");
}

#[bench]
fn _0019(b: &mut Bencher) {
    eq(b, "decision005_b", "true");
}

#[bench]
fn _0020(b: &mut Bencher) {
    eq(b, "decision005_c", "true");
}

#[bench]
fn _0021(b: &mut Bencher) {
    eq(b, "decision005_d", "true");
}

#[bench]
fn _0022(b: &mut Bencher) {
    eq(b, "decision005_e", "true");
}

#[bench]
fn _0023(b: &mut Bencher) {
    eq(b, "decision005_f", "true");
}

#[bench]
fn _0024(b: &mut Bencher) {
    eq(b, "decision006", "true");
}

#[bench]
fn _0025(b: &mut Bencher) {
    eq(b, "decision007_a", "true");
}

#[bench]
fn _0026(b: &mut Bencher) {
    eq(b, "decision007_b", "null(invalid range literal)");
}

#[bench]
fn _0027(b: &mut Bencher) {
    eq(b, "decision007_c", "null(invalid range literal)");
}

#[bench]
fn _0028(b: &mut Bencher) {
    eq(b, "decision008_a", "true");
}

#[bench]
fn _0029(b: &mut Bencher) {
    eq(b, "decision008_b", "null(invalid range literal)");
}

#[bench]
fn _0030(b: &mut Bencher) {
    eq(b, "decision008_c", "null(invalid range literal)");
}

#[bench]
fn _0031(b: &mut Bencher) {
    eq(b, "decision009_a", "true");
}

#[bench]
fn _0032(b: &mut Bencher) {
    eq(b, "decision009_b", "null(invalid range literal)");
}

#[bench]
fn _0033(b: &mut Bencher) {
    eq(b, "decision009_c", "null(invalid range literal)");
}

#[bench]
fn _0034(b: &mut Bencher) {
    eq(b, "decision010_a", "true");
}

#[bench]
fn _0035(b: &mut Bencher) {
    eq(b, "decision010_b", "null(invalid range literal)");
}

#[bench]
fn _0036(b: &mut Bencher) {
    eq(b, "decision010_c", "null(invalid range literal)");
}

#[bench]
fn _0037(b: &mut Bencher) {
    eq(b, "decision011", "true");
}

#[bench]
fn _0038(b: &mut Bencher) {
    eq(b, "decision012", "null(parameter 'from' not found)");
}

#[bench]
fn _0039(b: &mut Bencher) {
    eq(
        b,
        "decision013_a",
        "null(expected 1 parameters, actual number of parameters is 2)",
    );
}

#[bench]
fn _0040(b: &mut Bencher) {
    eq(
        b,
        "decision013_b",
        "null(expected 1 parameters, actual number of parameters is 0)",
    );
}

#[bench]
fn _0041(b: &mut Bencher) {
    eq(
        b,
        "decision014",
        "null([core::range] invalid argument type, expected string, actual type is range<number>)",
    );
}

#[bench]
fn _0042(b: &mut Bencher) {
    eq(b, "decision015_a", "null(invalid range literal)");
}

#[bench]
fn _0043(b: &mut Bencher) {
    eq(b, "decision015_b", "null(invalid range literal)");
}

#[bench]
fn _0044(b: &mut Bencher) {
    eq(b, "decision016", "null(invalid range literal)");
}

#[bench]
fn _0045(b: &mut Bencher) {
    eq(b, "decision017", "null(invalid range literal)");
}

#[bench]
fn _0046(b: &mut Bencher) {
    eq(b, "decision018", "null(invalid range)");
}

#[bench]
fn _0047(b: &mut Bencher) {
    eq(b, "decision019_a", "null(invalid range)");
}

#[bench]
fn _0048(b: &mut Bencher) {
    eq(b, "decision019_b", "null(invalid range)");
}

#[bench]
fn _0049(b: &mut Bencher) {
    eq(b, "decision020", "null(invalid range)");
}

#[bench]
fn _0050(b: &mut Bencher) {
    eq(b, "decision021", "null(invalid range)");
}

#[bench]
fn _0051(b: &mut Bencher) {
    eq(b, "decision022", "null(invalid range)");
}

#[bench]
fn _0052(b: &mut Bencher) {
    eq(b, "decision023", "null(invalid range)");
}

#[bench]
fn _0053(b: &mut Bencher) {
    eq(b, "decision024", "null(invalid range)");
}

#[bench]
fn _0054(b: &mut Bencher) {
    eq(b, "decision025", "null(invalid range)");
}

#[bench]
fn _0055(b: &mut Bencher) {
    eq(b, "decision023", "null(invalid range)");
}

#[bench]
fn _0056(b: &mut Bencher) {
    eq(b, "decision027", "null(invalid range literal)");
}
