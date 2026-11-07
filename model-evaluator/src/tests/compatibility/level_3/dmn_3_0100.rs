use super::*;

from_examples!(DMN_3_0100);

static_context!(CTX, r#"{}"#);

#[test]
fn _0001() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_number_001",
        &CTX,
        r#"100"#,
    );
}

#[test]
fn _0002() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_number_002",
        &CTX,
        r#"-100"#,
    );
}

#[test]
fn _0003() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_number_003",
        &CTX,
        r#"100"#,
    );
}

#[test]
fn _0004() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_number_004",
        &CTX,
        r#"0"#,
    );
}

#[test]
fn _0005() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_001",
        &CTX,
        r#"P10D"#,
    );
}

#[test]
fn _0006() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_002",
        &CTX,
        r#"-P10D"#,
    );
}

#[test]
fn _0007() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_003",
        &CTX,
        r#"P10D"#,
    );
}

#[test]
fn _0008() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_004",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0009() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_005",
        &CTX,
        r#"P1DT2H"#,
    );
}

#[test]
fn _0010() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_006",
        &CTX,
        r#"PT0S"#,
    );
}

#[test]
fn _0011() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_007",
        &CTX,
        r#"P6DT1H30M"#,
    );
}

#[test]
fn _0012() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_dtDuration_008",
        &CTX,
        r#"P2DT9H30M"#,
    );
}

#[test]
fn _0013() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_001",
        &CTX,
        r#"P10D"#,
    );
}

#[test]
fn _0014() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_002",
        &CTX,
        r#"-P10D"#,
    );
}

#[test]
fn _0015() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_003",
        &CTX,
        r#"P10D"#,
    );
}

#[test]
fn _0016() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_004",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0017() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_005",
        &CTX,
        r#"P1DT2H"#,
    );
}

#[test]
fn _0018() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_006",
        &CTX,
        r#"PT0S"#,
    );
}

#[test]
fn _0019() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_007",
        &CTX,
        r#"P6DT1H30M"#,
    );
}

#[test]
fn _0020() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_dtDuration_by_rhs_number_008",
        &CTX,
        r#"P2DT9H30M"#,
    );
}

#[test]
fn _0021() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_001",
        &CTX,
        r#"P10Y"#,
    );
}

#[test]
fn _0022() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_002",
        &CTX,
        r#"-P10Y"#,
    );
}

#[test]
fn _0023() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_003",
        &CTX,
        r#"P10Y"#,
    );
}

#[test]
fn _0024() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_004",
        &CTX,
        r#"P1Y"#,
    );
}

#[test]
fn _0025() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_005",
        &CTX,
        r#"P2Y"#,
    );
}

#[test]
fn _0026() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_006",
        &CTX,
        r#"P2Y2M"#,
    );
}

#[test]
fn _0027() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_007",
        &CTX,
        r#"P0M"#,
    );
}

#[test]
fn _0028() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_008",
        &CTX,
        r#"P3M"#,
    );
}

#[test]
fn _0029() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_009",
        &CTX,
        r#"-P3M"#,
    );
}

#[test]
fn _0030() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_number_by_rhs_ymDuration_010",
        &CTX,
        r#"-P4Y9M"#,
    );
}

#[test]
fn _0031() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_001",
        &CTX,
        r#"P10Y"#,
    );
}

#[test]
fn _0032() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_002",
        &CTX,
        r#"-P10Y"#,
    );
}

#[test]
fn _0033() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_003",
        &CTX,
        r#"P10Y"#,
    );
}

#[test]
fn _0034() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_004",
        &CTX,
        r#"P1Y"#,
    );
}

#[test]
fn _0035() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_005",
        &CTX,
        r#"P2Y"#,
    );
}

#[test]
fn _0036() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_006",
        &CTX,
        r#"P2Y2M"#,
    );
}

#[test]
fn _0037() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_007",
        &CTX,
        r#"P0M"#,
    );
}

#[test]
fn _0038() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_008",
        &CTX,
        r#"P3M"#,
    );
}

#[test]
fn _0039() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_009",
        &CTX,
        r#"-P3M"#,
    );
}

#[test]
fn _0040() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "multiply_lhs_ymDuration_by_rhs_number_010",
        &CTX,
        r#"-P4Y9M"#,
    );
}

#[test]
fn _0041() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_number_by_rhs_number_001",
        &CTX,
        r#"10"#,
    );
}

#[test]
fn _0042() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_number_by_rhs_number_002",
        &CTX,
        r#"null([division] division by zero)"#,
    );
}

#[test]
fn _0043() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_ymDuration_by_rhs_number_001",
        &CTX,
        r#"P1Y"#,
    );
}

#[test]
fn _0044() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_ymDuration_by_rhs_number_002",
        &CTX,
        r#"null([division] division by zero)"#,
    );
}

#[test]
fn _0045() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_ymDuration_by_rhs_number_003",
        &CTX,
        r#"P4Y4M"#,
    );
}

#[test]
fn _0046() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_ymDuration_by_rhs_number_004",
        &CTX,
        r#"-P4Y4M"#,
    );
}

#[test]
fn _0047() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_ymDuration_by_rhs_ymDuration_001",
        &CTX,
        r#"2"#,
    );
}

#[test]
fn _0048() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_ymDuration_by_rhs_ymDuration_002",
        &CTX,
        r#"null([division] incompatible types: P10D / P0M)"#,
    );
}

#[test]
fn _0049() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_dtDuration_by_rhs_number_001",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0050() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_dtDuration_by_rhs_number_002",
        &CTX,
        r#"null([division] division by zero)"#,
    );
}

#[test]
fn _0051() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_dtDuration_by_rhs_number_003",
        &CTX,
        r#"P4DT9H12M"#,
    );
}

#[test]
fn _0052() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_dtDuration_by_rhs_dtDuration_001",
        &CTX,
        r#"2"#,
    );
}

#[test]
fn _0053() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "divide_lhs_dtDuration_by_rhs_dtDuration_002",
        &CTX,
        r#"null([division] division by zero)"#,
    );
}

#[test]
fn _0054() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_number_to_rhs_number_001",
        &CTX,
        r#"15"#,
    );
}

#[test]
fn _0055() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_number_to_rhs_number_002",
        &CTX,
        r#"5"#,
    );
}

#[test]
fn _0056() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_number_to_rhs_number_003",
        &CTX,
        r#"-5"#,
    );
}

#[test]
fn _0057() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_number_to_rhs_number_004",
        &CTX,
        r#"-15"#,
    );
}

#[test]
fn _0058() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_number_to_rhs_number_005",
        &CTX,
        r#"3.4685"#,
    );
}

#[test]
fn _0059() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_001",
        &CTX,
        r#"2022-01-01T10:10:10"#,
    );
}

#[test]
fn _0060() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_002",
        &CTX,
        r#"2021-02-01T10:10:10"#,
    );
}

#[test]
fn _0061() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_003",
        &CTX,
        r#"2020-01-01T10:10:10"#,
    );
}

#[test]
fn _0062() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_004",
        &CTX,
        r#"2020-12-01T10:10:10"#,
    );
}

#[test]
fn _0063() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_005",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0064() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_006",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0065() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_007",
        &CTX,
        r#"2021-02-01T10:10:10+11:00"#,
    );
}

#[test]
fn _0066() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_008",
        &CTX,
        r#"2020-12-01T10:10:10+11:00"#,
    );
}

#[test]
fn _0067() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_009",
        &CTX,
        r#""2021-02-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0068() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_010",
        &CTX,
        r#""-2021-02-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0069() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_011",
        &CTX,
        r#""-2020-01-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0070() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_012",
        &CTX,
        r#""-2022-12-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0071() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_013",
        &CTX,
        r#""-2022-01-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0072() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_014",
        &CTX,
        r#""-2021-02-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0073() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_015",
        &CTX,
        r#""-2020-01-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0074() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_016",
        &CTX,
        r#""-2022-12-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0075() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_ymDuration_017",
        &CTX,
        r#""-2022-01-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0076() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_001",
        &CTX,
        r#"2022-01-01T10:10:10"#,
    );
}

#[test]
fn _0077() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_002",
        &CTX,
        r#"2021-02-01T10:10:10"#,
    );
}

#[test]
fn _0078() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_003",
        &CTX,
        r#"2020-01-01T10:10:10"#,
    );
}

#[test]
fn _0079() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_004",
        &CTX,
        r#"2020-12-01T10:10:10"#,
    );
}

#[test]
fn _0080() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_005",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0081() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_006",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0082() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_007",
        &CTX,
        r#"2021-02-01T10:10:10+11:00"#,
    );
}

#[test]
fn _0083() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_008",
        &CTX,
        r#"2020-12-01T10:10:10+11:00"#,
    );
}

#[test]
fn _0084() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_009",
        &CTX,
        r#""2021-02-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0085() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_010",
        &CTX,
        r#""-2021-02-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0086() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_011",
        &CTX,
        r#""-2020-01-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0087() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_012",
        &CTX,
        r#""-2022-12-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0088() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_013",
        &CTX,
        r#""-2022-01-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0089() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_014",
        &CTX,
        r#""-2021-02-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0090() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_015",
        &CTX,
        r#""-2020-01-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0091() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_016",
        &CTX,
        r#""-2022-12-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0092() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_dateAndTime_017",
        &CTX,
        r#""-2022-01-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0093() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_001",
        &CTX,
        r#"2021-01-13T11:10:10"#,
    );
}

#[test]
fn _0094() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_002",
        &CTX,
        r#"2021-01-13T11:10:10+11:00"#,
    );
}

#[test]
fn _0095() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_003",
        &CTX,
        r#"2021-01-02T10:10:10"#,
    );
}

#[test]
fn _0096() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_004",
        &CTX,
        r#"2021-01-01T11:10:10"#,
    );
}

#[test]
fn _0097() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_005",
        &CTX,
        r#"2020-12-31T10:10:10"#,
    );
}

#[test]
fn _0098() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_006",
        &CTX,
        r#"2021-01-01T09:10:10"#,
    );
}

#[test]
fn _0099() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_007",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0100() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_008",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0101() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_009",
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
}

#[test]
fn _0102() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_010",
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
}

#[test]
fn _0103() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_011",
        &CTX,
        r#"2021-01-01T11:10:10+11:00"#,
    );
}

#[test]
fn _0104() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_012",
        &CTX,
        r#"2021-01-01T09:10:10+11:00"#,
    );
}

#[test]
fn _0105() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_013",
        &CTX,
        r#""2021-01-13T11:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0106() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_014",
        &CTX,
        r#""-2021-01-01T11:10:10+11:00""#,
    );
}

#[test]
fn _0107() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_015",
        &CTX,
        r#""-2021-01-02T10:10:10+11:00""#,
    );
}

#[test]
fn _0108() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_016",
        &CTX,
        r#""-2021-01-01T09:10:10+11:00""#,
    );
}

#[test]
fn _0109() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_017",
        &CTX,
        r#""-2022-12-31T10:10:10+11:00""#,
    );
}

#[test]
fn _0110() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_018",
        &CTX,
        r#""-2021-01-01T11:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0111() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_019",
        &CTX,
        r#""-2021-01-02T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0112() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_020",
        &CTX,
        r#""-2021-01-01T09:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0113() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dateAndTime_to_rhs_dtDuration_021",
        &CTX,
        r#""-2022-12-31T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0114() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_001",
        &CTX,
        r#"2021-01-13T11:10:10"#,
    );
}

#[test]
fn _0115() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_002",
        &CTX,
        r#"2021-01-13T11:10:10+11:00"#,
    );
}

#[test]
fn _0116() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_003",
        &CTX,
        r#"2021-01-02T10:10:10"#,
    );
}

#[test]
fn _0117() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_004",
        &CTX,
        r#"2021-01-01T11:10:10"#,
    );
}

#[test]
fn _0118() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_005",
        &CTX,
        r#"2020-12-31T10:10:10"#,
    );
}

#[test]
fn _0119() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_006",
        &CTX,
        r#"2021-01-01T09:10:10"#,
    );
}

#[test]
fn _0120() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_007",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0121() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_008",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0122() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_009",
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
}

#[test]
fn _0123() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_010",
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
}

#[test]
fn _0124() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_011",
        &CTX,
        r#"2021-01-01T11:10:10+11:00"#,
    );
}

#[test]
fn _0125() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_012",
        &CTX,
        r#"2021-01-01T09:10:10+11:00"#,
    );
}

#[test]
fn _0126() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_013",
        &CTX,
        r#""2021-01-13T11:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0127() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_014",
        &CTX,
        r#""-2021-01-01T11:10:10+11:00""#,
    );
}

#[test]
fn _0128() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_015",
        &CTX,
        r#""-2021-01-02T10:10:10+11:00""#,
    );
}

#[test]
fn _0129() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_016",
        &CTX,
        r#""-2021-01-01T09:10:10+11:00""#,
    );
}

#[test]
fn _0130() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_017",
        &CTX,
        r#""-2022-12-31T10:10:10+11:00""#,
    );
}

#[test]
fn _0131() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_018",
        &CTX,
        r#""-2021-01-01T11:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0132() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_019",
        &CTX,
        r#""-2021-01-02T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0133() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_020",
        &CTX,
        r#""-2021-01-01T09:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0134() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dateAndTime_021",
        &CTX,
        r#""-2022-12-31T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0135() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_date_to_rhs_ymDuration_001",
        &CTX,
        r#"2022-01-01"#,
    );
}

#[test]
fn _0136() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_date_to_rhs_ymDuration_002",
        &CTX,
        r#"2021-02-01"#,
    );
}

#[test]
fn _0137() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_date_001",
        &CTX,
        r#"2022-01-01"#,
    );
}

#[test]
fn _0138() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_date_002",
        &CTX,
        r#"2021-02-01"#,
    );
}

#[test]
fn _0139() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_date_to_rhs_dtDuration_001",
        &CTX,
        r#"2021-01-02"#,
    );
}

#[test]
fn _0140() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_date_to_rhs_dtDuration_002",
        &CTX,
        r#"2021-01-02"#,
    );
}

#[test]
fn _0141() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_date_to_rhs_dtDuration_003",
        &CTX,
        r#"2021-01-03"#,
    );
}

#[test]
fn _0142() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_date_001",
        &CTX,
        r#"2021-01-02"#,
    );
}

#[test]
fn _0143() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_date_002",
        &CTX,
        r#"2021-01-02"#,
    );
}

#[test]
fn _0144() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_date_003",
        &CTX,
        r#"2021-01-03"#,
    );
}

#[test]
fn _0145() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_time_to_rhs_dtDuration_001",
        &CTX,
        r#"11:15:00"#,
    );
}

#[test]
fn _0146() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_time_to_rhs_dtDuration_002",
        &CTX,
        r#"10:15:00"#,
    );
}

#[test]
fn _0147() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_time_to_rhs_dtDuration_003",
        &CTX,
        r#"10:15:00+11:00"#,
    );
}

#[test]
fn _0148() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_time_to_rhs_dtDuration_004",
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0149() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_time_to_rhs_dtDuration_005",
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0150() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_time_to_rhs_dtDuration_006",
        &CTX,
        r#""11:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0151() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_time_to_rhs_dtDuration_007",
        &CTX,
        r#""09:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0152() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_time_001",
        &CTX,
        r#"11:15:00"#,
    );
}

#[test]
fn _0153() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_time_002",
        &CTX,
        r#"10:15:00"#,
    );
}

#[test]
fn _0154() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_time_003",
        &CTX,
        r#"10:15:00+11:00"#,
    );
}

#[test]
fn _0155() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_time_004",
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0156() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_time_005",
        &CTX,
        r#""10:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0157() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_time_006",
        &CTX,
        r#""11:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0158() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_time_007",
        &CTX,
        r#""09:15:00@Australia/Melbourne""#,
    );
}

#[test]
fn _0159() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_ymDuration_001",
        &CTX,
        r#"P1Y2M"#,
    );
}

#[test]
fn _0160() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_ymDuration_002",
        &CTX,
        r#"P10M"#,
    );
}

#[test]
fn _0161() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_ymDuration_003",
        &CTX,
        r#"-P10M"#,
    );
}

#[test]
fn _0162() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_ymDuration_004",
        &CTX,
        r#"-P1Y2M"#,
    );
}

#[test]
fn _0163() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_ymDuration_005",
        &CTX,
        r#"P1Y"#,
    );
}

#[test]
fn _0164() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_ymDuration_to_rhs_ymDuration_006",
        &CTX,
        r#"P1Y"#,
    );
}

#[test]
fn _0165() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dtDuration_001",
        &CTX,
        r#"P3D"#,
    );
}

#[test]
fn _0166() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_dtDuration_to_rhs_dtDuration_002",
        &CTX,
        r#"P2D"#,
    );
}

#[test]
fn _0167() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_string_to_rhs_string_001",
        &CTX,
        r#""foobar""#,
    );
}

#[test]
fn _0168() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "add_lhs_string_to_rhs_string_002",
        &CTX,
        r#""11""#,
    );
}

#[test]
fn _0169() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_number_minus_rhs_number_001",
        &CTX,
        r#"5"#,
    );
}

#[test]
fn _0170() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_number_minus_rhs_number_002",
        &CTX,
        r#"15"#,
    );
}

#[test]
fn _0171() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_number_minus_rhs_number_003",
        &CTX,
        r#"-15"#,
    );
}

#[test]
fn _0172() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_number_minus_rhs_number_004",
        &CTX,
        r#"-5"#,
    );
}

#[test]
fn _0173() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_number_minus_rhs_number_005",
        &CTX,
        r#"0.8889"#,
    );
}

#[test]
fn _0174() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_number_minus_rhs_number_006",
        &CTX,
        r#"1.3579"#,
    );
}

#[test]
fn _0175() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_number_minus_rhs_number_007",
        &CTX,
        r#"-0.8889"#,
    );
}

#[test]
fn _0176() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_001",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0177() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_002",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10@Europe/Paris - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0178() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_003",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10 - 2021-01-01T10:10:10@Europe/Paris)"#,
    );
}

#[test]
fn _0179() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_004",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10+02:00 - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0180() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_005",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10 - 2021-01-01T10:10:10+02:00)"#,
    );
}

#[test]
fn _0181() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_006",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0182() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_007",
        &CTX,
        r#"P1DT5H"#,
    );
}

#[test]
fn _0183() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_008",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0184() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_009",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02T10:10:10+01:00 - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0185() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_010",
        &CTX,
        r#"PT0S"#,
    );
}

#[test]
fn _0186() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_011",
        &CTX,
        r#"P9498D"#,
    );
}

#[test]
fn _0187() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dateAndTime_012",
        &CTX,
        r#"P9498DT5H"#,
    );
}

#[test]
fn _0188() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_001",
        &CTX,
        r#"2020-01-01T10:10:10"#,
    );
}

#[test]
fn _0189() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_002",
        &CTX,
        r#"2020-12-01T10:10:10"#,
    );
}

#[test]
fn _0190() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_003",
        &CTX,
        r#"2022-01-01T10:10:10"#,
    );
}

#[test]
fn _0191() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_004",
        &CTX,
        r#"2021-02-01T10:10:10"#,
    );
}

#[test]
fn _0192() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_005",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0193() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_006",
        &CTX,
        r#"2021-01-01T10:10:10"#,
    );
}

#[test]
fn _0194() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_007",
        &CTX,
        r#"2020-01-01T10:10:10+11:00"#,
    );
}

#[test]
fn _0195() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_008",
        &CTX,
        r#""2020-01-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0196() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_009",
        &CTX,
        r#""-2022-12-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0197() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_010",
        &CTX,
        r#""-2022-01-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0198() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_011",
        &CTX,
        r#""-2021-02-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0199() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_012",
        &CTX,
        r#""-2020-01-01T10:10:10+11:00""#,
    );
}

#[test]
fn _0200() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_013",
        &CTX,
        r#""-2022-12-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0201() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_014",
        &CTX,
        r#""-2022-01-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0202() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_015",
        &CTX,
        r#""-2021-02-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0203() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_ymDuration_016",
        &CTX,
        r#""-2020-01-01T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0204() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_date_001",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T00:00:00 - 2021-01-02)"#,
    );
}

#[test]
fn _0205() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_date_002",
        &CTX,
        r#"P1DT9H10M10S"#,
    );
}

#[test]
fn _0206() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_date_003",
        &CTX,
        r#"P1DT9H10M10S"#,
    );
}

#[test]
fn _0207() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_date_004",
        &CTX,
        r#"P9498DT9H10M10S"#,
    );
}

#[test]
fn _0208() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_date_005",
        &CTX,
        r#"P9498DT9H10M10S"#,
    );
}

#[test]
fn _0209() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_001",
        &CTX,
        r#"2020-12-31T10:10:10"#,
    );
}

#[test]
fn _0210() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_002",
        &CTX,
        r#"2021-01-01T09:10:10"#,
    );
}

#[test]
fn _0211() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_003",
        &CTX,
        r#"2021-01-01T23:00:00"#,
    );
}

#[test]
fn _0212() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_004",
        &CTX,
        r#"2020-12-31T10:10:10+11:00"#,
    );
}

#[test]
fn _0213() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_005",
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
}

#[test]
fn _0214() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_006",
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
}

#[test]
fn _0215() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_007",
        &CTX,
        r#""2020-12-31T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0216() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_008",
        &CTX,
        r#"2021-01-01T23:59:59"#,
    );
}

#[test]
fn _0217() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_009",
        &CTX,
        r#"2021-01-02T00:00:01"#,
    );
}

#[test]
fn _0218() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_010",
        &CTX,
        r#""-2021-01-01T09:10:10+11:00""#,
    );
}

#[test]
fn _0219() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_011",
        &CTX,
        r#""-2022-12-31T10:10:10+11:00""#,
    );
}

#[test]
fn _0220() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_012",
        &CTX,
        r#""-2021-01-01T11:10:10+11:00""#,
    );
}

#[test]
fn _0221() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_013",
        &CTX,
        r#""-2021-01-02T10:10:10+11:00""#,
    );
}

#[test]
fn _0222() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_014",
        &CTX,
        r#""-2021-01-01T09:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0223() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_015",
        &CTX,
        r#""-2022-12-31T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0224() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_016",
        &CTX,
        r#""-2021-01-01T11:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0225() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dateAndTime_minus_rhs_dtDuration_017",
        &CTX,
        r#""-2021-01-02T10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0226() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dateAndTime_001",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-02 - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0227() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dateAndTime_002",
        &CTX,
        r#"P1DT49M50S"#,
    );
}

#[test]
fn _0228() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dateAndTime_003",
        &CTX,
        r#"P1DT49M50S"#,
    );
}

#[test]
fn _0229() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dateAndTime_004",
        &CTX,
        r#"PT0S"#,
    );
}

#[test]
fn _0230() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dateAndTime_005",
        &CTX,
        r#"P9498DT49M50S"#,
    );
}

#[test]
fn _0231() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_date_001",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0232() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_date_002",
        &CTX,
        r#"P9498D"#,
    );
}

#[test]
fn _0233() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_date_003",
        &CTX,
        r#"P9498D"#,
    );
}

#[test]
fn _0234() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_ymDuration_001",
        &CTX,
        r#"2020-01-02"#,
    );
}

#[test]
fn _0235() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_ymDuration_002",
        &CTX,
        r#"2021-01-01"#,
    );
}

#[test]
fn _0236() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_ymDuration_003",
        &CTX,
        r#"2020-12-02"#,
    );
}

#[test]
fn _0237() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_ymDuration_004",
        &CTX,
        r#"2022-01-02"#,
    );
}

#[test]
fn _0238() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_001",
        &CTX,
        r#"2021-01-01"#,
    );
}

#[test]
fn _0239() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_002",
        &CTX,
        r#"2021-01-01"#,
    );
}

#[test]
fn _0240() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_003",
        &CTX,
        r#"2021-01-01"#,
    );
}

#[test]
fn _0241() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_004",
        &CTX,
        r#"2021-01-01"#,
    );
}

#[test]
fn _0242() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_005",
        &CTX,
        r#"2021-01-02"#,
    );
}

#[test]
fn _0243() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_006",
        &CTX,
        r#"2021-01-02"#,
    );
}

#[test]
fn _0244() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_007",
        &CTX,
        r#"2021-01-02"#,
    );
}

#[test]
fn _0245() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_008",
        &CTX,
        r#"2021-01-01"#,
    );
}

#[test]
fn _0246() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_009",
        &CTX,
        r#"2020-12-31"#,
    );
}

#[test]
fn _0247() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_010",
        &CTX,
        r#"2021-01-03"#,
    );
}

#[test]
fn _0248() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_011",
        &CTX,
        r#"2021-01-03"#,
    );
}

#[test]
fn _0249() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_date_minus_rhs_dtDuration_012",
        &CTX,
        r#"2020-02-29"#,
    );
}

#[test]
fn _0250() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_time_001",
        &CTX,
        r#"PT1H"#,
    );
}

#[test]
fn _0251() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_time_002",
        &CTX,
        r#"-PT1H"#,
    );
}

#[test]
fn _0252() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_time_003",
        &CTX,
        r#"-PT1H"#,
    );
}

#[test]
fn _0253() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_time_004",
        &CTX,
        r#"PT1H"#,
    );
}

#[test]
fn _0254() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_time_005",
        &CTX,
        r#"-PT1H"#,
    );
}

#[test]
fn _0255() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_time_006",
        &CTX,
        r#"PT1H"#,
    );
}

#[test]
fn _0256() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_dtDuration_001",
        &CTX,
        r#"09:10:10"#,
    );
}

#[test]
fn _0257() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_dtDuration_002",
        &CTX,
        r#"10:10:10"#,
    );
}

#[test]
fn _0258() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_dtDuration_003",
        &CTX,
        r#"10:10:10+11:00"#,
    );
}

#[test]
fn _0259() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_dtDuration_004",
        &CTX,
        r#""10:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0260() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_dtDuration_005",
        &CTX,
        r#"09:10:10+11:00"#,
    );
}

#[test]
fn _0261() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_time_minus_rhs_dtDuration_006",
        &CTX,
        r#""09:10:10@Australia/Melbourne""#,
    );
}

#[test]
fn _0262() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_ymDuration_minus_rhs_ymDuration_001",
        &CTX,
        r#"P10M"#,
    );
}

#[test]
fn _0263() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_ymDuration_minus_rhs_ymDuration_002",
        &CTX,
        r#"-P1Y2M"#,
    );
}

#[test]
fn _0264() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_ymDuration_minus_rhs_ymDuration_003",
        &CTX,
        r#"-P10M"#,
    );
}

#[test]
fn _0265() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_ymDuration_minus_rhs_ymDuration_004",
        &CTX,
        r#"P1Y2M"#,
    );
}

#[test]
fn _0266() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_ymDuration_minus_rhs_ymDuration_005",
        &CTX,
        r#"P1Y"#,
    );
}

#[test]
fn _0267() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_ymDuration_minus_rhs_ymDuration_006",
        &CTX,
        r#"P1Y"#,
    );
}

#[test]
fn _0268() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dtDuration_minus_rhs_dtDuration_001",
        &CTX,
        r#"PT22H"#,
    );
}

#[test]
fn _0269() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dtDuration_minus_rhs_dtDuration_002",
        &CTX,
        r#"-P1DT2H"#,
    );
}

#[test]
fn _0270() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dtDuration_minus_rhs_dtDuration_003",
        &CTX,
        r#"-PT22H"#,
    );
}

#[test]
fn _0271() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dtDuration_minus_rhs_dtDuration_004",
        &CTX,
        r#"P1DT2H"#,
    );
}

#[test]
fn _0272() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dtDuration_minus_rhs_dtDuration_005",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0273() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "subtract_lhs_dtDuration_minus_rhs_dtDuration_006",
        &CTX,
        r#"P1D"#,
    );
}

#[test]
fn _0274() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "exponent_lhs_number_exp_rhs_number_001",
        &CTX,
        r#"25"#,
    );
}

#[test]
fn _0275() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "exponent_lhs_number_exp_rhs_number_002",
        &CTX,
        r#"0.04"#,
    );
}

#[test]
fn _0276() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "exponent_lhs_number_exp_rhs_number_003",
        &CTX,
        r#"3486784401"#,
    );
}

#[test]
fn _0277() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "exponent_lhs_number_exp_rhs_number_004",
        &CTX,
        r#"25"#,
    );
}

#[test]
fn _0278() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "exponent_lhs_number_exp_rhs_number_005",
        &CTX,
        r#"60.58617166606633673745724928244261"#,
    );
}

#[test]
fn _0279() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "exponent_lhs_number_exp_rhs_number_006",
        &CTX,
        r#"25"#,
    );
}

#[test]
fn _0280() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "exponent_lhs_number_exp_rhs_number_007",
        &CTX,
        r#"30.25"#,
    );
}

#[test]
fn _0281() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_string",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * "10")"#,
    );
}

#[test]
fn _0282() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_boolean",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * true)"#,
    );
}

#[test]
fn _0283() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_date",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * 2021-01-01)"#,
    );
}

#[test]
fn _0284() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_dateAndTime",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0285() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_time",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * 10:10:10)"#,
    );
}

#[test]
fn _0286() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_list",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * [10])"#,
    );
}

#[test]
fn _0287() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_context",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * {a: 10})"#,
    );
}

#[test]
fn _0288() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_range",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * [1..10])"#,
    );
}

#[test]
fn _0289() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_function",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0290() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_number_by_rhs_null",
        &CTX,
        r#"null([multiplication] incompatible types: 10 * null)"#,
    );
}

#[test]
fn _0291() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0292() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0293() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0294() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0295() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0296() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0297() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0298() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0299() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0300() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0301() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0302() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0303() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_string_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: string)"#,
    );
}

#[test]
fn _0304() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0305() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0306() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0307() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0308() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0309() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0310() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0311() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0312() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0313() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0314() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0315() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0316() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_boolean_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: boolean)"#,
    );
}

#[test]
fn _0317() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0318() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0319() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0320() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0321() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0322() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0323() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0324() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0325() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0326() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0327() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0328() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0329() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_date_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: date)"#,
    );
}

#[test]
fn _0330() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0331() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0332() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0333() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0334() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0335() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0336() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0337() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0338() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0339() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0340() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0341() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0342() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dateAndTime_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: date and time)"#,
    );
}

#[test]
fn _0343() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0344() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0345() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0346() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0347() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0348() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0349() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0350() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0351() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0352() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0353() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0354() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0355() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_time_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: time)"#,
    );
}

#[test]
fn _0356() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0357() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0358() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0359() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0360() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0361() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0362() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0363() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0364() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0365() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0366() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0367() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0368() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_list_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: list<number>)"#,
    );
}

#[test]
fn _0369() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0370() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0371() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0372() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0373() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0374() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0375() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0376() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0377() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0378() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0379() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0380() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0381() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_context_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: context<a: number>)"#,
    );
}

#[test]
fn _0382() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_string",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * "10")"#,
    );
}

#[test]
fn _0383() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_boolean",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * true)"#,
    );
}

#[test]
fn _0384() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_date",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * 2021-01-01)"#,
    );
}

#[test]
fn _0385() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_dateAndTime",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0386() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_time",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * 10:10:10)"#,
    );
}

#[test]
fn _0387() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_list",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * [10])"#,
    );
}

#[test]
fn _0388() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_context",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * {a: 10})"#,
    );
}

#[test]
fn _0389() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_dtDuration",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * P1D)"#,
    );
}

#[test]
fn _0390() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_ymDuration",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * P1Y)"#,
    );
}

#[test]
fn _0391() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_range",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * [1..10])"#,
    );
}

#[test]
fn _0392() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_function",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0393() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_dtDuration_by_rhs_null",
        &CTX,
        r#"null([multiplication] incompatible types: P1D * null)"#,
    );
}

#[test]
fn _0394() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_string",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * "10")"#,
    );
}

#[test]
fn _0395() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_boolean",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * true)"#,
    );
}

#[test]
fn _0396() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_date",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * 2021-01-01)"#,
    );
}

#[test]
fn _0397() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_dateAndTime",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0398() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_time",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * 10:10:10)"#,
    );
}

#[test]
fn _0399() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_list",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * [10])"#,
    );
}

#[test]
fn _0400() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_context",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * {a: 10})"#,
    );
}

#[test]
fn _0401() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_dtDuration",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * P1D)"#,
    );
}

#[test]
fn _0402() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_ymDuration",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * P1Y)"#,
    );
}

#[test]
fn _0403() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_range",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * [1..10])"#,
    );
}

#[test]
fn _0404() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_function",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0405() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_ymDuration_by_rhs_null",
        &CTX,
        r#"null([multiplication] incompatible types: P1Y * null)"#,
    );
}

#[test]
fn _0406() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0407() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0408() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0409() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0410() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0411() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0412() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0413() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0414() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0415() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0416() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0417() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0418() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_range_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: range<number>)"#,
    );
}

#[test]
fn _0419() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_number",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0420() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_string",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0421() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_boolean",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0422() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_date",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0423() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_dateAndTime",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0424() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_time",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0425() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_list",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0426() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_context",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0427() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_dtDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0428() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_ymDuration",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0429() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_range",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0430() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_function",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0431() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_function_by_rhs_null",
        &CTX,
        r#"null(unexpected value type in multiplication: function<Any>->Any)"#,
    );
}

#[test]
fn _0432() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_number",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0433() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_string",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0434() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_boolean",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0435() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_date",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0436() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_dateAndTime",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0437() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_time",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0438() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_list",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0439() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_context",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0440() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_dtDuration",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0441() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_ymDuration",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0442() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_range",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0443() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_function",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0444() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_multiply_lhs_null_by_rhs_null",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0445() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: 10 / "10")"#,
    );
}

#[test]
fn _0446() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: 10 / true)"#,
    );
}

#[test]
fn _0447() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: 10 / 2021-01-01)"#,
    );
}

#[test]
fn _0448() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: 10 / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0449() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: 10 / 10:10:10)"#,
    );
}

#[test]
fn _0450() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: 10 / [10])"#,
    );
}

#[test]
fn _0451() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: 10 / {a: 10})"#,
    );
}

#[test]
fn _0452() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: 10 / P1D)"#,
    );
}

#[test]
fn _0453() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: 10 / P1Y)"#,
    );
}

#[test]
fn _0454() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: 10 / [1..10])"#,
    );
}

#[test]
fn _0455() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: 10 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0456() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_number_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: 10 / null)"#,
    );
}

#[test]
fn _0457() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: "10" / 10)"#,
    );
}

#[test]
fn _0458() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: "10" / "10")"#,
    );
}

#[test]
fn _0459() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: "10" / true)"#,
    );
}

#[test]
fn _0460() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: "10" / 2021-01-01)"#,
    );
}

#[test]
fn _0461() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: "10" / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0462() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: "10" / 10:10:10)"#,
    );
}

#[test]
fn _0463() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: "10" / [10])"#,
    );
}

#[test]
fn _0464() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: "10" / {a: 10})"#,
    );
}

#[test]
fn _0465() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: "10" / P1D)"#,
    );
}

#[test]
fn _0466() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: "10" / P1Y)"#,
    );
}

#[test]
fn _0467() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: "10" / [1..10])"#,
    );
}

#[test]
fn _0468() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: "10" / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0469() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_string_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: "10" / null)"#,
    );
}

#[test]
fn _0470() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: true / 10)"#,
    );
}

#[test]
fn _0471() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: true / "10")"#,
    );
}

#[test]
fn _0472() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: true / true)"#,
    );
}

#[test]
fn _0473() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: true / 2021-01-01)"#,
    );
}

#[test]
fn _0474() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: true / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0475() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: true / 10:10:10)"#,
    );
}

#[test]
fn _0476() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: true / [10])"#,
    );
}

#[test]
fn _0477() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: true / {a: 10})"#,
    );
}

#[test]
fn _0478() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: true / P1D)"#,
    );
}

#[test]
fn _0479() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: true / P1Y)"#,
    );
}

#[test]
fn _0480() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: true / [1..10])"#,
    );
}

#[test]
fn _0481() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: true / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0482() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_boolean_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: true / null)"#,
    );
}

#[test]
fn _0483() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 10)"#,
    );
}

#[test]
fn _0484() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / "10")"#,
    );
}

#[test]
fn _0485() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / true)"#,
    );
}

#[test]
fn _0486() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 2021-01-01)"#,
    );
}

#[test]
fn _0487() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0488() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / 10:10:10)"#,
    );
}

#[test]
fn _0489() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / [10])"#,
    );
}

#[test]
fn _0490() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / {a: 10})"#,
    );
}

#[test]
fn _0491() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / P1D)"#,
    );
}

#[test]
fn _0492() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / P1Y)"#,
    );
}

#[test]
fn _0493() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / [1..10])"#,
    );
}

#[test]
fn _0494() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0495() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_date_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01 / null)"#,
    );
}

#[test]
fn _0496() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 10)"#,
    );
}

#[test]
fn _0497() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / "10")"#,
    );
}

#[test]
fn _0498() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / true)"#,
    );
}

#[test]
fn _0499() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 2021-01-01)"#,
    );
}

#[test]
fn _0500() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0501() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / 10:10:10)"#,
    );
}

#[test]
fn _0502() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / [10])"#,
    );
}

#[test]
fn _0503() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / {a: 10})"#,
    );
}

#[test]
fn _0504() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / P1D)"#,
    );
}

#[test]
fn _0505() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / P1Y)"#,
    );
}

#[test]
fn _0506() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / [1..10])"#,
    );
}

#[test]
fn _0507() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0508() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dateAndTime_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: 2021-01-01T10:10:10 / null)"#,
    );
}

#[test]
fn _0509() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 10)"#,
    );
}

#[test]
fn _0510() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / "10")"#,
    );
}

#[test]
fn _0511() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / true)"#,
    );
}

#[test]
fn _0512() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 2021-01-01)"#,
    );
}

#[test]
fn _0513() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0514() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / 10:10:10)"#,
    );
}

#[test]
fn _0515() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / [10])"#,
    );
}

#[test]
fn _0516() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / {a: 10})"#,
    );
}

#[test]
fn _0517() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / P1D)"#,
    );
}

#[test]
fn _0518() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / P1Y)"#,
    );
}

#[test]
fn _0519() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / [1..10])"#,
    );
}

#[test]
fn _0520() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0521() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_time_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: 10:10:10 / null)"#,
    );
}

#[test]
fn _0522() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: [10] / 10)"#,
    );
}

#[test]
fn _0523() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: [10] / "10")"#,
    );
}

#[test]
fn _0524() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: [10] / true)"#,
    );
}

#[test]
fn _0525() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: [10] / 2021-01-01)"#,
    );
}

#[test]
fn _0526() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: [10] / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0527() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: [10] / 10:10:10)"#,
    );
}

#[test]
fn _0528() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: [10] / [10])"#,
    );
}

#[test]
fn _0529() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: [10] / {a: 10})"#,
    );
}

#[test]
fn _0530() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: [10] / P1D)"#,
    );
}

#[test]
fn _0531() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: [10] / P1Y)"#,
    );
}

#[test]
fn _0532() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: [10] / [1..10])"#,
    );
}

#[test]
fn _0533() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: [10] / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0534() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_list_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: [10] / null)"#,
    );
}

#[test]
fn _0535() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 10)"#,
    );
}

#[test]
fn _0536() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / "10")"#,
    );
}

#[test]
fn _0537() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / true)"#,
    );
}

#[test]
fn _0538() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 2021-01-01)"#,
    );
}

#[test]
fn _0539() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0540() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / 10:10:10)"#,
    );
}

#[test]
fn _0541() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / [10])"#,
    );
}

#[test]
fn _0542() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / {a: 10})"#,
    );
}

#[test]
fn _0543() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / P1D)"#,
    );
}

#[test]
fn _0544() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / P1Y)"#,
    );
}

#[test]
fn _0545() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / [1..10])"#,
    );
}

#[test]
fn _0546() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0547() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_context_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: {a: 10} / null)"#,
    );
}

#[test]
fn _0548() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: P1D / "10")"#,
    );
}

#[test]
fn _0549() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: P1D / true)"#,
    );
}

#[test]
fn _0550() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: P1D / 2021-01-01)"#,
    );
}

#[test]
fn _0551() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: P1D / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0552() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: P1D / 10:10:10)"#,
    );
}

#[test]
fn _0553() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: P1D / [10])"#,
    );
}

#[test]
fn _0554() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: P1D / {a: 10})"#,
    );
}

#[test]
fn _0555() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: P1D / P1Y)"#,
    );
}

#[test]
fn _0556() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: P1D / [1..10])"#,
    );
}

#[test]
fn _0557() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: P1D / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0558() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_dtDuration_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: P1D / null)"#,
    );
}

#[test]
fn _0559() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: P1Y / "10")"#,
    );
}

#[test]
fn _0560() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: P1Y / true)"#,
    );
}

#[test]
fn _0561() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: P1Y / 2021-01-01)"#,
    );
}

#[test]
fn _0562() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: P1Y / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0563() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: P1Y / 10:10:10)"#,
    );
}

#[test]
fn _0564() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: P1Y / [10])"#,
    );
}

#[test]
fn _0565() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: P1Y / {a: 10})"#,
    );
}

#[test]
fn _0566() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: P1Y / P1D)"#,
    );
}

#[test]
fn _0567() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: P1Y / [1..10])"#,
    );
}

#[test]
fn _0568() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: P1Y / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0569() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_ymDuration_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: P1Y / null)"#,
    );
}

#[test]
fn _0570() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: [1..10] / 10)"#,
    );
}

#[test]
fn _0571() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: [1..10] / "10")"#,
    );
}

#[test]
fn _0572() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: [1..10] / true)"#,
    );
}

#[test]
fn _0573() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: [1..10] / 2021-01-01)"#,
    );
}

#[test]
fn _0574() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: [1..10] / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0575() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: [1..10] / 10:10:10)"#,
    );
}

#[test]
fn _0576() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: [1..10] / [10])"#,
    );
}

#[test]
fn _0577() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: [1..10] / {a: 10})"#,
    );
}

#[test]
fn _0578() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: [1..10] / P1D)"#,
    );
}

#[test]
fn _0579() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: [1..10] / P1Y)"#,
    );
}

#[test]
fn _0580() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: [1..10] / [1..10])"#,
    );
}

#[test]
fn _0581() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: [1..10] / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0582() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_range_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: [1..10] / null)"#,
    );
}

#[test]
fn _0583() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 10)"#,
    );
}

#[test]
fn _0584() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / "10")"#,
    );
}

#[test]
fn _0585() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / true)"#,
    );
}

#[test]
fn _0586() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 2021-01-01)"#,
    );
}

#[test]
fn _0587() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0588() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / 10:10:10)"#,
    );
}

#[test]
fn _0589() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / [10])"#,
    );
}

#[test]
fn _0590() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / {a: 10})"#,
    );
}

#[test]
fn _0591() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / P1D)"#,
    );
}

#[test]
fn _0592() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / P1Y)"#,
    );
}

#[test]
fn _0593() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / [1..10])"#,
    );
}

#[test]
fn _0594() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0595() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_function_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) / null)"#,
    );
}

#[test]
fn _0596() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_number",
        &CTX,
        r#"null([division] incompatible types: null / 10)"#,
    );
}

#[test]
fn _0597() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_string",
        &CTX,
        r#"null([division] incompatible types: null / "10")"#,
    );
}

#[test]
fn _0598() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_boolean",
        &CTX,
        r#"null([division] incompatible types: null / true)"#,
    );
}

#[test]
fn _0599() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_date",
        &CTX,
        r#"null([division] incompatible types: null / 2021-01-01)"#,
    );
}

#[test]
fn _0600() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_dateAndTime",
        &CTX,
        r#"null([division] incompatible types: null / 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0601() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_time",
        &CTX,
        r#"null([division] incompatible types: null / 10:10:10)"#,
    );
}

#[test]
fn _0602() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_list",
        &CTX,
        r#"null([division] incompatible types: null / [10])"#,
    );
}

#[test]
fn _0603() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_context",
        &CTX,
        r#"null([division] incompatible types: null / {a: 10})"#,
    );
}

#[test]
fn _0604() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_dtDuration",
        &CTX,
        r#"null([division] incompatible types: null / P1D)"#,
    );
}

#[test]
fn _0605() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_ymDuration",
        &CTX,
        r#"null([division] incompatible types: null / P1Y)"#,
    );
}

#[test]
fn _0606() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_range",
        &CTX,
        r#"null([division] incompatible types: null / [1..10])"#,
    );
}

#[test]
fn _0607() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_function",
        &CTX,
        r#"null([division] incompatible types: null / FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0608() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_divide_lhs_null_by_rhs_null",
        &CTX,
        r#"null([division] incompatible types: null / null)"#,
    );
}

#[test]
fn _0609() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_string",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + "10"(string))"#,
    );
}

#[test]
fn _0610() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_boolean",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + true(boolean))"#,
    );
}

#[test]
fn _0611() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_date",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + 2021-01-01(date))"#,
    );
}

#[test]
fn _0612() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_dateAndTime",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + 2021-01-01T10:10:10(date and time))"#,
    );
}

#[test]
fn _0613() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_time",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + 10:10:10(time))"#,
    );
}

#[test]
fn _0614() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_list",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + [10](list<number>))"#,
    );
}

#[test]
fn _0615() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_context",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + {a: 10}(context<a: number>))"#,
    );
}

#[test]
fn _0616() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_dtDuration",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + P1D(days and time duration))"#,
    );
}

#[test]
fn _0617() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_ymDuration",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + P1Y(years and months duration))"#,
    );
}

#[test]
fn _0618() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_range",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + [1..10](range<number>))"#,
    );
}

#[test]
fn _0619() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_function",
        &CTX,
        r#"null(incompatible types in addition: 10(number) + FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any)(function<Any>->Any))"#,
    );
}

#[test]
fn _0620() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_number_to_rhs_null",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0621() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_number",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0622() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_boolean",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0623() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_date",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0624() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_dateAndTime",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0625() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_time",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0626() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_list",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0627() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_context",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0628() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_dtDuration",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0629() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_ymDuration",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0630() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_range",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0631() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_function",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0632() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_string_to_rhs_null",
        &CTX,
        r#"null(expected string as a second argument in addition)"#,
    );
}

#[test]
fn _0633() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0634() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0635() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0636() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0637() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0638() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0639() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0640() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0641() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_dtDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0642() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_ymDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0643() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0644() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0645() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_boolean_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean)"#,
    );
}

#[test]
fn _0646() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is number)"#,
    );
}

#[test]
fn _0647() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is string)"#,
    );
}

#[test]
fn _0648() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is boolean)"#,
    );
}

#[test]
fn _0649() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is date)"#,
    );
}

#[test]
fn _0650() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is date and time)"#,
    );
}

#[test]
fn _0651() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is time)"#,
    );
}

#[test]
fn _0652() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is list<number>)"#,
    );
}

#[test]
fn _0653() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0654() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is range<number>)"#,
    );
}

#[test]
fn _0655() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0656() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_date_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, actual type is Null)"#,
    );
}

#[test]
fn _0657() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is number)"#,
    );
}

#[test]
fn _0658() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is string)"#,
    );
}

#[test]
fn _0659() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is boolean)"#,
    );
}

#[test]
fn _0660() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is date)"#,
    );
}

#[test]
fn _0661() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is date and time)"#,
    );
}

#[test]
fn _0662() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is time)"#,
    );
}

#[test]
fn _0663() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is list<number>)"#,
    );
}

#[test]
fn _0664() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0665() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is range<number>)"#,
    );
}

#[test]
fn _0666() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0667() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dateAndTime_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is Null)"#,
    );
}

#[test]
fn _0668() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is number)"#,
    );
}

#[test]
fn _0669() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is string)"#,
    );
}

#[test]
fn _0670() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is boolean)"#,
    );
}

#[test]
fn _0671() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is date)"#,
    );
}

#[test]
fn _0672() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is date and time)"#,
    );
}

#[test]
fn _0673() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is time)"#,
    );
}

#[test]
fn _0674() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is list<number>)"#,
    );
}

#[test]
fn _0675() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0676() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_ymDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is years and months duration)"#,
    );
}

#[test]
fn _0677() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is range<number>)"#,
    );
}

#[test]
fn _0678() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0679() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_time_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, actual type is Null)"#,
    );
}

#[test]
fn _0680() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0681() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0682() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0683() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0684() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0685() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0686() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0687() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0688() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_dtDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0689() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_ymDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0690() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0691() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0692() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_list_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is list<number>)"#,
    );
}

#[test]
fn _0693() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0694() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0695() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0696() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0697() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0698() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0699() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0700() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0701() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_dtDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0702() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_ymDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0703() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0704() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0705() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_context_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0706() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is number)"#,
    );
}

#[test]
fn _0707() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is string)"#,
    );
}

#[test]
fn _0708() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is boolean)"#,
    );
}

#[test]
fn _0709() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is list<number>)"#,
    );
}

#[test]
fn _0710() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0711() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_ymDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is years and months duration)"#,
    );
}

#[test]
fn _0712() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is range<number>)"#,
    );
}

#[test]
fn _0713() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0714() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_dtDuration_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected days and time duration, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0715() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is number)"#,
    );
}

#[test]
fn _0716() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is string)"#,
    );
}

#[test]
fn _0717() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is boolean)"#,
    );
}

#[test]
fn _0718() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is time)"#,
    );
}

#[test]
fn _0719() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is list<number>)"#,
    );
}

#[test]
fn _0720() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is context<a: number>)"#,
    );
}

#[test]
fn _0721() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_dtDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is days and time duration)"#,
    );
}

#[test]
fn _0722() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is range<number>)"#,
    );
}

#[test]
fn _0723() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0724() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_ymDuration_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected years and months duration, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0725() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0726() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0727() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0728() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0729() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0730() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0731() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0732() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0733() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_dtDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0734() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_ymDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0735() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0736() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0737() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_range_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is range<number>)"#,
    );
}

#[test]
fn _0738() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_number",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0739() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_string",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0740() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_boolean",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0741() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_date",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0742() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_dateAndTime",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0743() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_time",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0744() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_list",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0745() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_context",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0746() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_dtDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0747() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_ymDuration",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0748() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_range",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0749() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_function",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0750() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_function_to_rhs_null",
        &CTX,
        r#"null([builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is function<Any>->Any)"#,
    );
}

#[test]
fn _0751() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_number",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0752() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_string",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0753() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_boolean",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0754() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_date",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0755() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_dateAndTime",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0756() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_time",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0757() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_list",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0758() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_context",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0759() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_dtDuration",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0760() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_ymDuration",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0761() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_range",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0762() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_function",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0763() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_add_lhs_null_to_rhs_null",
        &CTX,
        r#"null"#,
    );
}

#[test]
fn _0764() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - "10")"#,
    );
}

#[test]
fn _0765() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - true)"#,
    );
}

#[test]
fn _0766() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - 2021-01-01)"#,
    );
}

#[test]
fn _0767() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0768() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - 10:10:10)"#,
    );
}

#[test]
fn _0769() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - [10])"#,
    );
}

#[test]
fn _0770() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - {a: 10})"#,
    );
}

#[test]
fn _0771() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - P1D)"#,
    );
}

#[test]
fn _0772() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - P1Y)"#,
    );
}

#[test]
fn _0773() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - [1..10])"#,
    );
}

#[test]
fn _0774() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0775() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_number_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: 10 - null)"#,
    );
}

#[test]
fn _0776() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 10)"#,
    );
}

#[test]
fn _0777() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - "10")"#,
    );
}

#[test]
fn _0778() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - true)"#,
    );
}

#[test]
fn _0779() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 2021-01-01)"#,
    );
}

#[test]
fn _0780() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0781() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - 10:10:10)"#,
    );
}

#[test]
fn _0782() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - [10])"#,
    );
}

#[test]
fn _0783() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - {a: 10})"#,
    );
}

#[test]
fn _0784() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - P1D)"#,
    );
}

#[test]
fn _0785() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - P1Y)"#,
    );
}

#[test]
fn _0786() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - [1..10])"#,
    );
}

#[test]
fn _0787() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0788() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_string_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: "10" - null)"#,
    );
}

#[test]
fn _0789() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: true - 10)"#,
    );
}

#[test]
fn _0790() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: true - "10")"#,
    );
}

#[test]
fn _0791() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: true - true)"#,
    );
}

#[test]
fn _0792() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: true - 2021-01-01)"#,
    );
}

#[test]
fn _0793() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: true - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0794() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: true - 10:10:10)"#,
    );
}

#[test]
fn _0795() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: true - [10])"#,
    );
}

#[test]
fn _0796() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: true - {a: 10})"#,
    );
}

#[test]
fn _0797() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: true - P1D)"#,
    );
}

#[test]
fn _0798() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: true - P1Y)"#,
    );
}

#[test]
fn _0799() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: true - [1..10])"#,
    );
}

#[test]
fn _0800() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: true - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0801() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_boolean_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: true - null)"#,
    );
}

#[test]
fn _0802() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - 10)"#,
    );
}

#[test]
fn _0803() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - "10")"#,
    );
}

#[test]
fn _0804() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - true)"#,
    );
}

#[test]
fn _0805() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - 10:10:10)"#,
    );
}

#[test]
fn _0806() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - [10])"#,
    );
}

#[test]
fn _0807() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - {a: 10})"#,
    );
}

#[test]
fn _0808() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - [1..10])"#,
    );
}

#[test]
fn _0809() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0810() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_date_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01 - null)"#,
    );
}

#[test]
fn _0811() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - 10)"#,
    );
}

#[test]
fn _0812() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - "10")"#,
    );
}

#[test]
fn _0813() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - true)"#,
    );
}

#[test]
fn _0814() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - 10:10:10)"#,
    );
}

#[test]
fn _0815() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - [10])"#,
    );
}

#[test]
fn _0816() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - {a: 10})"#,
    );
}

#[test]
fn _0817() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - [1..10])"#,
    );
}

#[test]
fn _0818() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0819() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dateAndTime_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: 2021-01-01T10:10:10 - null)"#,
    );
}

#[test]
fn _0820() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - 10)"#,
    );
}

#[test]
fn _0821() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - "10")"#,
    );
}

#[test]
fn _0822() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - true)"#,
    );
}

#[test]
fn _0823() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - 2021-01-01)"#,
    );
}

#[test]
fn _0824() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0825() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - [10])"#,
    );
}

#[test]
fn _0826() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - {a: 10})"#,
    );
}

#[test]
fn _0827() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - P1Y)"#,
    );
}

#[test]
fn _0828() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - [1..10])"#,
    );
}

#[test]
fn _0829() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0830() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_time_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: 10:10:10 - null)"#,
    );
}

#[test]
fn _0831() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 10)"#,
    );
}

#[test]
fn _0832() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - "10")"#,
    );
}

#[test]
fn _0833() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - true)"#,
    );
}

#[test]
fn _0834() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 2021-01-01)"#,
    );
}

#[test]
fn _0835() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0836() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - 10:10:10)"#,
    );
}

#[test]
fn _0837() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - [10])"#,
    );
}

#[test]
fn _0838() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - {a: 10})"#,
    );
}

#[test]
fn _0839() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - P1D)"#,
    );
}

#[test]
fn _0840() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - P1Y)"#,
    );
}

#[test]
fn _0841() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - [1..10])"#,
    );
}

#[test]
fn _0842() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0843() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_list_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: [10] - null)"#,
    );
}

#[test]
fn _0844() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 10)"#,
    );
}

#[test]
fn _0845() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - "10")"#,
    );
}

#[test]
fn _0846() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - true)"#,
    );
}

#[test]
fn _0847() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 2021-01-01)"#,
    );
}

#[test]
fn _0848() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0849() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - 10:10:10)"#,
    );
}

#[test]
fn _0850() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - [10])"#,
    );
}

#[test]
fn _0851() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - {a: 10})"#,
    );
}

#[test]
fn _0852() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - P1D)"#,
    );
}

#[test]
fn _0853() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - P1Y)"#,
    );
}

#[test]
fn _0854() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - [1..10])"#,
    );
}

#[test]
fn _0855() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0856() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_context_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: {a: 10} - null)"#,
    );
}

#[test]
fn _0857() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 10)"#,
    );
}

#[test]
fn _0858() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - "10")"#,
    );
}

#[test]
fn _0859() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - true)"#,
    );
}

#[test]
fn _0860() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 2021-01-01)"#,
    );
}

#[test]
fn _0861() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0862() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - 10:10:10)"#,
    );
}

#[test]
fn _0863() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - [10])"#,
    );
}

#[test]
fn _0864() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - {a: 10})"#,
    );
}

#[test]
fn _0865() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - P1Y)"#,
    );
}

#[test]
fn _0866() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - [1..10])"#,
    );
}

#[test]
fn _0867() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0868() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_dtDuration_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: P1D - null)"#,
    );
}

#[test]
fn _0869() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 10)"#,
    );
}

#[test]
fn _0870() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - "10")"#,
    );
}

#[test]
fn _0871() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - true)"#,
    );
}

#[test]
fn _0872() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 2021-01-01)"#,
    );
}

#[test]
fn _0873() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0874() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - 10:10:10)"#,
    );
}

#[test]
fn _0875() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - [10])"#,
    );
}

#[test]
fn _0876() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - {a: 10})"#,
    );
}

#[test]
fn _0877() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - P1D)"#,
    );
}

#[test]
fn _0878() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - [1..10])"#,
    );
}

#[test]
fn _0879() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0880() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_ymDuration_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: P1Y - null)"#,
    );
}

#[test]
fn _0881() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 10)"#,
    );
}

#[test]
fn _0882() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - "10")"#,
    );
}

#[test]
fn _0883() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - true)"#,
    );
}

#[test]
fn _0884() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 2021-01-01)"#,
    );
}

#[test]
fn _0885() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0886() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - 10:10:10)"#,
    );
}

#[test]
fn _0887() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - [10])"#,
    );
}

#[test]
fn _0888() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - {a: 10})"#,
    );
}

#[test]
fn _0889() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - P1D)"#,
    );
}

#[test]
fn _0890() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - P1Y)"#,
    );
}

#[test]
fn _0891() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - [1..10])"#,
    );
}

#[test]
fn _0892() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0893() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_range_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: [1..10] - null)"#,
    );
}

#[test]
fn _0894() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 10)"#,
    );
}

#[test]
fn _0895() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - "10")"#,
    );
}

#[test]
fn _0896() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - true)"#,
    );
}

#[test]
fn _0897() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 2021-01-01)"#,
    );
}

#[test]
fn _0898() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0899() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - 10:10:10)"#,
    );
}

#[test]
fn _0900() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - [10])"#,
    );
}

#[test]
fn _0901() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - {a: 10})"#,
    );
}

#[test]
fn _0902() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - P1D)"#,
    );
}

#[test]
fn _0903() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - P1Y)"#,
    );
}

#[test]
fn _0904() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - [1..10])"#,
    );
}

#[test]
fn _0905() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0906() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_function_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any) - null)"#,
    );
}

#[test]
fn _0907() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_number",
        &CTX,
        r#"null([subtraction] incompatible types: null - 10)"#,
    );
}

#[test]
fn _0908() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_string",
        &CTX,
        r#"null([subtraction] incompatible types: null - "10")"#,
    );
}

#[test]
fn _0909() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_boolean",
        &CTX,
        r#"null([subtraction] incompatible types: null - true)"#,
    );
}

#[test]
fn _0910() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_date",
        &CTX,
        r#"null([subtraction] incompatible types: null - 2021-01-01)"#,
    );
}

#[test]
fn _0911() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_dateAndTime",
        &CTX,
        r#"null([subtraction] incompatible types: null - 2021-01-01T10:10:10)"#,
    );
}

#[test]
fn _0912() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_time",
        &CTX,
        r#"null([subtraction] incompatible types: null - 10:10:10)"#,
    );
}

#[test]
fn _0913() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_list",
        &CTX,
        r#"null([subtraction] incompatible types: null - [10])"#,
    );
}

#[test]
fn _0914() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_context",
        &CTX,
        r#"null([subtraction] incompatible types: null - {a: 10})"#,
    );
}

#[test]
fn _0915() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_dtDuration",
        &CTX,
        r#"null([subtraction] incompatible types: null - P1D)"#,
    );
}

#[test]
fn _0916() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_ymDuration",
        &CTX,
        r#"null([subtraction] incompatible types: null - P1Y)"#,
    );
}

#[test]
fn _0917() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_range",
        &CTX,
        r#"null([subtraction] incompatible types: null - [1..10])"#,
    );
}

#[test]
fn _0918() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_function",
        &CTX,
        r#"null([subtraction] incompatible types: null - FunctionDefinition([(Name("a"), Any)],_,false,[],{},Any))"#,
    );
}

#[test]
fn _0919() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_subtract_lhs_null_minus_rhs_null",
        &CTX,
        r#"null([subtraction] incompatible types: null - null)"#,
    );
}

#[test]
fn _0920() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_string",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0921() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0922() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_date",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0923() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0924() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_time",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0925() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_list",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0926() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_context",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0927() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0928() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0929() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_range",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0930() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_function",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0931() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_number_exp_rhs_null",
        &CTX,
        r#"null(exponentiation exponent is not a number)"#,
    );
}

#[test]
fn _0932() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0933() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0934() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0935() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0936() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0937() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0938() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0939() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0940() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0941() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0942() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0943() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0944() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_string_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0945() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0946() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0947() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0948() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0949() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0950() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0951() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0952() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0953() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0954() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0955() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0956() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0957() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_boolean_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0958() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0959() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0960() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0961() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0962() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0963() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0964() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0965() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0966() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0967() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0968() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0969() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0970() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_date_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0971() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0972() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0973() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0974() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0975() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0976() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0977() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0978() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0979() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0980() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0981() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0982() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0983() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dateAndTime_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0984() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0985() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0986() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0987() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0988() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0989() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0990() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0991() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0992() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0993() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0994() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0995() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0996() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_time_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0997() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0998() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _0999() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1000() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1001() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1002() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1003() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1004() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1005() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1006() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1007() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1008() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1009() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_list_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1010() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1011() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1012() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1013() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1014() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1015() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1016() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1017() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1018() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1019() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1020() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1021() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1022() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_context_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1023() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1024() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1025() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1026() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1027() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1028() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1029() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1030() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1031() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1032() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1033() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1034() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1035() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_dtDuration_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1036() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1037() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1038() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1039() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1040() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1041() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1042() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1043() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1044() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1045() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1046() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1047() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1048() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_ymDuration_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1049() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1050() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1051() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1052() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1053() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1054() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1055() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1056() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1057() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1058() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1059() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1060() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1061() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_range_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1062() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1063() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1064() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1065() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1066() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1067() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1068() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1069() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1070() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1071() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1072() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1073() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1074() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_function_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1075() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_number",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1076() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_string",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1077() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_boolean",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1078() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_date",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1079() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_dateAndTime",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1080() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_time",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1081() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_list",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1082() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_context",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1083() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_dtDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1084() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_ymDuration",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1085() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_range",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1086() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_function",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}

#[test]
fn _1087() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "error_when_exponent_lhs_null_exp_rhs_null",
        &CTX,
        r#"null(exponentiation base is not a number)"#,
    );
}
