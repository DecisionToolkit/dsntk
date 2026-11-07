use super::*;

from_examples!(DMN_3_0036);
static CTX1: LazyLock<FeelContext> = LazyLock::new(|| {
    context(
        r#"{"Another Date": @"2018-07-31", "Another Date and Time": @"2018-07-31T17:13:00Z", "Another Days and Time Duration": @"PT12H", "Another String": "Hello", "Another Time": @"17:13:00", "Another Years and Months Duration": @"P8M", "Another boolean": false, "Another number": 15, Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#,
    )
});
static CTX2: LazyLock<FeelContext> = LazyLock::new(|| {
    context(
        r#"{"Another Date": @"2018-07-29", "Another Date and Time": @"2018-07-29T15:13:00Z", "Another Days and Time Duration": @"PT8H",  "Another String": "Hello", "Another Time": @"15:13:00", "Another Years and Months Duration": @"P3M", "Another boolean": false, "Another number": 5,  Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#,
    )
});
static CTX3: LazyLock<FeelContext> = LazyLock::new(|| {
    context(
        r#"{"Another Date": @"2018-07-30", "Another Date and Time": @"2018-07-30T16:12:00Z", "Another Days and Time Duration": @"PT10H", "Another String": "Hi",    "Another Time": @"16:11:00", "Another Years and Months Duration": @"P5M", "Another boolean": true,  "Another number": 10, Complex: { aBoolean: true, aDate: @"2018-07-30", aDateTime: @"2018-07-30T16:12:00Z", aDaysAndTimeDuration: @"PT10H", aNumber: 10, aString: "Hi", aTime: @"16:11:00", aYearsAndMonthsDuration: @"P5M"}}"#,
    )
});

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable_name = "Compare String";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Different String""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable_name = "Compare Date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Future Date""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable_name = "Compare Number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Bigger""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let invocable_name = "Compare Date and Time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Future date time""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable_name = "Compare Days and Time Duration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Longer duration""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable_name = "Compare Years and Months Duration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Longer duration""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable_name = "Compare Time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Future Time""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    let invocable_name = "Compare Boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX1,
        r#""Not same boolean""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX1)
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    let invocable_name = "Compare String";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Different String""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    let invocable_name = "Compare Date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Past Date""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    let invocable_name = "Compare Number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Smaller""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    let invocable_name = "Compare Date and Time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Past date time""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    let invocable_name = "Compare Days and Time Duration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Shorter duration""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    let invocable_name = "Compare Years and Months Duration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Shorter duration""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0015(b: &mut Bencher) {
    let invocable_name = "Compare Time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Past Time""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    let invocable_name = "Compare Boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX2,
        r#""Not same boolean""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX2)
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    let invocable_name = "Compare String";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Same String""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    let invocable_name = "Compare Date";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Same Date""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    let invocable_name = "Compare Number";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Equals""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}

#[bench]
fn _0020(b: &mut Bencher) {
    let invocable_name = "Compare Date and Time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Same date time""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}

#[bench]
fn _0021(b: &mut Bencher) {
    let invocable_name = "Compare Days and Time Duration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Same duration""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}

#[bench]
fn _0022(b: &mut Bencher) {
    let invocable_name = "Compare Years and Months Duration";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Same duration""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}

#[bench]
fn _0023(b: &mut Bencher) {
    let invocable_name = "Compare Time";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Same Time""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}

#[bench]
fn _0024(b: &mut Bencher) {
    let invocable_name = "Compare Boolean";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &CTX3,
        r#""Same boolean""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &CTX3)
    );
}
