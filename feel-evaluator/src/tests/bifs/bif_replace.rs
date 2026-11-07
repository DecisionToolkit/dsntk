use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_string(
        false,
        &scope!(),
        r#"replace("abcd","(ab)|(a)","[1=$1][2=$2]")"#,
        "[1=ab][2=]cd",
    );
}

#[test]
fn _0002() {
    te_string(false, &scope!(), "replace(\"a\",\"[b-z]\",\"\")", "a");
}

#[test]
fn _0003() {
    te_string(false, &scope!(), "replace(\"a\",\"[a-z]\",\"#\")", "#");
}

#[test]
fn _0004() {
    te_string(false, &scope!(), "replace(\"abc\",\"def\",\"#\")", "abc");
}

#[test]
fn _0005() {
    te_string(false, &scope!(), "replace(\"abc\",\"e\",\"#\")", "abc");
}

#[test]
fn _0006() {
    te_string(
        false,
        &scope!(),
        "replace(\"foobar\",\"^fo*b*\",\"#\")",
        "#ar",
    );
}

#[test]
fn _0007() {
    te_string(
        false,
        &scope!(),
        "replace(\"abc\",\".^[d-z]\",\"#\")",
        "abc",
    );
}

#[test]
fn _0008() {
    te_string(
        false,
        &scope!(),
        r#"replace("abracadabra","bra","*")"#,
        "a*cada*",
    );
}

#[test]
fn _0009() {
    te_string(
        false,
        &scope!(),
        r#"replace("abracadabra","a.*a","*")"#,
        "*",
    );
}

#[test]
fn _0010() {
    te_string(
        false,
        &scope!(),
        r#"replace("abracadabra","a.*?a","*")"#,
        "*c*bra",
    );
}

#[test]
fn _0011() {
    te_string(
        false,
        &scope!(),
        r#"replace("abracadabra","a","")"#,
        "brcdbr",
    );
}

#[test]
fn _0012() {
    te_string(false, &scope!(), r#"replace("AAAA","A+","b")"#, "b");
}

#[test]
fn _0013() {
    te_string(false, &scope!(), r#"replace("AAAA","A+?","b")"#, "bbbb");
}

#[test]
fn _0014() {
    te_string(
        false,
        &scope!(),
        r#"replace("darted","^(.*?)d(.*)$","$1$2")"#,
        "arted",
    );
}

#[test]
fn _0015() {
    te_string(
        false,
        &scope!(),
        r#"replace("darted","^(.*?)d(.*)$","$1c$2")"#,
        "carted",
    );
}

#[test]
fn _0016() {
    te_string(
        false,
        &scope!(),
        r#"replace("reluctant","r.*?t","X")"#,
        r#"Xant"#,
    );
}

#[test]
fn _0017() {
    te_string(
        false,
        &scope!(),
        r#"replace("0123456789","(\d{3})(\d{3})(\d{4})","($1) $2-$3")"#,
        r#"(012) 345-6789"#,
    );
}

#[test]
fn _0018() {
    te_string(
        false,
        &scope!(),
        "replace(\"abc\",\"[a-z]\",\"#\",\"\")",
        "###",
    );
}

#[test]
fn _0019() {
    te_string(
        false,
        &scope!(),
        "replace(\"a.b.c.\",\"[a-z]\",\"#\",\"s\")",
        "#.#.#.",
    );
}

#[test]
fn _0020() {
    te_string(
        false,
        &scope!(),
        "replace(\"abc\",\"[A-Z]\",\"#\",\"i\")",
        "###",
    );
}

#[test]
fn _0021() {
    te_string(
        false,
        &scope!(),
        "replace(\"abc\",\"[a-z]\",\"#\",\"s\")",
        "###",
    );
}

#[test]
fn _0022() {
    te_string(
        false,
        &scope!(),
        "replace(\"a b c d \",\"[a-z]\",\"#\",\"x\")",
        "# # # # ",
    );
}

#[test]
fn _0023() {
    te_string(
        false,
        &scope!(),
        "replace(\"a b c d \",\"[a-z]\",\"#\")",
        "# # # # ",
    );
}

#[test]
fn _0024() {
    te_string(
        false,
        &scope!(),
        r#"replace("abc",".^[d-z]*","smix")"#,
        "abc",
    );
}

#[test]
fn _0025() {
    te_string(
        false,
        &scope!(),
        "replace(input:\"abc\",pattern:\"[a-z]\",replacement:\"#\")",
        "###",
    );
}

#[test]
fn _0026() {
    te_string(
        false,
        &scope!(),
        "replace(input:\"abc\",pattern:\"[A-Z]\",replacement:\"#\",flags:\"\")",
        "abc",
    );
}

#[test]
fn _0027() {
    te_string(
        false,
        &scope!(),
        "replace(input:\"abc\",pattern:\"[A-Z]\",replacement:\"#\",flags:\"i\")",
        "###",
    );
}

#[test]
fn _0028() {
    te_string(
        false,
        &scope!(),
        "replace(input:\"abc\",pattern:\".^[d-z]*\",replacement:\"#\",flags:\"smix\")",
        "abc",
    );
}

#[test]
fn _0029() {
    te_string(
        false,
        &scope!(),
        r#"replace("a\b\c","\\","\\\\","q")"#,
        r"a\\b\\c",
    );
}

#[test]
fn _0030() {
    te_string(false, &scope!(), r#"replace("a/b/c","/","$","q")"#, "a$b$c");
}

#[test]
fn _0031() {
    te_string(
        false,
        &scope!(),
        "replace(\"abc\",\"[A-Z]\",\"#\",\"all unknown but i\")",
        "###",
    );
}

#[test]
fn _0032() {
    te_string(
        false,
        &scope!(),
        r#"replace(replace("anbncnz","n","\u000A"),"[A-C]\n","u","i")"#,
        "uuuz",
    );
}

#[test]
fn _0033() {
    te_string(
        false,
        &scope!(),
        r#"replace("a\u000Ab\u000Ac\u000A","[A-Z]\n","u","i")"#,
        "uuu",
    );
}

#[test]
fn _0034() {
    te_null(
        false,
        &scope!(),
        r#"replace()"#,
        r#"expected 3,4 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0035() {
    te_null(
        false,
        &scope!(),
        r#"replace("abc")"#,
        r#"expected 3,4 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0036() {
    te_null(
        false,
        &scope!(),
        r#"replace("abc","b")"#,
        r#"expected 3,4 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0037() {
    te_null(
        false,
        &scope!(),
        r#"replace("abc","b","c","d","e")"#,
        r#"expected 3,4 parameters, actual number of parameters is 5"#,
    );
}

#[test]
fn _0038() {
    te_null(
        false,
        &scope!(),
        "replace(i:\"abc\",pattern:\".^[d-z]*\",replacement:\"#\",flags:\"smix\")",
        r#"parameter 'input' not found"#,
    );
}

#[test]
fn _0039() {
    te_null(
        false,
        &scope!(),
        "replace(input:\"abc\",p:\".^[d-z]*\",replacement:\"#\",flags:\"smix\")",
        r#"parameter 'pattern' not found"#,
    );
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        "replace(input: \"abc\", pattern: \".^[d-z]*\", r:\"\", flags: \"smix\")",
        r#"parameter 'replacement' not found"#,
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        r#"replace(10,"[a-z]","A")"#,
        r#"replace: input must be a string"#,
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        r#"replace("abc1",10,"A")"#,
        r#"replace: pattern must be a string"#,
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        r#"replace("abc1","[a-z]",10)"#,
        r#"replace: replacement must be a string"#,
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        r#"replace("abc1","[a-z","A")"#,
        r#"replace: invalid pattern"#,
    );
}
