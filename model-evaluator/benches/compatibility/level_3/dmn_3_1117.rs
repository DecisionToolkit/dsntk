use super::*;

from_examples!(DMN_3_1117);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_001_05fd7d6215";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid argument type, expected string, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_002_8c66ed2d1a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid argument type, expected date and time or date, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_003_335cff371a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid argument type, expected time, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_004_28ef3e7882";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid argument type, expected time, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_005_15df95b27a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid argument type, expected date and time or date, actual type is Null)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_006_8c794da0bb";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null(expected 1,2 parameters, actual number of parameters is 0)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_007_59863d1b57";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2012-12-24T00:00:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_008_83eb9a93ba";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T00:00:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_009_1982fa549c";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_010_59f5b47012";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"-2017-12-31T11:22:33"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_011_eec2d5bdcd";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""99999-12-31T11:22:33""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_012_225a105eef";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""-99999-12-31T11:22:33""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_013_c4fd0a0e8d";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33.345"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_014_ded0e5fe2f";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33.123456789"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0015(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_015_9e27148afd";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_016_c08e4d417a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33.567Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_017_47816add0e";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33+01:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_018_0614e473e7";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33-02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_019_c312e3dfe3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33+01:35"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0020(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_020_29e0585b6f";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33-01:35"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_021_99f0215b60";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-12-31T11:22:33.456+01:35"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0022(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_022_b8b20f0328";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"-2017-12-31T11:22:33.456+01:35"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0023(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_023_2e41497673";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2011-12-31T10:15:30@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0024(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_024_b4d1fb8735";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2011-12-31T10:15:30@Etc/UTC""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0025(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_025_0cb7f83ec6";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2011-12-31T10:15:30.987@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0026(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_026_5ba081cd5f";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2011-12-31T10:15:30.123456789@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0027(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_027_ae365197dd";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""262142-12-31T23:59:59.999999999@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0028(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_028_1c3d56275f";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""-262143-01-01T23:59:59.999999999+02:00""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0029(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_029_e3a5e786a0";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-01-01T23:59:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0030(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_030_2f97bff606";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-01-01T23:59:01Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0031(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_031_61e70c285f";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-01-01T23:59:01+02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0032(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_032_1e95e8726e";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2017-01-01T23:59:01@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0033(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_033_2fac4d6807";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2017-01-01T23:59:01.123456789@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0034(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_034_75580be3aa";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-08-10T23:59:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0035(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_035_831b1ad0c5";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-08-10T23:59:01.987654321"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0036(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_036_189e1c3095";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30+02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0037(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_037_c7aec7ecf7";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0038(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_038_1493e6d873";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30.987654321+02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0039(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_039_593292b25c";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30.123456Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0040(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_040_d9116e1daa";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2017-09-05T09:15:30.987654321@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0041(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_041_c6decfe6a3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-08-10T23:59:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0042(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_042_0cbcc3d1dc";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-08-10T23:59:01.987654321"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0043(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_043_2e4177d00c";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30+02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0044(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_044_9404547f9d";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0045(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_045_5d93a541eb";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30.987654321+02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0046(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_046_89c1cd8daa";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30.123456Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0047(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_047_60ea7838ce";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2017-09-05T09:15:30.987654321@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0048(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_048_e387922273";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-08-10T23:59:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0049(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_049_eb9cd1f777";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-08-10T23:59:01.987654321"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0050(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_050_2d960354af";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30+02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0051(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_051_46bdaa00b0";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0052(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_052_911dbd0a24";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30.987654321+02:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0053(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_053_283c083df9";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-09-05T09:15:30.123456Z"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0054(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_054_2561a406fc";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""2017-09-05T09:15:30.987654321@Europe/Paris""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0055(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_055_6ce9202e17";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid argument type, expected string, actual type is number)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0056(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_056_e66397568e";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid argument type, expected string, actual type is list<Null>)"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0057(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_057_0452ca8719";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0058(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_058_588040ceaa";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '11:00:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0059(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_059_dfc62a3ebc";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2011-12-0310:15:30')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0060(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_060_890c302575";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2011-12-03T10:15:30+01:00@Europe/Paris')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0061(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_061_38ea1fc94d";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '9999999999-12-27T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0062(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_062_528aa370a3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-13-10T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0063(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_063_2c94303011";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-00-10T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0064(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_064_926a372666";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-13-32T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0065(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_065_a13de18ee4";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-13-0T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0066(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_066_e9f3d6d2c2";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '998-12-31T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0067(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_067_35fef99b53";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '01211-12-31T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0068(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_068_abaa1c2774";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '+99999-12-01T11:22:33')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0069(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_069_ca84e9c806";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T24:00:01')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0070(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_070_889c75a0cf";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T24:01:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0071(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_071_e90b813dfe";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T25:00:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0072(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_072_9f3e9b9c21";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T00:60:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0073(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_073_717548bec6";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T00:00:61')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0074(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_074_a15e7f8d29";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T7:00:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0075(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_075_4c3b8e7097";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T07:1:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0076(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_076_4d31fed18e";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T07:01:2')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0077(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_077_f83b3ac8bb";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00@xyz/abc')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0078(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_078_e113dabcdd";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+19:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0079(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_079_2e6f80eb94";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00-19:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0080(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_080_69de952053";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+05:0')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0081(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_081_e063215a7c";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+5:00')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0082(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_082_5b6ed4e801";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+5')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0083(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_083_4f41731f2a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T13:20:00+02:00@Europe/Paris')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0084(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_084_c633b01603";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T7:20')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0085(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_ErrorCase_085_a604a1bc80";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"null([core::date and time] invalid date or date and time '2017-12-31T07:2')"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0086(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_086_12ca8ac1d3";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2012-12-24T23:59:00"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0087(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_087_e9fd32063a";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-01-01T23:59:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0088(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "feel-date-and-time-function_088_1db0287718";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2017-01-01T23:59:01"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
