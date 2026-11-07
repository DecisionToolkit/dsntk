use super::*;

from_examples!(DMN_N_0079);

static_context!(CTX, r#"{}"#);

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "decision_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 0)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "decision_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 2)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "decision_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""foo""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable_name = "decision_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 2)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable_name = "null_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable_name = "string_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""foo""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""123.45""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "boolean_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""true""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let invocable_name = "boolean_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""false""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let invocable_name = "date_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2018-12-10""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    let invocable_name = "dateTime_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2018-12-10T00:00:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    let invocable_name = "dateTime_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2018-12-10T10:30:00.0001""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    let invocable_name = "dateTime_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2018-12-10T10:30:00.0001+05:00:01""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    let invocable_name = "dateTime_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2018-12-10T10:30:00@Etc/UTC""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0015(b: &mut Bencher) {
    let invocable_name = "time_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:30:00.0001""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    let invocable_name = "time_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:30:00.0001+05:00:01""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    let invocable_name = "time_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:30:00@Etc/UTC""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    let invocable_name = "dt_duration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""P1D""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    let invocable_name = "dt_duration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-P1D""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0020(b: &mut Bencher) {
    let invocable_name = "dt_duration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""PT0S""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    let invocable_name = "dt_duration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""P1DT2H3M4.1234S""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0022(b: &mut Bencher) {
    let invocable_name = "dt_duration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""P2DT1H""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0023(b: &mut Bencher) {
    let invocable_name = "ym_duration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""P1Y""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0024(b: &mut Bencher) {
    let invocable_name = "ym_duration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-P1Y""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0025(b: &mut Bencher) {
    let invocable_name = "ym_duration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""P0M""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0026(b: &mut Bencher) {
    let invocable_name = "ym_duration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""P1Y2M""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0027(b: &mut Bencher) {
    let invocable_name = "ym_duration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""P2Y1M""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0028(b: &mut Bencher) {
    let invocable_name = "list_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""[1, 2, 3, "foo"]""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0029(b: &mut Bencher) {
    let invocable_name = "list_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""[1, 2, 3, [4, 5, "foo"]]""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0030(b: &mut Bencher) {
    let invocable_name = "list_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""["\"foo\""]""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0031(b: &mut Bencher) {
    let invocable_name = "context_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""{a: "foo"}""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0032(b: &mut Bencher) {
    let invocable_name = "context_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""{a: "foo", b: {bar: "baz"}}""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0033(b: &mut Bencher) {
    let invocable_name = "context_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""{"{": "foo"}""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0034(b: &mut Bencher) {
    let invocable_name = "context_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""{":": "foo"}""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0035(b: &mut Bencher) {
    let invocable_name = "context_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""{",": "foo"}""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0036(b: &mut Bencher) {
    let invocable_name = "context_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""{"}": "foo"}""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0037(b: &mut Bencher) {
    let invocable_name = "context_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""{"\"": "foo"}""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}
