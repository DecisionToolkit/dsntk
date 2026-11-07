use super::*;

from_examples!(DMN_3_0085);

const CTX: &str = "{}";

fn eqs(b: &mut Bencher, invocable: &str, input: &str, expected: &str) {
    let ctx = context(input);
    assert_decision_service(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &ctx)
    );
}

fn eqd(b: &mut Bencher, invocable: &str, input: &str, expected: &str) {
    let ctx = context(input);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &ctx,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &ctx)
    );
}

#[bench]
fn _0001(b: &mut Bencher) {
    eqs(b, "decision_001", CTX, r#""foo""#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eqs(b, "decisionService_001", CTX, r#""foo""#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eqs(
        b,
        "decision_002",
        r#"{decision_002_input: "baz"}"#,
        r#""foo baz""#,
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    eqs(
        b,
        "decisionService_002",
        r#"{decision_002_input: "baz"}"#,
        r#""foo baz""#,
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    eqs(b, "decision_002", CTX, r#""foo bar""#);
}

#[bench]
fn _0006(b: &mut Bencher) {
    eqs(
        b,
        "decisionService_002",
        CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    eqs(
        b,
        "decision_002",
        r#"{decision_002_input: null}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    eqs(
        b,
        "decisionService_002",
        r#"{decision_002_input: null}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    eqs(
        b,
        "decision_002",
        r#"{decision_002_input: 1234}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    eqs(
        b,
        "decisionService_002",
        r#"{decision_002_input: 1234}"#,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    eqs(
        b,
        "decision_003",
        r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
        r#""A B C D""#,
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    eqs(
        b,
        "decisionService_003",
        r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
        r#""A B C D""#,
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eqd(b, "decision_004_1", CTX, r#""foo""#);
}

#[bench]
fn _0014(b: &mut Bencher) {
    eqs(b, "decisionService_005", CTX, r#""foo""#);
}

#[bench]
fn _0015(b: &mut Bencher) {
    eqs(
        b,
        "decision_005_1",
        CTX,
        r#"null(invalid number of arguments)"#,
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    eqs(b, "decision_005_2", CTX, r#""foo""#);
}

#[bench]
fn _0017(b: &mut Bencher) {
    eqs(b, "decision_006_1", CTX, r#""foo bar""#);
}

#[bench]
fn _0018(b: &mut Bencher) {
    eqs(
        b,
        "decision_007_1",
        CTX,
        r#"null(equal err 'null(after coercion)' =?= 'null')"#,
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    eqs(b, "decisionService_007", CTX, r#"true"#);
}

#[bench]
fn _0020(b: &mut Bencher) {
    eqs(
        b,
        "decision_008_1",
        CTX,
        r#"null(invalid number of arguments)"#,
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    eqs(b, "decision_009_1", CTX, r#""foo bar""#);
}

#[bench]
fn _0022(b: &mut Bencher) {
    eqs(
        b,
        "decision_010_1",
        CTX,
        r#"null(parameter with name decision_010_3 not found in arguments)"#,
    );
}

#[bench]
fn _0023(b: &mut Bencher) {
    eqs(b, "decision_011_1", CTX, r#""A B C D""#);
}

#[bench]
fn _0024(b: &mut Bencher) {
    eqs(b, "decision_012_1", CTX, r#""A B C D""#);
}

#[bench]
fn _0025(b: &mut Bencher) {
    eqs(
        b,
        "decision_013_1",
        CTX,
        r#"{decisionService_013: "A B", decision_013_3: "D", inputData_013_1: null}"#,
    );
}

#[bench]
fn _0026(b: &mut Bencher) {
    eqs(
        b,
        "decision_014_1",
        CTX,
        r#"{decisionService_014: "A B", decision_014_3: "D", inputData_014_1: null}"#,
    );
}
