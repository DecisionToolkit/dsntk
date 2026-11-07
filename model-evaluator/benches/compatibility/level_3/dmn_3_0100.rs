use super::*;

from_examples!(DMN_3_0100);
static CTX: LazyLock<FeelContext> = LazyLock::new(|| context(r#"{}"#));

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"100"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-100"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"100"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_number_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"0"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P10D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT2H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT0S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P6DT1H30M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_dtDuration_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P2DT9H30M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P10D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0015(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT2H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT0S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P6DT1H30M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0020(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_dtDuration_by_rhs_number_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P2DT9H30M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0022(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P10Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0023(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0024(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0025(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P2Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0026(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P2Y2M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0027(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P0M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0028(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P3M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0029(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P3M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0030(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_number_by_rhs_ymDuration_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P4Y9M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0031(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0032(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P10Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0033(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0034(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0035(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P2Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0036(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P2Y2M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0037(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P0M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0038(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P3M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0039(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P3M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0040(b: &mut Bencher) {
    let invocable_name = "multiply_lhs_ymDuration_by_rhs_number_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P4Y9M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0041(b: &mut Bencher) {
    let invocable_name = "divide_lhs_number_by_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0042(b: &mut Bencher) {
    let invocable_name = "divide_lhs_number_by_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] division by zero)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0043(b: &mut Bencher) {
    let invocable_name = "divide_lhs_ymDuration_by_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0044(b: &mut Bencher) {
    let invocable_name = "divide_lhs_ymDuration_by_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] division by zero)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0045(b: &mut Bencher) {
    let invocable_name = "divide_lhs_ymDuration_by_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P4Y4M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0046(b: &mut Bencher) {
    let invocable_name = "divide_lhs_ymDuration_by_rhs_number_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P4Y4M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0047(b: &mut Bencher) {
    let invocable_name = "divide_lhs_ymDuration_by_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0048(b: &mut Bencher) {
    let invocable_name = "divide_lhs_ymDuration_by_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P10D / P0M)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0049(b: &mut Bencher) {
    let invocable_name = "divide_lhs_dtDuration_by_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0050(b: &mut Bencher) {
    let invocable_name = "divide_lhs_dtDuration_by_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] division by zero)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0051(b: &mut Bencher) {
    let invocable_name = "divide_lhs_dtDuration_by_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P4DT9H12M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0052(b: &mut Bencher) {
    let invocable_name = "divide_lhs_dtDuration_by_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0053(b: &mut Bencher) {
    let invocable_name = "divide_lhs_dtDuration_by_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] division by zero)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0054(b: &mut Bencher) {
    let invocable_name = "add_lhs_number_to_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"15"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0055(b: &mut Bencher) {
    let invocable_name = "add_lhs_number_to_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"5"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0056(b: &mut Bencher) {
    let invocable_name = "add_lhs_number_to_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-5"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0057(b: &mut Bencher) {
    let invocable_name = "add_lhs_number_to_rhs_number_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-15"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0058(b: &mut Bencher) {
    let invocable_name = "add_lhs_number_to_rhs_number_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"3.4685"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0059(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2022-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0060(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-02-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0061(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0062(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0063(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0064(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0065(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-02-01T10:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0066(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-01T10:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0067(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2021-02-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0068(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-02-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0069(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2020-01-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0070(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0071(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-01-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0072(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-02-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0073(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2020-01-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0074(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0075(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_ymDuration_017";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-01-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0076(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2022-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0077(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-02-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0078(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0079(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0080(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0081(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0082(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-02-01T10:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0083(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-01T10:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0084(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2021-02-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0085(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-02-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0086(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2020-01-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0087(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0088(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-01-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0089(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-02-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0090(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2020-01-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0091(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0092(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_dateAndTime_017";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-01-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0093(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-13T11:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0094(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-13T11:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0095(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0096(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T11:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0097(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-31T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0098(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T09:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0099(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0100(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0101(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0102(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0103(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T11:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0104(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T09:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0105(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2021-01-13T11:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0106(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T11:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0107(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-02T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0108(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T09:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0109(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_017";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-31T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0110(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_018";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T11:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0111(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_019";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-02T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0112(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_020";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T09:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0113(b: &mut Bencher) {
    let invocable_name = "add_lhs_dateAndTime_to_rhs_dtDuration_021";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-31T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0114(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-13T11:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0115(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-13T11:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0116(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0117(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T11:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0118(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-31T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0119(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T09:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0120(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0121(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0122(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0123(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0124(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T11:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0125(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T09:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0126(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2021-01-13T11:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0127(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T11:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0128(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-02T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0129(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T09:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0130(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_017";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-31T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0131(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_018";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T11:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0132(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_019";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-02T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0133(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_020";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T09:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0134(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dateAndTime_021";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-31T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0135(b: &mut Bencher) {
    let invocable_name = "add_lhs_date_to_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2022-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0136(b: &mut Bencher) {
    let invocable_name = "add_lhs_date_to_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-02-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0137(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_date_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2022-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0138(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_date_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-02-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0139(b: &mut Bencher) {
    let invocable_name = "add_lhs_date_to_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0140(b: &mut Bencher) {
    let invocable_name = "add_lhs_date_to_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0141(b: &mut Bencher) {
    let invocable_name = "add_lhs_date_to_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-03"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0142(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_date_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0143(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_date_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0144(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_date_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-03"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0145(b: &mut Bencher) {
    let invocable_name = "add_lhs_time_to_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"11:15:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0146(b: &mut Bencher) {
    let invocable_name = "add_lhs_time_to_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"10:15:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0147(b: &mut Bencher) {
    let invocable_name = "add_lhs_time_to_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"10:15:00+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0148(b: &mut Bencher) {
    let invocable_name = "add_lhs_time_to_rhs_dtDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0149(b: &mut Bencher) {
    let invocable_name = "add_lhs_time_to_rhs_dtDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0150(b: &mut Bencher) {
    let invocable_name = "add_lhs_time_to_rhs_dtDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""11:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0151(b: &mut Bencher) {
    let invocable_name = "add_lhs_time_to_rhs_dtDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""09:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0152(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_time_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"11:15:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0153(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_time_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"10:15:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0154(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_time_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"10:15:00+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0155(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_time_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0156(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_time_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0157(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_time_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""11:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0158(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_time_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""09:15:00@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0159(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y2M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0160(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0161(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_ymDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P10M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0162(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_ymDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P1Y2M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0163(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_ymDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0164(b: &mut Bencher) {
    let invocable_name = "add_lhs_ymDuration_to_rhs_ymDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0165(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P3D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0166(b: &mut Bencher) {
    let invocable_name = "add_lhs_dtDuration_to_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P2D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0167(b: &mut Bencher) {
    let invocable_name = "add_lhs_string_to_rhs_string_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""foobar""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0168(b: &mut Bencher) {
    let invocable_name = "add_lhs_string_to_rhs_string_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""11""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0169(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_number_minus_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"5"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0170(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_number_minus_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"15"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0171(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_number_minus_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-15"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0172(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_number_minus_rhs_number_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-5"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0173(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_number_minus_rhs_number_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"0.8889"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0174(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_number_minus_rhs_number_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"1.3579"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0175(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_number_minus_rhs_number_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-0.8889"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0176(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0177(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10@Europe/Paris - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0178(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10 - 2021-01-01T10:10:10@Europe/Paris)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0179(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10+02:00 - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0180(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10 - 2021-01-01T10:10:10+02:00)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0181(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0182(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT5H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0183(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0184(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10+01:00 - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0185(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT0S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0186(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P9498D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0187(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P9498DT5H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0188(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0189(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0190(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2022-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0191(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-02-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0192(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0193(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0194(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-01-01T10:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0195(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2020-01-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0196(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0197(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-01-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0198(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-02-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0199(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2020-01-01T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0200(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0201(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-01-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0202(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-02-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0203(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_ymDuration_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2020-01-01T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0204(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_date_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T00:00:00 - 2021-01-02)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0205(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_date_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT9H10M10S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0206(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_date_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT9H10M10S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0207(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_date_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P9498DT9H10M10S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0208(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_date_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P9498DT9H10M10S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0209(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-31T10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0210(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T09:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0211(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T23:00:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0212(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-31T10:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0213(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0214(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0215(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""2020-12-31T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0216(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0217(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0218(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T09:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0219(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-31T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0220(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T11:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0221(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_013";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-02T10:10:10+11:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0222(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_014";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T09:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0223(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_015";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2022-12-31T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0224(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_016";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-01T11:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0225(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dateAndTime_minus_rhs_dtDuration_017";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""-2021-01-02T10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0226(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dateAndTime_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02 - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0227(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dateAndTime_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT49M50S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0228(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dateAndTime_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT49M50S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0229(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dateAndTime_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT0S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0230(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dateAndTime_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P9498DT49M50S"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0231(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_date_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0232(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_date_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P9498D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0233(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_date_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P9498D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0234(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0235(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0236(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_ymDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0237(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_ymDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2022-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0238(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0239(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0240(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0241(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0242(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0243(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0244(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-02"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0245(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_008";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0246(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_009";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-12-31"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0247(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_010";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-03"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0248(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2021-01-03"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0249(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_date_minus_rhs_dtDuration_012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"2020-02-29"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0250(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_time_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT1H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0251(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_time_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-PT1H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0252(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_time_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-PT1H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0253(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_time_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT1H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0254(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_time_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-PT1H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0255(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_time_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT1H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0256(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"09:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0257(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"10:10:10"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0258(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"10:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0259(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_dtDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""10:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0260(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_dtDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"09:10:10+11:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0261(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_time_minus_rhs_dtDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#""09:10:10@Australia/Melbourne""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0262(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_ymDuration_minus_rhs_ymDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P10M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0263(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_ymDuration_minus_rhs_ymDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P1Y2M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0264(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_ymDuration_minus_rhs_ymDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P10M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0265(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_ymDuration_minus_rhs_ymDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y2M"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0266(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_ymDuration_minus_rhs_ymDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0267(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_ymDuration_minus_rhs_ymDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1Y"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0268(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dtDuration_minus_rhs_dtDuration_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"PT22H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0269(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dtDuration_minus_rhs_dtDuration_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-P1DT2H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0270(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dtDuration_minus_rhs_dtDuration_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"-PT22H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0271(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dtDuration_minus_rhs_dtDuration_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1DT2H"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0272(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dtDuration_minus_rhs_dtDuration_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0273(b: &mut Bencher) {
    let invocable_name = "subtract_lhs_dtDuration_minus_rhs_dtDuration_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"P1D"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0274(b: &mut Bencher) {
    let invocable_name = "exponent_lhs_number_exp_rhs_number_001";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"25"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0275(b: &mut Bencher) {
    let invocable_name = "exponent_lhs_number_exp_rhs_number_002";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"0.04"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0276(b: &mut Bencher) {
    let invocable_name = "exponent_lhs_number_exp_rhs_number_003";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"3486784401"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0277(b: &mut Bencher) {
    let invocable_name = "exponent_lhs_number_exp_rhs_number_004";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"25"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0278(b: &mut Bencher) {
    let invocable_name = "exponent_lhs_number_exp_rhs_number_005";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"60.58617166606633673745724928244261"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0279(b: &mut Bencher) {
    let invocable_name = "exponent_lhs_number_exp_rhs_number_006";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"25"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0280(b: &mut Bencher) {
    let invocable_name = "exponent_lhs_number_exp_rhs_number_007";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"30.25"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0281(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0282(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0283(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0284(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0285(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0286(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0287(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0288(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0289(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0290(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_number_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: 10 * null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0291(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0292(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0293(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0294(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0295(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0296(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0297(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0298(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0299(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0300(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0301(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0302(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0303(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_string_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0304(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0305(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0306(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0307(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0308(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0309(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0310(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0311(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0312(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0313(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0314(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0315(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0316(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_boolean_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0317(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0318(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0319(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0320(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0321(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0322(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0323(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0324(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0325(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0326(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0327(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0328(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0329(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_date_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0330(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0331(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0332(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0333(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0334(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0335(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0336(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0337(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0338(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0339(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0340(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0341(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0342(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dateAndTime_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0343(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0344(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0345(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0346(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0347(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0348(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0349(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0350(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0351(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0352(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0353(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0354(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0355(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_time_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0356(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0357(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0358(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0359(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0360(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0361(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0362(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0363(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0364(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0365(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0366(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0367(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0368(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_list_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0369(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0370(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0371(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0372(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0373(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0374(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0375(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0376(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0377(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0378(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0379(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0380(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0381(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_context_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0382(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0383(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0384(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0385(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0386(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0387(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0388(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0389(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0390(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0391(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0392(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0393(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_dtDuration_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1D * null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0394(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0395(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0396(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0397(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0398(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0399(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0400(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0401(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0402(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0403(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0404(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0405(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_ymDuration_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0406(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0407(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0408(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0409(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0410(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0411(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0412(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0413(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0414(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0415(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0416(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0417(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0418(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_range_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0419(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0420(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0421(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0422(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0423(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0424(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0425(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0426(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0427(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0428(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0429(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0430(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0431(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_function_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0432(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_number";
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
fn _0433(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_string";
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
fn _0434(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_boolean";
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
fn _0435(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_date";
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
fn _0436(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_dateAndTime";
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
fn _0437(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_time";
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
fn _0438(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_list";
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
fn _0439(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_context";
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
fn _0440(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_dtDuration";
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
fn _0441(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_ymDuration";
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
fn _0442(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_range";
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
fn _0443(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_function";
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
fn _0444(b: &mut Bencher) {
    let invocable_name = "error_when_multiply_lhs_null_by_rhs_null";
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
fn _0445(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0446(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0447(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0448(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0449(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0450(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0451(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0452(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0453(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0454(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0455(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0456(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_number_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10 / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0457(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0458(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0459(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0460(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0461(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0462(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0463(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0464(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0465(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0466(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0467(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0468(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0469(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_string_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: "10" / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0470(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0471(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0472(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0473(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0474(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0475(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0476(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0477(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0478(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0479(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0480(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0481(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0482(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_boolean_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: true / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0483(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0484(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0485(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0486(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0487(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0488(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0489(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0490(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0491(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0492(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0493(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0494(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0495(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_date_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0496(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0497(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0498(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0499(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0500(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0501(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0502(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0503(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0504(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0505(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0506(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0507(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0508(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dateAndTime_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0509(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0510(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0511(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0512(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0513(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0514(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0515(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0516(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0517(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0518(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0519(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0520(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0521(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_time_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0522(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0523(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0524(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0525(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0526(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0527(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0528(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0529(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0530(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0531(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0532(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0533(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0534(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_list_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [10] / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0535(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0536(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0537(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0538(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0539(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0540(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0541(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0542(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0543(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0544(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0545(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0546(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0547(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_context_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: {a: 10} / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0548(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0549(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0550(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0551(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0552(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0553(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0554(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0555(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0556(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0557(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0558(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_dtDuration_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1D / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0559(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0560(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0561(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0562(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0563(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0564(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0565(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0566(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0567(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0568(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0569(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_ymDuration_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: P1Y / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0570(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0571(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0572(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0573(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0574(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0575(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0576(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0577(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0578(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0579(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0580(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0581(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0582(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_range_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: [1..10] / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0583(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0584(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0585(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0586(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0587(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0588(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0589(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0590(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0591(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0592(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0593(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0594(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0595(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_function_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0596(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0597(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0598(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0599(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0600(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0601(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0602(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0603(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0604(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0605(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0606(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0607(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0608(b: &mut Bencher) {
    let invocable_name = "error_when_divide_lhs_null_by_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([division] incompatible types: null / null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0609(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + "10"(string))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0610(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + true(boolean))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0611(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + 2021-01-01(date))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0612(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + 2021-01-01T10:10:10(date and time))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0613(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + 10:10:10(time))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0614(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + [10](list<number>))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0615(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + {a: 10}(context<a: number>))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0616(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + P1D(days and time duration))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0617(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + P1Y(years and months duration))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0618(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + [1..10](range<number>))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0619(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(incompatible types in addition: 10(number) + FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any)(function<Any>->Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0620(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_number_to_rhs_null";
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
fn _0621(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0622(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0623(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0624(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0625(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0626(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0627(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0628(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0629(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0630(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0631(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0632(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_string_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0633(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0634(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0635(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0636(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0637(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0638(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0639(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0640(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0641(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0642(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0643(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0644(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0645(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_boolean_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0646(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0647(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0648(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0649(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0650(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0651(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0652(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0653(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0654(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0655(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0656(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_date_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0657(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0658(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0659(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0660(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0661(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0662(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0663(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0664(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0665(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0666(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0667(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dateAndTime_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0668(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0669(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0670(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0671(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is date)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0672(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is date and time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0673(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0674(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0675(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0676(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is years and months duration)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0677(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0678(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0679(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_time_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0680(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0681(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0682(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0683(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0684(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0685(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0686(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0687(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0688(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0689(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0690(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0691(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0692(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_list_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0693(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0694(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0695(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0696(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0697(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0698(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0699(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0700(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0701(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0702(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0703(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0704(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0705(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_context_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0706(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0707(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0708(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0709(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0710(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0711(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is years and months duration)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0712(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0713(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0714(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_dtDuration_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0715(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0716(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is string)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0717(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is boolean)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0718(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is time)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0719(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is list<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0720(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is context<a: number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0721(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is days and time duration)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0722(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0723(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0724(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_ymDuration_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0725(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0726(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0727(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0728(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0729(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0730(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0731(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0732(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0733(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0734(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0735(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0736(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0737(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_range_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0738(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0739(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0740(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0741(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0742(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0743(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0744(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0745(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0746(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0747(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0748(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0749(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0750(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_function_to_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0751(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_number";
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
fn _0752(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_string";
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
fn _0753(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_boolean";
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
fn _0754(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_date";
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
fn _0755(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_dateAndTime";
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
fn _0756(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_time";
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
fn _0757(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_list";
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
fn _0758(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_context";
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
fn _0759(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_dtDuration";
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
fn _0760(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_ymDuration";
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
fn _0761(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_range";
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
fn _0762(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_function";
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
fn _0763(b: &mut Bencher) {
    let invocable_name = "error_when_add_lhs_null_to_rhs_null";
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
fn _0764(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0765(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0766(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0767(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0768(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0769(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0770(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0771(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0772(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0773(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0774(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0775(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_number_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10 - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0776(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0777(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0778(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0779(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0780(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0781(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0782(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0783(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0784(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0785(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0786(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0787(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0788(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_string_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: "10" - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0789(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0790(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0791(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0792(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0793(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0794(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0795(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0796(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0797(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0798(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0799(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0800(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0801(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_boolean_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: true - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0802(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0803(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0804(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0805(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0806(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0807(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0808(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0809(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0810(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_date_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0811(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0812(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0813(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0814(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0815(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0816(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0817(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0818(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0819(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dateAndTime_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0820(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0821(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0822(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0823(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0824(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0825(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0826(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0827(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0828(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0829(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0830(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_time_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0831(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0832(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0833(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0834(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0835(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0836(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0837(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0838(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0839(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0840(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0841(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0842(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0843(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_list_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [10] - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0844(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0845(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0846(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0847(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0848(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0849(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0850(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0851(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0852(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0853(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0854(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0855(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0856(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_context_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0857(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0858(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0859(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0860(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0861(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0862(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0863(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0864(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0865(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0866(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0867(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0868(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_dtDuration_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1D - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0869(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0870(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0871(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0872(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0873(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0874(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0875(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0876(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0877(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0878(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0879(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0880(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_ymDuration_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0881(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0882(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0883(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0884(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0885(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0886(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0887(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0888(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0889(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0890(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0891(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0892(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0893(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_range_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0894(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0895(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0896(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0897(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0898(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0899(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0900(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0901(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0902(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0903(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0904(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0905(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0906(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_function_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0907(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - 10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0908(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - "10")"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0909(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - true)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0910(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - 2021-01-01)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0911(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - 2021-01-01T10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0912(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - 10:10:10)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0913(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - [10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0914(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - {a: 10})"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0915(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - P1D)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0916(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - P1Y)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0917(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - [1..10])"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0918(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0919(b: &mut Bencher) {
    let invocable_name = "error_when_subtract_lhs_null_minus_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null([subtraction] incompatible types: null - null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0920(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0921(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0922(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0923(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0924(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0925(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0926(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0927(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0928(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0929(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0930(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0931(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_number_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0932(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0933(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0934(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0935(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0936(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0937(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0938(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0939(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0940(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0941(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0942(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0943(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0944(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_string_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0945(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0946(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0947(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0948(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0949(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0950(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0951(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0952(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0953(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0954(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0955(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0956(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0957(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_boolean_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0958(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0959(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0960(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0961(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0962(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0963(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0964(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0965(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0966(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0967(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0968(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0969(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0970(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_date_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0971(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0972(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0973(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0974(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0975(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0976(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0977(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0978(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0979(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0980(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0981(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0982(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0983(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dateAndTime_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0984(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0985(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0986(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0987(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0988(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0989(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0990(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0991(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0992(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0993(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0994(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0995(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0996(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_time_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0997(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0998(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _0999(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1000(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1001(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1002(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1003(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1004(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1005(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1006(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1007(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1008(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1009(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_list_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1010(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1011(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1012(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1013(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1014(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1015(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1016(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1017(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1018(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1019(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1020(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1021(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1022(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_context_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1023(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1024(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1025(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1026(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1027(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1028(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1029(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1030(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1031(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1032(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1033(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1034(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1035(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_dtDuration_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1036(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1037(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1038(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1039(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1040(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1041(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1042(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1043(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1044(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1045(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1046(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1047(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1048(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_ymDuration_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1049(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1050(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1051(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1052(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1053(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1054(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1055(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1056(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1057(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1058(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1059(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1060(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1061(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_range_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1062(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1063(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1064(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1065(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1066(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1067(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1068(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1069(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1070(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1071(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1072(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1073(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1074(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_function_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1075(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1076(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_string";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1077(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1078(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1079(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_dateAndTime";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1080(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1081(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_list";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1082(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_context";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1083(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_dtDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1084(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_ymDuration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1085(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_range";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1086(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_function";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}

#[bench]
fn _1087(b: &mut Bencher) {
    let invocable_name = "error_when_exponent_lhs_null_exp_rhs_null";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX)
    );
}
