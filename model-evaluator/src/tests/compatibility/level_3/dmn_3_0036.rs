use super::*;

from_examples!(DMN_3_0036);

static_context!(
    CTX1,
    r#"{"Another Date": @"2018-07-31", "Another Date and Time": @"2018-07-31T17:13:00Z", "Another Days and Time Duration": @"PT12H", "Another String": "Hello", "Another Time": @"17:13:00", "Another Years and Months Duration": @"P8M", "Another boolean": false, "Another number": 15, Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#
);

static_context!(
    CTX2,
    r#"{"Another Date": @"2018-07-29", "Another Date and Time": @"2018-07-29T15:13:00Z", "Another Days and Time Duration": @"PT8H",  "Another String": "Hello", "Another Time": @"15:13:00", "Another Years and Months Duration": @"P3M", "Another boolean": false, "Another number": 5,  Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#
);

static_context!(
    CTX3,
    r#"{"Another Date": @"2018-07-30", "Another Date and Time": @"2018-07-30T16:12:00Z", "Another Days and Time Duration": @"PT10H", "Another String": "Hi",    "Another Time": @"16:11:00", "Another Years and Months Duration": @"P5M", "Another boolean": true,  "Another number": 10, Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#
);

#[test]
fn _0001() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare String",
        &CTX1,
        r#""Different String""#,
    );
}

#[test]
fn _0002() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Date",
        &CTX1,
        r#""Future Date""#,
    );
}

#[test]
fn _0003() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Number",
        &CTX1,
        r#""Bigger""#,
    );
}

#[test]
fn _0004() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Date and Time",
        &CTX1,
        r#""Future date time""#,
    );
}

#[test]
fn _0005() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Days and Time Duration",
        &CTX1,
        r#""Longer duration""#,
    );
}

#[test]
fn _0006() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Years and Months Duration",
        &CTX1,
        r#""Longer duration""#,
    );
}

#[test]
fn _0007() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Time",
        &CTX1,
        r#""Future Time""#,
    );
}

#[test]
fn _0008() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Boolean",
        &CTX1,
        r#""Not same boolean""#,
    );
}

#[test]
fn _0009() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare String",
        &CTX2,
        r#""Different String""#,
    );
}

#[test]
fn _0010() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Date",
        &CTX2,
        r#""Past Date""#,
    );
}

#[test]
fn _0011() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Number",
        &CTX2,
        r#""Smaller""#,
    );
}

#[test]
fn _0012() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Date and Time",
        &CTX2,
        r#""Past date time""#,
    );
}

#[test]
fn _0013() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Days and Time Duration",
        &CTX2,
        r#""Shorter duration""#,
    );
}

#[test]
fn _0014() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Years and Months Duration",
        &CTX2,
        r#""Shorter duration""#,
    );
}

#[test]
fn _0015() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Time",
        &CTX2,
        r#""Past Time""#,
    );
}

#[test]
fn _0016() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Boolean",
        &CTX2,
        r#""Not same boolean""#,
    );
}

#[test]
fn _0017() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare String",
        &CTX3,
        r#""Same String""#,
    );
}

#[test]
fn _0018() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Date",
        &CTX3,
        r#""Same Date""#,
    );
}

#[test]
fn _0019() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Number",
        &CTX3,
        r#""Equals""#,
    );
}

#[test]
fn _0020() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Date and Time",
        &CTX3,
        r#""Same date time""#,
    );
}

#[test]
fn _0021() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Days and Time Duration",
        &CTX3,
        r#""Same duration""#,
    );
}

#[test]
fn _0022() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Years and Months Duration",
        &CTX3,
        r#""Same duration""#,
    );
}

#[test]
fn _0023() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Time",
        &CTX3,
        r#""Same Time""#,
    );
}

#[test]
fn _0024() {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Compare Boolean",
        &CTX3,
        r#""Same boolean""#,
    );
}
