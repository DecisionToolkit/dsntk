use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"matches("foobar","^fo*b")"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"matches(input: "foobar", pattern: "^fo*b")"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"matches("abracadabra","bra")"#, true);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"matches("abracadabra","^a.*a$")"#, true);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"matches("abracadabra","^bra")"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"matches("hello\nworld","hello.*world")"#, false);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"matches("hello", "hello", null)"#, true);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"matches(input: "hello", pattern: "hello", flags: null)"#, true);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"matches("hello\nworld","hello.*world","s")"#, true);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"matches(input: "hello\nworld", pattern: "hello.*world", flags: "s")"#, true);
}

#[test]
fn _0011() {
  let scope = &te_scope(
    r#"{poem:"<poem author=\"Wilhelm Busch\">\nKaum hat dies der Hahn gesehen,\nFängt er auch schon an zu krähen:\nKikeriki! Kikikerikih!!\nTak, tak, tak! - da kommen sie.\n</poem>"}"#,
  );
  te_bool(false, scope, r#"matches(poem, "Kaum.*krähen")"#, false);
}

#[test]
fn _0012() {
  te_null(false, &scope!(), r#"matches()"#, "expected 2,3 parameters, actual number of parameters is 0");
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"matches("abc")"#, "expected 2,3 parameters, actual number of parameters is 1");
}

#[test]
fn _0014() {
  te_null(
    false,
    &scope!(),
    r#"matches("abc","a","b","c")"#,
    "expected 2,3 parameters, actual number of parameters is 4",
  );
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#"matches(i: "foobar", pattern: "^fo*b")"#, "parameter 'input' not found");
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"matches(input: "foobar", p: "^fo*b")"#, "parameter 'pattern' not found");
}

#[test]
fn _0017() {
  te_null(false, &scope!(), r#"matches(i: "foobar", pattern: "^fo*b", flags: "s")"#, "parameter 'input' not found");
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#"matches(input: "foobar", p: "^fo*b", flags: "s")"#, "parameter 'pattern' not found");
}

#[test]
fn _0019() {
  te_null(false, &scope!(), r#"matches(input: "foobar", pattern: "^fo*b", f: "s")"#, "parameter 'flags' not found");
}

#[test]
fn _0020() {
  te_null(
    false,
    &scope!(),
    r#"matches(input: "abc", pattern: "a", flags: "x", other: "c")"#,
    "expected 2,3 parameters, actual number of parameters is 4",
  );
}

#[test]
fn _0021() {
  te_null(
    false,
    &scope!(),
    r#"matches(input: 10, pattern: "^fo*b")"#,
    r#"[core::matches] invalid argument type, expected string, actual type is number"#,
  );
}

#[test]
fn _0022() {
  te_null(
    false,
    &scope!(),
    r#"matches(input: "foobar", pattern: true)"#,
    r#"[core::matches] invalid argument type, expected string, actual type is boolean"#,
  );
}

#[test]
fn _0023() {
  te_null(
    false,
    &scope!(),
    r#"matches(input: 10, pattern: "^fo*b", flags: "s")"#,
    r#"[core::matches] invalid argument type, expected string, actual type is number"#,
  );
}

#[test]
fn _0024() {
  te_null(
    false,
    &scope!(),
    r#"matches(input: "foobar", pattern: true, flags: "s")"#,
    r#"[core::matches] invalid argument type, expected string, actual type is boolean"#,
  );
}

#[test]
fn _0025() {
  te_bool(false, &scope!(), r#"matches("Mary\rJones", "Mary.Jones")"#, false);
}

#[test]
fn _0026() {
  te_bool(false, &scope!(), r#"matches("Mary\rJones", "Mary.Jones", "s")"#, true);
}

#[test]
fn _0027() {
  te_bool(false, &scope!(), r#"matches("Mary\nJones", "Mary.Jones")"#, false);
}

#[test]
fn _0028() {
  te_bool(false, &scope!(), r#"matches("Mary\nJones", "Mary.Jones", "s")"#, true);
}

#[test]
fn _0029() {
  te_bool(false, &scope!(), r#"matches("Mary\r\nJones", "Mary.Jones")"#, false);
}

fn _0030() {
  te_bool(false, &scope!(), r#"matches("Mary\r\nJones", "Mary.Jones", "s")"#, true);
}

#[test]
fn _0031() {
  te_bool(false, &scope!(), r#"matches("Mary\u000AJones", "Mary.Jones")"#, false);
}

#[test]
fn _0032() {
  te_bool(false, &scope!(), r#"matches("Mary\u000AJones", "Mary.Jones", "s")"#, true);
}

#[test]
fn _0033() {
  te_bool(false, &scope!(), r#"matches("Mary\u000DJones", "Mary.Jones")"#, false);
}

#[test]
fn _0034() {
  te_bool(false, &scope!(), r#"matches("Mary\u000DJones", "Mary.Jones", "s")"#, true);
}

#[test]
fn _0035() {
  te_bool(false, &scope!(), r#"matches("x", "[A-Z-[OI]]", "i")"#, true);
}

#[test]
fn _0036() {
  te_bool(false, &scope!(), r#"matches("O", "[A-Z-[OI]]", "i")"#, false);
}

#[test]
fn _0037() {
  te_bool(false, &scope!(), r#"matches("O", "[A-Z--O]", "i")"#, false);
}

#[test]
fn _0038() {
  te_null(
    false,
    &scope!(),
    r#"matches("input", "pattern", [])"#,
    "[core::matches] invalid argument type, expected string, actual type is list<Null>",
  );
}

#[test]
fn _0039() {
  te_null(false, &scope!(), r#"matches("input", "pattern", "")"#, "<FeelRegexError> empty regex flags");
}

#[test]
fn _0040() {
  te_bool(false, &scope!(), r#"matches("hello!world", " hello[ ! ]world", "x")"#, true);
}

#[test]
fn _0041() {
  te_bool(false, &scope!(), r#"matches("hello world", " hello[ ]world", "x")"#, true);
}

#[test]
fn _0042() {
  te_bool(false, &scope!(), r#"matches("hello world", "\p{ IsLatin}+", "x")"#, true);
}

#[test]
fn _0043() {
  te_bool(false, &scope!(), r#"matches("hello world", "\p{IsLatin}+")"#, true);
}

#[test]
fn _0044() {
  te_bool(false, &scope!(), r#"matches("hello world", "\p{ I s L a t i n }+", "x")"#, true);
}

#[test]
fn _0045() {
  te_bool(false, &scope!(), r#"matches("abracadabra", "bra", null)"#, true);
}

#[test]
#[ignore]
fn _0046() {
  te_bool(false, &scope!(), r#"matches("hello world", "\p{IsBasicLatin}+")"#, true);
}

#[test]
#[ignore]
fn _0047() {
  te_bool(false, &scope!(), r#"matches("hello world", "\p{ I s B a s i c L a t i n }+", "x")"#, true);
}

#[test]
#[ignore]
fn _0048() {
  te_bool(false, &scope!(), r#"matches("aA", "(a)\1", "i")"#, true);
}

/// `matches` inside a `for` body long enough to take the parallel path.
///
/// The list has 2,000 elements, well past the 512-element parallel threshold. The body
/// passes every static rule, so Rayon workers call `matches` concurrently. The count
/// pins the answer to the serial one. This guards the compiled-pattern cache, which is
/// per-thread: every worker must compile and match independently and still agree with
/// a single-threaded run.
#[test]
fn _0049() {
  te_number(
    false,
    &scope!(),
    r#"count((for s in (for i in 1..2000 return string(i)) return matches(s, "^1\\d{2}$"))[item = true])"#,
    100,
    0,
  );
}
