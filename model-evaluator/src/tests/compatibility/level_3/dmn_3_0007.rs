use super::*;

from_examples!(DMN_3_0007);

const INPUT_DATA: &str = r#"{Day: 22, Hours: 12, Minutes: 59, Month: 11, Seconds: 1.3, Timezone: @"-PT1H", Year: 1999, dateString: "2015-12-24", dateTimeString: "2016-12-24T23:59:00-08:00", durationString: "P13DT2H14S", oneHour: PT1H, timeString: "00:00:01-01:00"}"#;

#[test]
fn _0001() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Date-Time",
        &ctx,
        r#"2016-12-24T23:59:00-08:00"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Date",
        &ctx,
        r#"{fromDateTime: 2016-12-24, fromString: 2015-12-24, fromYearMonthDay: 1999-11-22}"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Time",
        &ctx,
        r#"00:00:01-01:00"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Date-Time2",
        &ctx,
        r#"2015-12-24T00:00:01-01:00"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Time2",
        &ctx,
        r#"00:00:01-01:00"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Time3",
        &ctx,
        r#"12:59:01.3-01:00"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "dtDuration1",
        &ctx,
        r#"P13DT2H14S"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "dtDuration2",
        &ctx,
        r#"P367DT6H58M59S"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "sumDurations",
        &ctx,
        r#"P380DT8H59M13S"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "ymDuration2",
        &ctx,
        r#"P1Y"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "cDay",
        &ctx,
        r#"24"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "cYear",
        &ctx,
        r#"2015"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "cMonth",
        &ctx,
        r#"12"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "cHour",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "cMinute",
        &ctx,
        r#"0"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "cSecond",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "cOffset",
        &ctx,
        r#"-PT1H"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "years",
        &ctx,
        r#"1"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "seconds",
        &ctx,
        r#"14"#,
    );
}
