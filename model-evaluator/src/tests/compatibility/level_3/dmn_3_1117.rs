use super::*;

from_examples!(DMN_3_1117);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_001_05fd7d6215",
        &ctx,
        r#"null([core::date and time] invalid argument type, expected string, actual type is Null)"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_002_8c66ed2d1a",
        &ctx,
        r#"null([core::date and time] invalid argument type, expected date and time or date, actual type is Null)"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_003_335cff371a",
        &ctx,
        r#"null([core::date and time] invalid argument type, expected time, actual type is Null)"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_004_28ef3e7882",
        &ctx,
        r#"null([core::date and time] invalid argument type, expected time, actual type is Null)"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_005_15df95b27a",
        &ctx,
        r#"null([core::date and time] invalid argument type, expected date and time or date, actual type is Null)"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_006_8c794da0bb",
        &ctx,
        r#"null(expected 1,2 parameters, actual number of parameters is 0)"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_007_59863d1b57",
        &ctx,
        r#"2012-12-24T00:00:00"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_008_83eb9a93ba",
        &ctx,
        r#"2017-12-31T00:00:00"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_009_1982fa549c",
        &ctx,
        r#"2017-12-31T11:22:33"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_010_59f5b47012",
        &ctx,
        r#"-2017-12-31T11:22:33"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_011_eec2d5bdcd",
        &ctx,
        r#""99999-12-31T11:22:33""#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_012_225a105eef",
        &ctx,
        r#""-99999-12-31T11:22:33""#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_013_c4fd0a0e8d",
        &ctx,
        r#"2017-12-31T11:22:33.345"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_014_ded0e5fe2f",
        &ctx,
        r#"2017-12-31T11:22:33.123456789"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_015_9e27148afd",
        &ctx,
        r#"2017-12-31T11:22:33Z"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_016_c08e4d417a",
        &ctx,
        r#"2017-12-31T11:22:33.567Z"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_017_47816add0e",
        &ctx,
        r#"2017-12-31T11:22:33+01:00"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_018_0614e473e7",
        &ctx,
        r#"2017-12-31T11:22:33-02:00"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_019_c312e3dfe3",
        &ctx,
        r#"2017-12-31T11:22:33+01:35"#,
    );
}

#[test]
fn _0020() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_020_29e0585b6f",
        &ctx,
        r#"2017-12-31T11:22:33-01:35"#,
    );
}

#[test]
fn _0021() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_021_99f0215b60",
        &ctx,
        r#"2017-12-31T11:22:33.456+01:35"#,
    );
}

#[test]
fn _0022() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_022_b8b20f0328",
        &ctx,
        r#"-2017-12-31T11:22:33.456+01:35"#,
    );
}

#[test]
fn _0023() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_023_2e41497673",
        &ctx,
        r#""2011-12-31T10:15:30@Europe/Paris""#,
    );
}

#[test]
fn _0024() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_024_b4d1fb8735",
        &ctx,
        r#""2011-12-31T10:15:30@Etc/UTC""#,
    );
}

#[test]
fn _0025() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_025_0cb7f83ec6",
        &ctx,
        r#""2011-12-31T10:15:30.987@Europe/Paris""#,
    );
}

#[test]
fn _0026() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_026_5ba081cd5f",
        &ctx,
        r#""2011-12-31T10:15:30.123456789@Europe/Paris""#,
    );
}

#[test]
fn _0027() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_027_ae365197dd",
        &ctx,
        r#""262142-12-31T23:59:59.999999999@Europe/Paris""#,
    );
}

#[test]
fn _0028() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_028_1c3d56275f",
        &ctx,
        r#""-262143-01-01T23:59:59.999999999+02:00""#,
    );
}

#[test]
fn _0029() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_029_e3a5e786a0",
        &ctx,
        r#"2017-01-01T23:59:01"#,
    );
}

#[test]
fn _0030() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_030_2f97bff606",
        &ctx,
        r#"2017-01-01T23:59:01Z"#,
    );
}

#[test]
fn _0031() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_031_61e70c285f",
        &ctx,
        r#"2017-01-01T23:59:01+02:00"#,
    );
}

#[test]
fn _0032() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_032_1e95e8726e",
        &ctx,
        r#""2017-01-01T23:59:01@Europe/Paris""#,
    );
}

#[test]
fn _0033() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_033_2fac4d6807",
        &ctx,
        r#""2017-01-01T23:59:01.123456789@Europe/Paris""#,
    );
}

#[test]
fn _0034() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_034_75580be3aa",
        &ctx,
        r#"2017-08-10T23:59:01"#,
    );
}

#[test]
fn _0035() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_035_831b1ad0c5",
        &ctx,
        r#"2017-08-10T23:59:01.987654321"#,
    );
}

#[test]
fn _0036() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_036_189e1c3095",
        &ctx,
        r#"2017-09-05T09:15:30+02:00"#,
    );
}

#[test]
fn _0037() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_037_c7aec7ecf7",
        &ctx,
        r#"2017-09-05T09:15:30Z"#,
    );
}

#[test]
fn _0038() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_038_1493e6d873",
        &ctx,
        r#"2017-09-05T09:15:30.987654321+02:00"#,
    );
}

#[test]
fn _0039() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_039_593292b25c",
        &ctx,
        r#"2017-09-05T09:15:30.123456Z"#,
    );
}

#[test]
fn _0040() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_040_d9116e1daa",
        &ctx,
        r#""2017-09-05T09:15:30.987654321@Europe/Paris""#,
    );
}

#[test]
fn _0041() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_041_c6decfe6a3",
        &ctx,
        r#"2017-08-10T23:59:01"#,
    );
}

#[test]
fn _0042() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_042_0cbcc3d1dc",
        &ctx,
        r#"2017-08-10T23:59:01.987654321"#,
    );
}

#[test]
fn _0043() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_043_2e4177d00c",
        &ctx,
        r#"2017-09-05T09:15:30+02:00"#,
    );
}

#[test]
fn _0044() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_044_9404547f9d",
        &ctx,
        r#"2017-09-05T09:15:30Z"#,
    );
}

#[test]
fn _0045() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_045_5d93a541eb",
        &ctx,
        r#"2017-09-05T09:15:30.987654321+02:00"#,
    );
}

#[test]
fn _0046() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_046_89c1cd8daa",
        &ctx,
        r#"2017-09-05T09:15:30.123456Z"#,
    );
}

#[test]
fn _0047() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_047_60ea7838ce",
        &ctx,
        r#""2017-09-05T09:15:30.987654321@Europe/Paris""#,
    );
}

#[test]
fn _0048() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_048_e387922273",
        &ctx,
        r#"2017-08-10T23:59:01"#,
    );
}

#[test]
fn _0049() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_049_eb9cd1f777",
        &ctx,
        r#"2017-08-10T23:59:01.987654321"#,
    );
}

#[test]
fn _0050() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_050_2d960354af",
        &ctx,
        r#"2017-09-05T09:15:30+02:00"#,
    );
}

#[test]
fn _0051() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_051_46bdaa00b0",
        &ctx,
        r#"2017-09-05T09:15:30Z"#,
    );
}

#[test]
fn _0052() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_052_911dbd0a24",
        &ctx,
        r#"2017-09-05T09:15:30.987654321+02:00"#,
    );
}

#[test]
fn _0053() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_053_283c083df9",
        &ctx,
        r#"2017-09-05T09:15:30.123456Z"#,
    );
}

#[test]
fn _0054() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_054_2561a406fc",
        &ctx,
        r#""2017-09-05T09:15:30.987654321@Europe/Paris""#,
    );
}

#[test]
fn _0055() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_055_6ce9202e17",
        &ctx,
        r#"null([core::date and time] invalid argument type, expected string, actual type is number)"#,
    );
}

#[test]
fn _0056() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_056_e66397568e",
        &ctx,
        r#"null([core::date and time] invalid argument type, expected string, actual type is list<Null>)"#,
    );
}

#[test]
fn _0057() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_057_0452ca8719",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '')"#,
    );
}

#[test]
fn _0058() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_058_588040ceaa",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '11:00:00')"#,
    );
}

#[test]
fn _0059() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_059_dfc62a3ebc",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2011-12-0310:15:30')"#,
    );
}

#[test]
fn _0060() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_060_890c302575",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2011-12-03T10:15:30+01:00@Europe/Paris')"#,
    );
}

#[test]
fn _0061() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_061_38ea1fc94d",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '9999999999-12-27T11:22:33')"#,
    );
}

#[test]
fn _0062() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_062_528aa370a3",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-13-10T11:22:33')"#,
    );
}

#[test]
fn _0063() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_063_2c94303011",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-00-10T11:22:33')"#,
    );
}

#[test]
fn _0064() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_064_926a372666",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-13-32T11:22:33')"#,
    );
}

#[test]
fn _0065() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_065_a13de18ee4",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-13-0T11:22:33')"#,
    );
}

#[test]
fn _0066() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_066_e9f3d6d2c2",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '998-12-31T11:22:33')"#,
    );
}

#[test]
fn _0067() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_067_35fef99b53",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '01211-12-31T11:22:33')"#,
    );
}

#[test]
fn _0068() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_068_abaa1c2774",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '+99999-12-01T11:22:33')"#,
    );
}

#[test]
fn _0069() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_069_ca84e9c806",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T24:00:01')"#,
    );
}

#[test]
fn _0070() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_070_889c75a0cf",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T24:01:00')"#,
    );
}

#[test]
fn _0071() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_071_e90b813dfe",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T25:00:00')"#,
    );
}

#[test]
fn _0072() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_072_9f3e9b9c21",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T00:60:00')"#,
    );
}

#[test]
fn _0073() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_073_717548bec6",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T00:00:61')"#,
    );
}

#[test]
fn _0074() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_074_a15e7f8d29",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T7:00:00')"#,
    );
}

#[test]
fn _0075() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_075_4c3b8e7097",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T07:1:00')"#,
    );
}

#[test]
fn _0076() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_076_4d31fed18e",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T07:01:2')"#,
    );
}

#[test]
fn _0077() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_077_f83b3ac8bb",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00@xyz/abc')"#,
    );
}

#[test]
fn _0078() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_078_e113dabcdd",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+19:00')"#,
    );
}

#[test]
fn _0079() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_079_2e6f80eb94",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00-19:00')"#,
    );
}

#[test]
fn _0080() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_080_69de952053",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+05:0')"#,
    );
}

#[test]
fn _0081() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_081_e063215a7c",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+5:00')"#,
    );
}

#[test]
fn _0082() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_082_5b6ed4e801",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+5')"#,
    );
}

#[test]
fn _0083() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_083_4f41731f2a",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+02:00@Europe/Paris')"#,
    );
}

#[test]
fn _0084() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_084_c633b01603",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T7:20')"#,
    );
}

#[test]
fn _0085() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_ErrorCase_085_a604a1bc80",
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T07:2')"#,
    );
}

#[test]
fn _0086() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_086_12ca8ac1d3",
        &ctx,
        r#"2012-12-24T23:59:00"#,
    );
}

#[test]
fn _0087() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_087_e9fd32063a",
        &ctx,
        r#"2017-01-01T23:59:01"#,
    );
}

#[test]
fn _0088() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-date-and-time-function_088_1db0287718",
        &ctx,
        r#"2017-01-01T23:59:01"#,
    );
}
