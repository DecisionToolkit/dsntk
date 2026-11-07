use super::*;

from_examples!(DMN_3_1116);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_001_bdf26fdc72",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_002_9d2e399b96",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_003_d1f0ea5bb9",
        &ctx,
        r#"null([core::time_4] minutes must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_004_57aea91d1c",
        &ctx,
        r#"null([core::time_4] seconds must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_005_32ea20b34f",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_006_e266498180",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_007_ee82c7bf12",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_008_08078c6c29",
        &ctx,
        r#"null([core::time_4] minutes must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_009_804c21ed52",
        &ctx,
        r#"null([core::time_4] seconds must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_010_cc773bb44b",
        &ctx,
        r#"null([core::time_4] minutes must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_011_ad5b3a26b5",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_012_3c2f416fc9",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_013_7f22c0bda8",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_014_0dc13176e8",
        &ctx,
        r#"null([core::time_4] hour must be a number, current type is: Null)"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_015_376d693a79",
        &ctx,
        r#"12:00:00"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_016_c3cccff405",
        &ctx,
        r#"null(expected 1,3,4 parameters, actual number of parameters is 0)"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_017_f3683885f5",
        &ctx,
        r#"01:02:03"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_018_35f1f2cce8",
        &ctx,
        r#"00:00:00"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_019_879be89d63",
        &ctx,
        r#"11:22:33.444"#,
    );
}

#[test]
fn _0020() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_020_72b421086e",
        &ctx,
        r#"11:22:33.123456789"#,
    );
}

#[test]
fn _0021() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_021_5c50fa1dff",
        &ctx,
        r#"23:59:00Z"#,
    );
}

#[test]
fn _0022() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_022_55e76d3595",
        &ctx,
        r#"11:00:00Z"#,
    );
}

#[test]
fn _0023() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_023_5cbbb85435",
        &ctx,
        r#"00:00:00Z"#,
    );
}

#[test]
fn _0024() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_024_5f7f735e8f",
        &ctx,
        r#"13:20:00+02:00"#,
    );
}

#[test]
fn _0025() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_025_139b25b795",
        &ctx,
        r#"13:20:00-05:00"#,
    );
}

#[test]
fn _0026() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_026_c5208af118",
        &ctx,
        r#"11:22:33Z"#,
    );
}

#[test]
fn _0027() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_027_45082fd26c",
        &ctx,
        r#"11:22:33Z"#,
    );
}

#[test]
fn _0028() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_028_eaea7a943c",
        &ctx,
        r#""00:01:00@Etc/UTC""#,
    );
}

#[test]
fn _0029() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_029_f0d5c2c16a",
        &ctx,
        r#""00:01:00@Europe/Paris""#,
    );
}

#[test]
fn _0030() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_030_390d4f4648",
        &ctx,
        r#"10:20:00"#,
    );
}

#[test]
fn _0031() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_031_4d086a3b59",
        &ctx,
        r#"10:20:00Z"#,
    );
}

#[test]
fn _0032() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_032_d9b0d7f931",
        &ctx,
        r#"10:20:00Z"#,
    );
}

#[test]
fn _0033() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_033_8420160da1",
        &ctx,
        r#"10:20:00+01:00"#,
    );
}

#[test]
fn _0034() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_034_13c312c376",
        &ctx,
        r#"10:20:00-01:00"#,
    );
}

#[test]
fn _0035() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_035_fbfce88ac4",
        &ctx,
        r#"10:20:00Z"#,
    );
}

#[test]
fn _0036() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_036_eb05fabc01",
        &ctx,
        r#""10:20:00@Europe/Paris""#,
    );
}

#[test]
fn _0037() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_037_eed195f693",
        &ctx,
        r#""11:20:00@Asia/Dhaka""#,
    );
}

#[test]
fn _0038() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_038_05b311131c",
        &ctx,
        r#"11:59:45"#,
    );
}

#[test]
fn _0039() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_039_5b65992f0d",
        &ctx,
        r#"11:59:45Z"#,
    );
}

#[test]
fn _0040() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_040_6c9d17b491",
        &ctx,
        r#"11:59:45+02:00"#,
    );
}

#[test]
fn _0041() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_041_29a448d57e",
        &ctx,
        r#"11:59:45-02:00"#,
    );
}

#[test]
fn _0042() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_042_00146f2977",
        &ctx,
        r#"11:59:00+02:01"#,
    );
}

#[test]
fn _0043() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_043_2edfae8414",
        &ctx,
        r#"11:59:00-02:01"#,
    );
}

#[test]
fn _0044() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_044_3073ffd026",
        &ctx,
        r#"11:59:00+02:01"#,
    );
}

#[test]
fn _0045() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_045_ad1339e858",
        &ctx,
        r#"11:59:00-02:01"#,
    );
}

#[test]
fn _0046() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_046_7b80221ec1",
        &ctx,
        r#""11:59:45+02:45:55""#,
    );
}

#[test]
fn _0047() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_047_33cd7b9b15",
        &ctx,
        r#""11:59:45-02:45:55""#,
    );
}

#[test]
fn _0048() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_048_9bedd52886",
        &ctx,
        r#"11:59:45Z"#,
    );
}

#[test]
fn _0049() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_049_617d9e09d6",
        &ctx,
        r#"23:59:01"#,
    );
}

#[test]
fn _0050() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_050_524d9a8146",
        &ctx,
        r#"23:59:01.987654321"#,
    );
}

#[test]
fn _0051() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_051_a71d2a08f7",
        &ctx,
        r#"09:15:30+02:00"#,
    );
}

#[test]
fn _0052() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_052_d825d58888",
        &ctx,
        r#"09:15:30Z"#,
    );
}

#[test]
fn _0053() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_053_3d956966c0",
        &ctx,
        r#"00:00:00Z"#,
    );
}

#[test]
fn _0054() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_054_fdc3094237",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0055() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_055_9b47db6ea4",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0056() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_056_a8e828d64d",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0057() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_057_d039115cce",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0058() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_058_81dd4b1639",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0059() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_059_c7e1705fe1",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0060() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_060_0cf4734fae",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0061() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_061_da2717f085",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0062() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_062_6cd1313fa9",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0063() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_063_e85c40b474",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0064() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_064_df74038c67",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0065() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_065_79eaef6fee",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0066() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_066_5116e12fd3",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0067() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_067_8285edad7b",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0068() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_068_ad528abb23",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0069() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_069_5096701e2e",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0070() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_070_8b2e39f570",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0071() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_071_cf9417648b",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0072() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_072_4c8c3835e4",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0073() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_073_a5fc245959",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0074() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_074_387d4411ea",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0075() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_075_1606dda03d",
        &ctx,
        r#"null(time_1)"#,
    );
}

#[test]
fn _0076() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_076_cb117ca612",
        &ctx,
        r#"null([core::time_4] hour must be 0..23, current value is: 24)"#,
    );
}

#[test]
fn _0077() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_077_a4daad060c",
        &ctx,
        r#"null([core::time_4] hour must be 0..23, current value is: -24)"#,
    );
}

#[test]
fn _0078() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_078_c2fe73418b",
        &ctx,
        r#"null([core::time_4] minute must be 0..59, current value is: 60)"#,
    );
}

#[test]
fn _0079() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_ErrorCase_079_d2d226c3cd",
        &ctx,
        r#"null([core::time_4] second must be 0..59, current value is: 60)"#,
    );
}

#[test]
fn _0080() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_080_2bbb8c86af",
        &ctx,
        r#"23:59:00"#,
    );
}

#[test]
fn _0081() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_081_69f4e0231e",
        &ctx,
        r#"12:45:00"#,
    );
}

#[test]
fn _0082() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_082_36a78e5396",
        &ctx,
        r#"11:59:00+02:01"#,
    );
}

#[test]
fn _0083() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-time-function_083_6b608254c7",
        &ctx,
        r#"11:59:00-02:00"#,
    );
}
