use super::*;

from_examples!(DMN_3_0035);

#[test]
fn _0001() {
    let ctx = context(r#"{B Value: 83,G Value: 65,R Value: 0}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Profile of Color",
        &ctx,
        r##"{CMYK notation: {C: 100, K: 67, M: 22, Y: 0}, Hex notation: "#004153", RGB notation: {B: 83, G: 65, R: 0}}"##,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{B Value: 0,G Value: 0,R Value: 0}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Profile of Color",
        &ctx,
        r##"{CMYK notation: {C: 0, K: 100, M: 0, Y: 0}, Hex notation: "#000000", RGB notation: {B: 0, G: 0, R: 0}}"##,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{B Value: 0,G Value: 0,R Value: 204}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Profile of Color",
        &ctx,
        r##"{CMYK notation: {C: 0, K: 20, M: 100, Y: 100}, Hex notation: "#CC0000", RGB notation: {B: 0, G: 0, R: 204}}"##,
    );
}
