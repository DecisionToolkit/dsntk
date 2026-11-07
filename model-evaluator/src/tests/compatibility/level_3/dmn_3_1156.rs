use super::*;

from_examples!(DMN_3_1156);

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
    eq("decision001", "true");
}

#[test]
fn _0002() {
    eq("decision001_a", "true");
}

#[test]
fn _0003() {
    eq("decision001_b", "true");
}

#[test]
fn _0004() {
    eq("decision001_c", "true");
}

#[test]
fn _0005() {
    eq("decision001_d", "true");
}

#[test]
fn _0006() {
    eq("decision001_e", "true");
}

#[test]
fn _0007() {
    eq("decision001_g", "true");
}

#[test]
fn _0008() {
    eq("decision001_h", "true");
}

#[test]
fn _0009() {
    eq("decision001_i", "true");
}

#[test]
fn _0010() {
    eq("decision002", "true");
}

#[test]
fn _0011() {
    eq("decision003_a", "true");
}

#[test]
fn _0012() {
    eq("decision003_b", "true");
}

#[test]
fn _0013() {
    eq("decision003_c", "true");
}

#[test]
fn _0014() {
    eq("decision003_d", "true");
}

#[test]
fn _0015() {
    eq("decision003_e", "true");
}

#[test]
fn _0016() {
    eq("decision004_e", "null(invalid range literal)");
}

#[test]
fn _0017() {
    eq("decision004_f", "null(invalid range literal)");
}

#[test]
fn _0018() {
    eq("decision005_a", "true");
}

#[test]
fn _0019() {
    eq("decision005_b", "true");
}

#[test]
fn _0020() {
    eq("decision005_c", "true");
}

#[test]
fn _0021() {
    eq("decision005_d", "true");
}

#[test]
fn _0022() {
    eq("decision005_e", "true");
}

#[test]
fn _0023() {
    eq("decision005_f", "true");
}

#[test]
fn _0024() {
    eq("decision006", "true");
}

#[test]
fn _0025() {
    eq("decision007_a", "true");
}

#[test]
fn _0026() {
    eq("decision007_b", "null(invalid range literal)");
}

#[test]
fn _0027() {
    eq("decision007_c", "null(invalid range literal)");
}

#[test]
fn _0028() {
    eq("decision008_a", "true");
}

#[test]
fn _0029() {
    eq("decision008_b", "null(invalid range literal)");
}

#[test]
fn _0030() {
    eq("decision008_c", "null(invalid range literal)");
}

#[test]
fn _0031() {
    eq("decision009_a", "true");
}

#[test]
fn _0032() {
    eq("decision009_b", "null(invalid range literal)");
}

#[test]
fn _0033() {
    eq("decision009_c", "null(invalid range literal)");
}

#[test]
fn _0034() {
    eq("decision010_a", "true");
}

#[test]
fn _0035() {
    eq("decision010_b", "null(invalid range literal)");
}

#[test]
fn _0036() {
    eq("decision010_c", "null(invalid range literal)");
}

#[test]
fn _0037() {
    eq("decision011", "true");
}

#[test]
fn _0038() {
    eq("decision012", "null(parameter 'from' not found)");
}

#[test]
fn _0039() {
    eq(
        "decision013_a",
        "null(expected 1 parameters, actual number of parameters is 2)",
    );
}

#[test]
fn _0040() {
    eq(
        "decision013_b",
        "null(expected 1 parameters, actual number of parameters is 0)",
    );
}

#[test]
fn _0041() {
    eq(
        "decision014",
        "null([core::range] invalid argument type, expected string, actual type is range<number>)",
    );
}

#[test]
fn _0042() {
    eq("decision015_a", "null(invalid range literal)");
}

#[test]
fn _0043() {
    eq("decision015_b", "null(invalid range literal)");
}

#[test]
fn _0044() {
    eq("decision016", "null(invalid range literal)");
}

#[test]
fn _0045() {
    eq("decision017", "null(invalid range literal)");
}

#[test]
fn _0046() {
    eq("decision018", "null(invalid range)");
}

#[test]
fn _0047() {
    eq("decision019_a", "null(invalid range)");
}

#[test]
fn _0048() {
    eq("decision019_b", "null(invalid range)");
}

#[test]
fn _0049() {
    eq("decision020", "null(invalid range)");
}

#[test]
fn _0050() {
    eq("decision021", "null(invalid range)");
}

#[test]
fn _0051() {
    eq("decision022", "null(invalid range)");
}

#[test]
fn _0052() {
    eq("decision023", "null(invalid range)");
}

#[test]
fn _0053() {
    eq("decision024", "null(invalid range)");
}

#[test]
fn _0054() {
    eq("decision025", "null(invalid range)");
}

#[test]
fn _0055() {
    eq("decision023", "null(invalid range)");
}

#[test]
fn _0056() {
    eq("decision027", "null(invalid range literal)");
}
