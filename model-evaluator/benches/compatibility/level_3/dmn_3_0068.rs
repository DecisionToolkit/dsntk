use super::*;

from_examples!(DMN_3_0068);
static_context!(CTX, "{}");
static_context!(CTXN, "{null_input: null}");

fn bench(b: &mut Bencher, invocable: &str, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

fn bench_n(b: &mut Bencher, invocable: &str, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTXN,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTXN)
    );
}

#[bench]
fn _0001(b: &mut Bencher) {
    bench(b, "null_001", "true");
}

#[bench]
fn _0002(b: &mut Bencher) {
    bench(b, "null_002", "false");
}

#[bench]
fn _0003(b: &mut Bencher) {
    bench(b, "boolean_001", "true");
}

#[bench]
fn _0004(b: &mut Bencher) {
    bench(b, "boolean_002", "false");
}

#[bench]
fn _0005(b: &mut Bencher) {
    bench(b, "boolean_003", "true");
}

#[bench]
fn _0006(b: &mut Bencher) {
    bench(b, "boolean_004", "false");
}

#[bench]
fn _0007(b: &mut Bencher) {
    bench(b, "boolean_005", "false");
}

#[bench]
fn _0008(b: &mut Bencher) {
    bench(b, "boolean_006", "false");
}

#[bench]
fn _0009(b: &mut Bencher) {
    bench(b, "boolean_007", "false");
}

#[bench]
fn _0010(b: &mut Bencher) {
    bench(b, "boolean_008", "null(equal err 'false' =?= '0')");
}

#[bench]
fn _0011(b: &mut Bencher) {
    bench(b, "boolean_009", "null(equal err 'true' =?= '1')");
}

#[bench]
fn _0012(b: &mut Bencher) {
    bench(b, "number_001", "true");
}

#[bench]
fn _0013(b: &mut Bencher) {
    bench(b, "number_002", "true");
}

#[bench]
fn _0014(b: &mut Bencher) {
    bench(b, "number_003", "true");
}

#[bench]
fn _0015(b: &mut Bencher) {
    bench(b, "number_004", "true");
}

#[bench]
fn _0016(b: &mut Bencher) {
    bench(b, "number_005", "false");
}

#[bench]
fn _0017(b: &mut Bencher) {
    bench(b, "number_006", "false");
}

#[bench]
fn _0018(b: &mut Bencher) {
    bench(b, "number_007", r#"null(equal err '100' =?= '"100"')"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
    bench(b, "string_001", "true");
}

#[bench]
fn _0020(b: &mut Bencher) {
    bench(b, "string_002", "false");
}

#[bench]
fn _0021(b: &mut Bencher) {
    bench(b, "string_003", "true");
}

#[bench]
fn _0022(b: &mut Bencher) {
    bench(b, "string_004", "false");
}

#[bench]
fn _0023(b: &mut Bencher) {
    bench(b, "string_005", r#"null(equal err '"foo"' =?= '100')"#);
}

#[bench]
fn _0024(b: &mut Bencher) {
    bench(b, "list_001", "true");
}

#[bench]
fn _0025(b: &mut Bencher) {
    bench(b, "list_002", "false");
}

#[bench]
fn _0026(b: &mut Bencher) {
    bench(b, "list_003", "false");
}

#[bench]
fn _0027(b: &mut Bencher) {
    bench(b, "list_004", "true");
}

#[bench]
fn _0028(b: &mut Bencher) {
    bench(b, "list_005", "false");
}

#[bench]
fn _0029(b: &mut Bencher) {
    bench(b, "list_006", "true");
}

#[bench]
fn _0030(b: &mut Bencher) {
    bench(b, "list_007", "true");
}

#[bench]
fn _0031(b: &mut Bencher) {
    bench(b, "list_008", "true");
}

#[bench]
fn _0032(b: &mut Bencher) {
    bench(b, "list_009", "true");
}

#[bench]
fn _0033(b: &mut Bencher) {
    bench(b, "list_010", "true");
}

#[bench]
fn _0034(b: &mut Bencher) {
    bench(b, "list_011", "true");
}

#[bench]
fn _0035(b: &mut Bencher) {
    bench(b, "list_012", "true");
}

#[bench]
fn _0036(b: &mut Bencher) {
    bench(b, "list_013", "true");
}

#[bench]
fn _0037(b: &mut Bencher) {
    bench(b, "list_014", "true");
}

#[bench]
fn _0038(b: &mut Bencher) {
    bench(b, "list_015", "false");
}

#[bench]
fn _0039(b: &mut Bencher) {
    bench(b, "list_016", "null(equal err '[]' =?= '0')");
}

#[bench]
fn _0040(b: &mut Bencher) {
    bench(b, "context_001", "true");
}

#[bench]
fn _0041(b: &mut Bencher) {
    bench(b, "context_002", "true");
}

#[bench]
fn _0042(b: &mut Bencher) {
    bench(b, "context_003", "true");
}

#[bench]
fn _0043(b: &mut Bencher) {
    bench(b, "context_004", "true");
}

#[bench]
fn _0044(b: &mut Bencher) {
    bench(b, "context_005", "false");
}

#[bench]
fn _0045(b: &mut Bencher) {
    bench(b, "context_006", "false");
}

#[bench]
fn _0046(b: &mut Bencher) {
    bench(b, "context_007", "null(equal err '{}' =?= '[]')");
}

#[bench]
fn _0047(b: &mut Bencher) {
    bench(b, "date_001", "true");
}

#[bench]
fn _0048(b: &mut Bencher) {
    bench(b, "date_002", "false");
}

#[bench]
fn _0049(b: &mut Bencher) {
    bench(b, "date_003", "false");
}

#[bench]
fn _0050(b: &mut Bencher) {
    bench(b, "date_004", "null(equal err '2018-12-07' =?= '100')");
}

#[bench]
fn _0051(b: &mut Bencher) {
    bench(b, "time_001", "true");
}

#[bench]
fn _0052(b: &mut Bencher) {
    bench(b, "time_002", "false");
}

#[bench]
fn _0053(b: &mut Bencher) {
    bench(b, "time_002_a", "false");
}

#[bench]
fn _0054(b: &mut Bencher) {
    bench(b, "time_002_b", "false");
}

#[bench]
fn _0055(b: &mut Bencher) {
    bench(b, "time_003", "true");
}

#[bench]
fn _0056(b: &mut Bencher) {
    bench(b, "time_004", "true");
}

#[bench]
fn _0057(b: &mut Bencher) {
    bench(b, "time_005", "true");
}

#[bench]
fn _0058(b: &mut Bencher) {
    bench(b, "time_006", "true");
}

#[bench]
fn _0059(b: &mut Bencher) {
    bench(b, "time_010", "true");
}

#[bench]
fn _0060(b: &mut Bencher) {
    bench(b, "time_011", "false");
}

#[bench]
fn _0061(b: &mut Bencher) {
    bench(b, "time_012", "null(equal err '10:30:00' =?= '100')");
}

#[bench]
fn _0062(b: &mut Bencher) {
    bench(b, "datetime_001", "true");
}

#[bench]
fn _0063(b: &mut Bencher) {
    bench(b, "datetime_002", "true");
}

#[bench]
fn _0064(b: &mut Bencher) {
    bench(b, "datetime_003", "true");
}

#[bench]
fn _0065(b: &mut Bencher) {
    bench(b, "datetime_003_a", "true");
}

#[bench]
fn _0066(b: &mut Bencher) {
    bench(b, "datetime_004", "false");
}

#[bench]
fn _0067(b: &mut Bencher) {
    bench(b, "datetime_005", "true");
}

#[bench]
fn _0068(b: &mut Bencher) {
    bench(b, "datetime_006", "false");
}

/*
// Commented out, pending RTF equality comparisons with zones clarification.
#[bench]
fn _0069(b: &mut Bencher) {
  bench(b, "datetime_007", "false");
}
*/

#[bench]
fn _0070(b: &mut Bencher) {
    bench(b, "datetime_008", "true");
}

#[bench]
fn _0071(b: &mut Bencher) {
    bench(b, "datetime_008_a", "true");
}

#[bench]
fn _0072(b: &mut Bencher) {
    bench(b, "datetime_008_b", "true");
}

#[bench]
fn _0073(b: &mut Bencher) {
    bench(b, "datetime_009", "true");
}

#[bench]
fn _0074(b: &mut Bencher) {
    bench(b, "datetime_010", "false");
}

#[bench]
fn _0075(b: &mut Bencher) {
    bench(
        b,
        "datetime_011",
        "null(equal err '2018-12-08T00:00:00' =?= '100')",
    );
}

#[bench]
fn _0076(b: &mut Bencher) {
    bench(b, "datetime_012", "true");
}

#[bench]
fn _0077(b: &mut Bencher) {
    bench(b, "datetime_013", "true");
}

#[bench]
fn _0078(b: &mut Bencher) {
    bench(b, "dt_duration_001", "true");
}

#[bench]
fn _0079(b: &mut Bencher) {
    bench(b, "dt_duration_002", "true");
}

#[bench]
fn _0080(b: &mut Bencher) {
    bench(b, "dt_duration_003", "false");
}

#[bench]
fn _0081(b: &mut Bencher) {
    bench(b, "dt_duration_004", "false");
}

#[bench]
fn _0082(b: &mut Bencher) {
    bench(b, "dt_duration_005", "true");
}

#[bench]
fn _0083(b: &mut Bencher) {
    bench(b, "dt_duration_006", "false");
}

#[bench]
fn _0084(b: &mut Bencher) {
    bench(b, "dt_duration_007", "null(equal err 'PT0S' =?= '0')");
}

#[bench]
fn _0085(b: &mut Bencher) {
    bench(b, "ym_duration_001", "true");
}

#[bench]
fn _0086(b: &mut Bencher) {
    bench(b, "ym_duration_002", "true");
}

#[bench]
fn _0087(b: &mut Bencher) {
    bench(b, "ym_duration_003", "false");
}

#[bench]
fn _0088(b: &mut Bencher) {
    bench(b, "ym_duration_004", "false");
}

#[bench]
fn _0089(b: &mut Bencher) {
    bench(b, "ym_duration_005", "true");
}

#[bench]
fn _0090(b: &mut Bencher) {
    bench(b, "ym_duration_006", "null(equal err 'P1Y' =?= 'P365D')");
}

#[bench]
fn _0091(b: &mut Bencher) {
    bench(b, "ym_duration_007", "false");
}

#[bench]
fn _0092(b: &mut Bencher) {
    bench(b, "ym_duration_008", "null(equal err 'P0M' =?= '0')");
}

#[bench]
fn _0093(b: &mut Bencher) {
    bench(b, "deep_001", "true");
}

#[bench]
fn _0094(b: &mut Bencher) {
    bench(b, "deep_002", "true");
}

#[bench]
fn _0095(b: &mut Bencher) {
    bench(b, "deep_003", "false");
}

#[bench]
fn _0096(b: &mut Bencher) {
    bench(b, "deep_004", "false");
}

#[bench]
fn _0097(b: &mut Bencher) {
    bench(b, "deep_005", "true");
}

#[bench]
fn _0098(b: &mut Bencher) {
    bench(b, "deep_006", "true");
}

#[bench]
fn _0099(b: &mut Bencher) {
    bench(b, "deep_007", "false");
}

#[bench]
fn _0100(b: &mut Bencher) {
    bench_n(b, "range_001", "true");
}

#[bench]
fn _0101(b: &mut Bencher) {
    bench_n(b, "range_002", "false");
}

#[bench]
fn _0102(b: &mut Bencher) {
    bench_n(b, "range_003", "false");
}

#[bench]
fn _0103(b: &mut Bencher) {
    bench_n(b, "range_004", "true");
}

#[bench]
fn _0104(b: &mut Bencher) {
    bench_n(b, "range_005", "true");
}

#[bench]
fn _0105(b: &mut Bencher) {
    bench_n(b, "range_006", "false");
}

#[bench]
fn _0106(b: &mut Bencher) {
    bench_n(b, "range_006_a", "true");
}

#[bench]
fn _0107(b: &mut Bencher) {
    bench_n(b, "range_007", "false");
}

#[bench]
fn _0108(b: &mut Bencher) {
    bench_n(b, "range_008", "false");
}

#[bench]
fn _0109(b: &mut Bencher) {
    bench_n(b, "range_009", "false");
}

#[bench]
fn _0110(b: &mut Bencher) {
    bench_n(b, "range_010", "false");
}

#[bench]
fn _0111(b: &mut Bencher) {
    bench_n(b, "range_011", "true");
}

#[bench]
fn _0112(b: &mut Bencher) {
    bench_n(b, "range_012", "true");
}
