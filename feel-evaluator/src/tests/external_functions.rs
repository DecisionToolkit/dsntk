use super::*;

#[test]
fn _0001() {
    let scope = te_scope(
        r#"{ cos: function(x) external { java: { class: "java.lang.Math",method signature: "cos(double)" } } }"#,
    );
    te_number(false, &scope, "cos(123)", -8879689066918555, 16);
}

#[test]
fn _0002() {
    let scope = te_scope(
        r#"{ cos: function(x) external { java: { class: "java.lang.Math",method signature: "cos(double)" } } }"#,
    );
    te_null(false, &scope, "cos(1,2)", r#"invalid number of arguments"#);
}

#[test]
fn _0003() {
    let scope = te_scope(
        r#"{ cos: function(arg: number) external { java: { class: "java.lang.Math",method signature: "cos(double)" } } }"#,
    );
    te_number(false, &scope, "cos(arg: 123)", -8879689066918555, 16);
}

#[test]
fn _0004() {
    let scope = te_scope(
        r#"{ cos: function(arg: number) external { java: { class: "java.lang.Math",method signature: "cos(double)" } } }"#,
    );
    te_null(
        false,
        &scope,
        "cos(x: 123)",
        "parameter with name arg not found in arguments",
    );
}

#[test]
fn _0005() {
    let scope = te_scope(
        r#"{ cos: function(arg: number) external { java: { class: "java.lang.Math",method signature: "cos(double)" } } }"#,
    );
    te_null(
        false,
        &scope,
        "cos(x: 1, y: 2)",
        r#"invalid number of arguments"#,
    );
}

#[test]
fn _0006() {
    let scope = te_scope(
        r#"{ mathFoo: function(x) external { java: { class: "java.lang.Math",method signature: "foo(double)" } } }"#,
    );
    te_null(
        false,
        &scope,
        "mathFoo(123)",
        "java.lang.NoSuchMethodException: java.lang.Math.foo(double)",
    );
}

#[test]
fn _0007() {
    let scope = te_scope(
        r#"{ Weather on Ῥόδος: function() external { pmml:{document: "https://en.wiktionary.org/wiki/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82", model: "weather"}} }"#,
    );
    te_string(
        false,
        &scope,
        "Weather on Ῥόδος()",
        r#"PMML, document = https://en.wiktionary.org/wiki/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82, model name = weather"#,
    );
}

#[test]
fn _0008() {
    let scope = te_scope(
        r#"{ run: function() external { java: { class: "io.dsntk.Runner", method signature: "run()"}} }"#,
    );
    te_null(
        false,
        &scope,
        "run()",
        "java.lang.ClassNotFoundException: io.dsntk.Runner",
    );
}

#[test]
fn _0009() {
    let scope = te_scope(
        r#"{ Weather on Ῥόδος: function() external { pmml:{document: "", model: "weather"}} }"#,
    );
    te_null(
        false,
        &scope,
        "Weather on Ῥόδος()",
        r#"PMML document not specified"#,
    );
}

#[test]
fn _0010() {
    let scope = te_scope(
        r#"{ Weather on Ῥόδος: function() external { pmml:{document: "https://en.wiktionary.org/wiki/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82", model: ""}} }"#,
    );
    te_null(
        false,
        &scope,
        "Weather on Ῥόδος()",
        r#"PMML model name not specified"#,
    );
}

#[test]
fn _0011() {
    let scope = te_scope(r#"{ fair value: function() external { rust:{ fun: ""} } }"#);
    te_null(
        false,
        &scope,
        "fair value()",
        r#"expected built-in function name or function definition, actual is null(invalid body in function definition)"#,
    );
}

#[test]
fn _0012() {
    let scope = te_scope(
        r#"{ valueOf: function(c) external { java: { class: "java.lang.String", method signature: "valueOf(double)" } } }"#,
    );
    te_string(false, &scope, "valueOf(1)", "1.0");
}

#[test]
fn _0013() {
    let scope = te_scope(
        r#"{ valueOf: function(c) external { java: { class: "java.lang.String", method signature: "valueOf(char)" } } }"#,
    );
    te_null(
        false,
        &scope,
        "valueOf(1)",
        "simple DTO conversion to object failed, class: char, type: XSD_DECIMAL",
    );
}

#[test]
fn _0014() {
    let scope = te_scope(
        r#"{ valueOf: function(n) external { java: { class:"java.lang.Integer", method signature: "valueOf(java.lang.String)" } } }"#,
    );
    te_null(
        false,
        &scope,
        "valueOf(1)",
        "simple DTO conversion to object failed, class: java.lang.String, type: XSD_DECIMAL",
    );
}

#[test]
fn _0015() {
    let scope = te_scope(
        r#"{ valueOf: function(n) external { java: { class:"java.lang.Integer", method signature: "valueOf(int)" } } }"#,
    );
    te_number(false, &scope, "valueOf(1)", 1, 0);
}

#[test]
fn _0016() {
    let scope = te_scope(
        r#"{ valueOf: function(n) external { java: { class:"java.lang.Integer", method signature: "valueOf(java.lang.String)" } } }"#,
    );
    te_null(
        false,
        &scope,
        r#"valueOf("394857340958730495873204598374503")"#,
        r#"java.lang.NumberFormatException: For input string: "394857340958730495873204598374503""#,
    );
}

#[test]
fn _0017() {
    let scope = te_scope(
        r#"{ valueOf: function(n) external { java: { class: "java.lang.Float", method signature: "valueOf(java.lang.String)" } } }"#,
    );
    te_number(false, &scope, r#"valueOf("1.2345")"#, 12345, 4);
}

#[test]
fn _0018() {
    let scope = te_scope(
        r#"{ valueOf: function(n) external { java: { class:"java.lang.Float", method signature: "valueOf(java.lang.String)" } } }"#,
    );
    te_null(
        false,
        &scope,
        r#"valueOf(1)"#,
        "simple DTO conversion to object failed, class: java.lang.String, type: XSD_DECIMAL",
    );
}

#[test]
fn _0019() {
    let scope = te_scope(
        r#"{ valueOf: function(n) external { java: { class: "java.lang.Float", method signature: "valueOf(java.lang.String)" } } }"#,
    );
    te_null(
        false,
        &scope,
        r#"valueOf("99,9")"#,
        r#"java.lang.NumberFormatException: For input string: "99,9""#,
    );
}

#[test]
fn _0020() {
    let scope =
        te_scope(r#"{ valueOf: function(n) external { java: { class: "java.lang.Float" } } }"#);
    te_null(
        false,
        &scope,
        r#"valueOf("99,9")"#,
        "expected built-in function name or function definition, actual is null(invalid body in function definition)",
    );
}

#[test]
fn _0021() {
    let scope = te_scope(
        r#"{ valueOf: function(n) external { java: { method signature: "valueOf(java.lang.String)" } } }"#,
    );
    te_null(
        false,
        &scope,
        r#"valueOf("99,9")"#,
        "expected built-in function name or function definition, actual is null(invalid body in function definition)",
    );
}

#[test]
fn _0022() {
    let scope =
        te_scope(r#"{ valueOf: function(n) external { pmml: { document: "document-name" } } }"#);
    te_null(
        false,
        &scope,
        r#"valueOf("99,9")"#,
        "expected built-in function name or function definition, actual is null(invalid body in function definition)",
    );
}

#[test]
fn _0023() {
    let scope = te_scope(r#"{ valueOf: function(n) external { pmml: { model: "model-name" } } }"#);
    te_null(
        false,
        &scope,
        r#"valueOf("99,9")"#,
        "expected built-in function name or function definition, actual is null(invalid body in function definition)",
    );
}

#[test]
fn _0024() {
    let scope = te_scope(r#"{ valueOf: function(n) external 20 }"#);
    te_null(
        false,
        &scope,
        r#"valueOf("99,9")"#,
        "expected built-in function name or function definition, actual is null(invalid body in function definition)",
    );
}

#[test]
fn _0025() {
    let scope = te_scope(
        r#"{ format: function(s1, n1) external { java: { class: "java.lang.String", method signature: "format(java.lang.String, [Ljava.lang.Object;)" } } }"#,
    );
    te_string(false, &scope, r#"format("foo %s", "bar")"#, r#"foo bar"#);
}
