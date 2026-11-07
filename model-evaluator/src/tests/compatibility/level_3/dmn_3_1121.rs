use super::*;

from_examples!(DMN_3_1121);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_001_b24a0c91f2",
        &ctx,
        r#"null(expected 2 parameters, actual number of parameters is 1)"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_002_4e7651ae0e",
        &ctx,
        r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_003_0886738d31",
        &ctx,
        r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_004_1bdfef922b",
        &ctx,
        r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_005_d0a077da4e",
        &ctx,
        r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_006_f20de28d3f",
        &ctx,
        r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_007_0921c3d61a",
        &ctx,
        r#"null(expected 2 parameters, actual number of parameters is 0)"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_008_015d35b442",
        &ctx,
        r#"P1Y8M"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_009_635028a5d8",
        &ctx,
        r#"-P1Y8M"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_010_caaa2e5002",
        &ctx,
        r#"P1Y"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_011_3fac022eb0",
        &ctx,
        r#"-P1Y"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_012_331ef38ce0",
        &ctx,
        r#"P0M"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_013_2f3cc46d9d",
        &ctx,
        r#"P0M"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_014_1fadbba7cd",
        &ctx,
        r#"P1Y2M"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_015_0e496f94fc",
        &ctx,
        r#"P7Y6M"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_016_b38662aa93",
        &ctx,
        r#"P4Y9M"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_017_86744b9a54",
        &ctx,
        r#"-P11M"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_018_8a9ed1d66d",
        &ctx,
        r#"-P4033Y2M"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_019_90c2084588",
        &ctx,
        r#"-P4035Y11M"#,
    );
}

#[test]
fn _0020() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_020_8ead9a0377",
        &ctx,
        r#"P2Y"#,
    );
}

#[test]
fn _0021() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_021_8a7d311ae9",
        &ctx,
        r#"P11M"#,
    );
}

#[test]
fn _0022() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_022_87e369773b",
        &ctx,
        r#"P5Y7M"#,
    );
}

#[test]
fn _0023() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_023_6385c7a83e",
        &ctx,
        r#"P1Y"#,
    );
}

#[test]
fn _0024() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_024_e96d1bd93a",
        &ctx,
        r#"P4Y"#,
    );
}

#[test]
fn _0025() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_025_161f6fca54",
        &ctx,
        r#"P2Y9M"#,
    );
}

#[test]
fn _0026() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_026_fcc906b375",
        &ctx,
        r#"P3Y"#,
    );
}

#[test]
fn _0027() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_027_3374dd86c6",
        &ctx,
        r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0028() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_028_77600e7b35",
        &ctx,
        r#"null(expected 2 parameters, actual number of parameters is 1)"#,
    );
}

#[test]
fn _0029() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_029_15a0d0d9c1",
        &ctx,
        r#"null(expected 2 parameters, actual number of parameters is 1)"#,
    );
}

#[test]
fn _0030() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_ErrorCase_030_ec16878596",
        &ctx,
        r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is list<Null>)"#,
    );
}

#[test]
fn _0031() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_031_4fd9c09d89",
        &ctx,
        r#"P4Y3M"#,
    );
}

#[test]
fn _0032() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_032_2a09ac80d0",
        &ctx,
        r#"P2Y4M"#,
    );
}

#[test]
fn _0033() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_033_7333eca866",
        &ctx,
        r#"P1Y"#,
    );
}

#[test]
fn _0034() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_034_c2cc06724c",
        &ctx,
        r#"P2Y"#,
    );
}

#[test]
fn _0035() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_035_dc05f9555d",
        &ctx,
        r#"P1Y8M"#,
    );
}

#[test]
fn _0036() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-years-and-months-duration-function_036_f8c8b02ba3",
        &ctx,
        r#"-P1Y"#,
    );
}
