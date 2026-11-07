use super::*;

from_examples!(DMN_3_0068);
static_context!(CTX, "{}");
static_context!(CTXN, "{null_input: null}");

fn test(invocable: &str, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
}

fn test_n(invocable: &str, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTXN,
        expected,
    );
}

#[test]
fn _0001() {
    test("null_001", "true");
}

#[test]
fn _0002() {
    test("null_002", "false");
}

#[test]
fn _0003() {
    test("boolean_001", "true");
}

#[test]
fn _0004() {
    test("boolean_002", "false");
}

#[test]
fn _0005() {
    test("boolean_003", "true");
}

#[test]
fn _0006() {
    test("boolean_004", "false");
}

#[test]
fn _0007() {
    test("boolean_005", "false");
}

#[test]
fn _0008() {
    test("boolean_006", "false");
}

#[test]
fn _0009() {
    test("boolean_007", "false");
}

#[test]
fn _0010() {
    test("boolean_008", "null(equal err 'false' =?= '0')");
}

#[test]
fn _0011() {
    test("boolean_009", "null(equal err 'true' =?= '1')");
}

#[test]
fn _0012() {
    test("number_001", "true");
}

#[test]
fn _0013() {
    test("number_002", "true");
}

#[test]
fn _0014() {
    test("number_003", "true");
}

#[test]
fn _0015() {
    test("number_004", "true");
}

#[test]
fn _0016() {
    test("number_005", "false");
}

#[test]
fn _0017() {
    test("number_006", "false");
}

#[test]
fn _0018() {
    test("number_007", r#"null(equal err '100' =?= '"100"')"#);
}

#[test]
fn _0019() {
    test("string_001", "true");
}

#[test]
fn _0020() {
    test("string_002", "false");
}

#[test]
fn _0021() {
    test("string_003", "true");
}

#[test]
fn _0022() {
    test("string_004", "false");
}

#[test]
fn _0023() {
    test("string_005", r#"null(equal err '"foo"' =?= '100')"#);
}

#[test]
fn _0024() {
    test("list_001", "true");
}

#[test]
fn _0025() {
    test("list_002", "false");
}

#[test]
fn _0026() {
    test("list_003", "false");
}

#[test]
fn _0027() {
    test("list_004", "true");
}

#[test]
fn _0028() {
    test("list_005", "false");
}

#[test]
fn _0029() {
    test("list_006", "true");
}

#[test]
fn _0030() {
    test("list_007", "true");
}

#[test]
fn _0031() {
    test("list_008", "true");
}

#[test]
fn _0032() {
    test("list_009", "true");
}

#[test]
fn _0033() {
    test("list_010", "true");
}

#[test]
fn _0034() {
    test("list_011", "true");
}

#[test]
fn _0035() {
    test("list_012", "true");
}

#[test]
fn _0036() {
    test("list_013", "true");
}

#[test]
fn _0037() {
    test("list_014", "true");
}

#[test]
fn _0038() {
    test("list_015", "false");
}

#[test]
fn _0039() {
    test("list_016", "null(equal err '[]' =?= '0')");
}

#[test]
fn _0040() {
    test("context_001", "true");
}

#[test]
fn _0041() {
    test("context_002", "true");
}

#[test]
fn _0042() {
    test("context_003", "true");
}

#[test]
fn _0043() {
    test("context_004", "true");
}

#[test]
fn _0044() {
    test("context_005", "false");
}

#[test]
fn _0045() {
    test("context_006", "false");
}

#[test]
fn _0046() {
    test("context_007", "null(equal err '{}' =?= '[]')");
}

#[test]
fn _0047() {
    test("date_001", "true");
}

#[test]
fn _0048() {
    test("date_002", "false");
}

#[test]
fn _0049() {
    test("date_003", "false");
}

#[test]
fn _0050() {
    test("date_004", "null(equal err '2018-12-07' =?= '100')");
}

#[test]
fn _0051() {
    test("time_001", "true");
}

#[test]
fn _0052() {
    test("time_002", "false");
}

#[test]
fn _0053() {
    test("time_002_a", "false");
}

#[test]
fn _0054() {
    test("time_002_b", "false");
}

#[test]
fn _0055() {
    test("time_003", "true");
}

#[test]
fn _0056() {
    test("time_004", "true");
}

#[test]
fn _0057() {
    test("time_005", "true");
}

#[test]
fn _0058() {
    test("time_006", "true");
}

#[test]
fn _0059() {
    test("time_010", "true");
}

#[test]
fn _0060() {
    test("time_011", "false");
}

#[test]
fn _0061() {
    test("time_012", "null(equal err '10:30:00' =?= '100')");
}

#[test]
fn _0062() {
    test("datetime_001", "true");
}

#[test]
fn _0063() {
    test("datetime_002", "true");
}

#[test]
fn _0064() {
    test("datetime_003", "true");
}

#[test]
fn _0065() {
    test("datetime_003_a", "true");
}

#[test]
fn _0066() {
    test("datetime_004", "false");
}

#[test]
fn _0067() {
    test("datetime_005", "true");
}

#[test]
fn _0068() {
    test("datetime_006", "false");
}

/*
// Commented out, pending RTF equality comparisons with zones clarification.
#[test]
fn _0069() {
  test("datetime_007", "false");
}
*/

#[test]
fn _0070() {
    test("datetime_008", "true");
}

#[test]
fn _0071() {
    test("datetime_008_a", "true");
}

#[test]
fn _0072() {
    test("datetime_008_b", "true");
}

#[test]
fn _0073() {
    test("datetime_009", "true");
}

#[test]
fn _0074() {
    test("datetime_010", "false");
}

#[test]
fn _0075() {
    test(
        "datetime_011",
        "null(equal err '2018-12-08T00:00:00' =?= '100')",
    );
}

#[test]
fn _0076() {
    test("datetime_012", "true");
}

#[test]
fn _0077() {
    test("datetime_013", "true");
}

#[test]
fn _0078() {
    test("dt_duration_001", "true");
}

#[test]
fn _0079() {
    test("dt_duration_002", "true");
}

#[test]
fn _0080() {
    test("dt_duration_003", "false");
}

#[test]
fn _0081() {
    test("dt_duration_004", "false");
}

#[test]
fn _0082() {
    test("dt_duration_005", "true");
}

#[test]
fn _0083() {
    test("dt_duration_006", "false");
}

#[test]
fn _0084() {
    test("dt_duration_007", "null(equal err 'PT0S' =?= '0')");
}

#[test]
fn _0085() {
    test("ym_duration_001", "true");
}

#[test]
fn _0086() {
    test("ym_duration_002", "true");
}

#[test]
fn _0087() {
    test("ym_duration_003", "false");
}

#[test]
fn _0088() {
    test("ym_duration_004", "false");
}

#[test]
fn _0089() {
    test("ym_duration_005", "true");
}

#[test]
fn _0090() {
    test("ym_duration_006", "null(equal err 'P1Y' =?= 'P365D')");
}

#[test]
fn _0091() {
    test("ym_duration_007", "false");
}

#[test]
fn _0092() {
    test("ym_duration_008", "null(equal err 'P0M' =?= '0')");
}

#[test]
fn _0093() {
    test("deep_001", "true");
}

#[test]
fn _0094() {
    test("deep_002", "true");
}

#[test]
fn _0095() {
    test("deep_003", "false");
}

#[test]
fn _0096() {
    test("deep_004", "false");
}

#[test]
fn _0097() {
    test("deep_005", "true");
}

#[test]
fn _0098() {
    test("deep_006", "true");
}

#[test]
fn _0099() {
    test("deep_007", "false");
}

#[test]
fn _0100() {
    test_n("range_001", "true");
}

#[test]
fn _0101() {
    test_n("range_002", "false");
}

#[test]
fn _0102() {
    test_n("range_003", "false");
}

#[test]
fn _0103() {
    test_n("range_004", "true");
}

#[test]
fn _0104() {
    test_n("range_005", "true");
}

#[test]
fn _0105() {
    test_n("range_006", "false");
}

#[test]
fn _0106() {
    test_n("range_006_a", "true");
}

#[test]
fn _0107() {
    test_n("range_007", "false");
}

#[test]
fn _0108() {
    test_n("range_008", "false");
}

#[test]
fn _0109() {
    test_n("range_009", "false");
}

#[test]
fn _0110() {
    test_n("range_010", "false");
}

#[test]
fn _0111() {
    test_n("range_011", "true");
}

#[test]
fn _0112() {
    test_n("range_012", "true");
}
