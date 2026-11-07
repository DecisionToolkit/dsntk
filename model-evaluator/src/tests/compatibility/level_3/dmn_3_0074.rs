use super::*;

from_examples!(DMN_3_0074);
static_context!(CTX, "{}");

#[test]
fn _0001() {
    let invocable = "context_001";
    let expected = r#""foo""#;
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
fn _0002() {
    let invocable = "date_001";
    let expected = "2018";
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
fn _0003() {
    let invocable = "date_002";
    let expected = "12";
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
fn _0004() {
    let invocable = "date_003";
    let expected = "10";
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
fn _0005() {
    let invocable = "date_004";
    let expected = "1";
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
fn _0006() {
    let invocable = "dateTime_001";
    let expected = "2018";
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
fn _0007() {
    let invocable = "dateTime_002";
    let expected = "12";
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
fn _0008() {
    let invocable = "dateTime_003";
    let expected = "10";
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
fn _0009() {
    let invocable = "dateTime_004";
    let expected = "1";
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
fn _0010() {
    let invocable = "dateTime_005";
    let expected = "10";
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
fn _0011() {
    let invocable = "dateTime_005_a";
    let expected = "0";
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
fn _0012() {
    let invocable = "dateTime_006";
    let expected = "30";
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
fn _0013() {
    let invocable = "dateTime_006_a";
    let expected = "0";
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
fn _0014() {
    let invocable = "dateTime_007";
    let expected = "1";
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
fn _0015() {
    let invocable = "dateTime_007_a";
    let expected = "0";
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
fn _0016() {
    let invocable = "dateTime_008";
    let expected = "PT5H";
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
fn _0017() {
    let invocable = "dateTime_008_a";
    let expected = "null(could not retrieve time offset for date and time)";

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
fn _0018() {
    let invocable = "dateTime_009";
    let expected = r#""Etc/UTC""#;
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
fn _0019() {
    let invocable = "dateTime_009_a";
    let expected = "null(could not retrieve timezone for date and time)";
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
fn _0020() {
    let invocable = "time_001";
    let expected = "10";
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
fn _0021() {
    let invocable = "time_002";
    let expected = "30";
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
fn _0022() {
    let invocable = "time_003";
    let expected = "1";
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
fn _0023() {
    let invocable = "time_004";
    let expected = "PT5H";
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
fn _0024() {
    let invocable = "time_004_a";
    let expected = "null(could not retrieve time offset for time)";

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
fn _0025() {
    let invocable = "time_005_a";
    let expected = "null(could not retrieve timezone for time)";

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
fn _0026() {
    let invocable = "ym_duration_001";
    let expected = "1";
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
fn _0027() {
    let invocable = "ym_duration_001_a";
    let expected = "0";
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
fn _0028() {
    let invocable = "ym_duration_002";
    let expected = "2";
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
fn _0029() {
    let invocable = "ym_duration_002_a";
    let expected = "0";
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
fn _0030() {
    let invocable = "ym_duration_003";
    let expected = "null(no such property in years and months duration: days)";

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
fn _0031() {
    let invocable = "ym_duration_004";
    let expected = "null(no such property in years and months duration: hours)";

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
fn _0032() {
    let invocable = "ym_duration_005";
    let expected = "null(no such property in years and months duration: minutes)";

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
fn _0033() {
    let invocable = "ym_duration_006";
    let expected = "null(no such property in years and months duration: seconds)";

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
fn _0034() {
    let invocable = "dt_duration_001";
    let expected = "null(no such property in days and time duration: years)";

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
fn _0035() {
    let invocable = "dt_duration_002";
    let expected = "null(no such property in days and time duration: months)";

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
fn _0036() {
    let invocable = "dt_duration_003";
    let expected = "1";
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
fn _0037() {
    let invocable = "dt_duration_003_a";
    let expected = "0";
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
fn _0038() {
    let invocable = "dt_duration_004";
    let expected = "2";
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
fn _0039() {
    let invocable = "dt_duration_004_a";
    let expected = "0";
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
fn _0040() {
    let invocable = "dt_duration_005";
    let expected = "2";
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
fn _0041() {
    let invocable = "dt_duration_005_a";
    let expected = "0";
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
fn _0042() {
    let invocable = "dt_duration_006";
    let expected = "2";
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
fn _0043() {
    let invocable = "dt_duration_006_a";
    let expected = "0";
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
fn _0044() {
    let invocable = "range_001";
    let expected = "1";
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
fn _0045() {
    let invocable = "range_002";
    let expected = "10";
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
fn _0046() {
    let invocable = "range_003";
    let expected = "true";
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
fn _0047() {
    let invocable = "range_004";
    let expected = "false";
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
fn _0048() {
    let invocable = "range_005";
    let expected = "false";
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
fn _0049() {
    let invocable = "range_006";
    let expected = "true";
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
fn _0050() {
    let invocable = "range_007";
    let expected = "false";
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
fn _0051() {
    let invocable = "range_008";
    let expected = "false";
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
fn _0052() {
    let invocable = "range_009";
    let expected = "null(no such property in unary less: start)";

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
fn _0053() {
    let invocable = "range_010";
    let expected = "null(no such property in unary less or equal: start)";

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
fn _0054() {
    let invocable = "range_011";
    let expected = "10";
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
fn _0055() {
    let invocable = "range_012";
    let expected = "10";
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
fn _0056() {
    let invocable = "range_013";
    let expected = "10";
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
fn _0057() {
    let invocable = "range_014";
    let expected = "10";
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
fn _0058() {
    let invocable = "range_015";
    let expected = "null(no such property in unary greater: end)";

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
fn _0059() {
    let invocable = "range_016";
    let expected = "null(no such property in unary greater or equal: end)";

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
fn _0060() {
    let invocable = "range_017";
    let expected = "false";
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
fn _0061() {
    let invocable = "range_018";
    let expected = "false";
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
fn _0062() {
    let invocable = "range_019";
    let expected = "false";
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
fn _0063() {
    let invocable = "range_020";
    let expected = "true";
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
fn _0064() {
    let invocable = "range_021";
    let expected = "false";
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
fn _0065() {
    let invocable = "range_022";
    let expected = "true";
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
fn _0066() {
    let invocable = "range_023";
    let expected = "false";
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
fn _0067() {
    let invocable = "range_024";
    let expected = "false";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
}
