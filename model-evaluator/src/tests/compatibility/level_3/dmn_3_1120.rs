use super::*;

from_examples!(DMN_3_1120);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_001_f2c6cd6866",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_002_ddca5756ca",
        &ctx,
        r#"null(expected 1 parameters, actual number of parameters is 0)"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_003_951e1d1c31",
        &ctx,
        r#"P1D"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_004_6b31e7cde7",
        &ctx,
        r#"PT2H"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_005_202d863d07",
        &ctx,
        r#"PT3M"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_006_a885f926d9",
        &ctx,
        r#"PT4S"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_007_2f0ad399f3",
        &ctx,
        r#"PT0.999S"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_008_747f56743d",
        &ctx,
        r#"P1DT2H3M4.123456789S"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_009_cef3c1ed26",
        &ctx,
        r#"PT0S"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_010_5b452a4975",
        &ctx,
        r#"PT0S"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_011_2169615b94",
        &ctx,
        r#"PT0S"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_012_2affe6d169",
        &ctx,
        r#"PT0S"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_013_0e8e26513c",
        &ctx,
        r#"PT0S"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_014_598ba6fabd",
        &ctx,
        r#"PT0S"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_015_ce2cb09830",
        &ctx,
        r#"-PT1H2M"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_016_af3e37fdbd",
        &ctx,
        r#"PT16H40M"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_017_4f4549fda4",
        &ctx,
        r#"PT16H40M0.999999999S"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_018_f5ec776811",
        &ctx,
        r#"PT9H15M"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_019_2e6885755a",
        &ctx,
        r#"PT1H1M"#,
    );
}

#[test]
fn _0020() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_020_af58b3766e",
        &ctx,
        r#"P1D"#,
    );
}

#[test]
fn _0021() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_021_e48e70ad4e",
        &ctx,
        r#"P10D"#,
    );
}

#[test]
fn _0022() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_022_668f24bed7",
        &ctx,
        r#"P2DT1H40M"#,
    );
}

#[test]
fn _0023() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_023_6fc32087db",
        &ctx,
        r#"PT1H"#,
    );
}

#[test]
fn _0024() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_024_fd7000d72f",
        &ctx,
        r#"P2DT4H34M"#,
    );
}

#[test]
fn _0025() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_025_f8ffbd8658",
        &ctx,
        r#"P1Y2M"#,
    );
}

#[test]
fn _0026() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_026_e6c47f0cae",
        &ctx,
        r#"P1Y"#,
    );
}

#[test]
fn _0027() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_027_33b7fb8704",
        &ctx,
        r#"P0M"#,
    );
}

#[test]
fn _0028() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_028_971b94f16d",
        &ctx,
        r#"P0M"#,
    );
}

#[test]
fn _0029() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_029_1a12a226cc",
        &ctx,
        r#"-P1Y"#,
    );
}

#[test]
fn _0030() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_030_afac0f2062",
        &ctx,
        r#"P2Y2M"#,
    );
}

#[test]
fn _0031() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_031_1ddad718b9",
        &ctx,
        r#"P3Y3M"#,
    );
}

#[test]
fn _0032() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_032_72c46a9ec9",
        &ctx,
        r#"P8Y4M"#,
    );
}

#[test]
fn _0033() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_033_5d1540abaf",
        &ctx,
        r#"-P8Y4M"#,
    );
}

#[test]
fn _0034() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_034_aa9cbb21a6",
        &ctx,
        r#"P83333333Y3M"#,
    );
}

#[test]
fn _0035() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_035_93eef01ae7",
        &ctx,
        r#"-P83333333Y3M"#,
    );
}

#[test]
fn _0036() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_036_5f2775875e",
        &ctx,
        r#"P99999999Y"#,
    );
}

#[test]
fn _0037() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_037_8c9ea9c0e6",
        &ctx,
        r#"-P99999999Y"#,
    );
}

#[test]
fn _0038() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_038_67dc4c254c",
        &ctx,
        r#"P1Y"#,
    );
}

#[test]
fn _0039() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_039_4aa0b67804",
        &ctx,
        r#"P1D"#,
    );
}

#[test]
fn _0040() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_040_7d8eae461f",
        &ctx,
        r#"P2Y2M"#,
    );
}

#[test]
fn _0041() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_041_264bc9d682",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0042() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_042_59a0000245",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0043() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_043_253815dc6c",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0044() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_044_f3b338d877",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0045() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_045_2ffcc37801",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0046() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_046_eb637de5f6",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0047() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_047_3210c46a5a",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0048() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_048_ab6244f767",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0049() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_049_2225b503a0",
        &ctx,
        r#"null(duration)"#,
    );
}

#[test]
fn _0050() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-duration-function_ErrorCase_050_dd2ef33bbd",
        &ctx,
        r#"null(duration)"#,
    );
}
