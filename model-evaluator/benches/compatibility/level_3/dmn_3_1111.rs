use super::*;

from_examples!(DMN_3_1111);

static_context!(CTX, "{}");

fn eq(b: &mut Bencher, invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, ctx)
    );
}

#[bench]
fn _0001(b: &mut Bencher) {
    let invocable = "decision001";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is Null)";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0002(b: &mut Bencher) {
    let invocable = "decision002";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is Null)";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0003(b: &mut Bencher) {
    let invocable = "decision003";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}
#[bench]
fn _0004(b: &mut Bencher) {
    let invocable = "fn-matches2args-1";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0005(b: &mut Bencher) {
    let invocable = "fn-matchesErr-1";
    let expected = "null(<FeelRegexError> invalid regex flag: 'p')";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0006(b: &mut Bencher) {
    let invocable = "fn-matches-27";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0007(b: &mut Bencher) {
    let invocable = "fn-matches-28";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
#[ignore]
fn _0008(b: &mut Bencher) {
    let invocable = "fn-matches-32";
    let expected = "?";
    eq(b, invocable, &CTX, expected);
}

#[bench]
#[ignore]
fn _0009(b: &mut Bencher) {
    let invocable = "fn-matches-33";
    let expected = "?";
    eq(b, invocable, &CTX, expected);
}

#[bench]
#[ignore]
fn _0010(b: &mut Bencher) {
    let invocable = "fn-matches-34";
    let expected = "?";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0011(b: &mut Bencher) {
    let invocable = "fn-matches-45";
    let expected = "false";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0012(b: &mut Bencher) {
    let invocable = "caselessmatch01";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0013(b: &mut Bencher) {
    let invocable = "caselessmatch02";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0014(b: &mut Bencher) {
    let invocable = "caselessmatch03";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0015(b: &mut Bencher) {
    let invocable = "caselessmatch07";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0016(b: &mut Bencher) {
    let invocable = "caselessmatch08";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0017(b: &mut Bencher) {
    let invocable = "caselessmatch09";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0018(b: &mut Bencher) {
    let invocable = "caselessmatch10";
    let expected = "false";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0019(b: &mut Bencher) {
    let invocable = "caselessmatch11";
    let expected = "false";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0020(b: &mut Bencher) {
    let invocable = "caselessmatch12";
    let expected = "false";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0021(b: &mut Bencher) {
    let invocable = "caselessmatch13";
    let expected = "false";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0022(b: &mut Bencher) {
    let invocable = "K-MatchesFunc-1";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is list<Null>)";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0023(b: &mut Bencher) {
    let invocable = "K-MatchesFunc-2";
    let expected = "null(expected 2,3 parameters, actual number of parameters is 1)";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0024(b: &mut Bencher) {
    let invocable = "K-MatchesFunc-3";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is list<Null>)";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0025(b: &mut Bencher) {
    let invocable = "K-MatchesFunc-4";
    let expected = "null(expected 2,3 parameters, actual number of parameters is 4)";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0026(b: &mut Bencher) {
    let invocable = "K-MatchesFunc-5";
    let expected = "null(<FeelRegexError> empty regex flags)";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0027(b: &mut Bencher) {
    let invocable = "K-MatchesFunc-6";
    let expected = "null(<FeelRegexError> invalid regex flag: 'X')";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0028(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-1";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0029(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-2";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0030(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-3";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0031(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-4";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0032(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-5";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0033(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-6";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0034(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-7";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '\p{ IsBasicLatin}+')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0035(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-8";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(.)\3')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0036(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-9";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(.)\2')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0037(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-10";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '\3')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0038(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-11";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(asd)[\1]')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0039(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-12";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(asd)[asd\1]')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0040(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-13";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(asd)[asd\0]')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0041(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-14";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '1[asd\0]')"#;
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0042(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-15";
    let expected = "[false, true]";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0043(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-16";
    let expected = "false";
    eq(b, invocable, &CTX, expected);
}

#[bench]
fn _0044(b: &mut Bencher) {
    let invocable = "K2-MatchesFunc-17";
    let expected = "true";
    eq(b, invocable, &CTX, expected);
}
