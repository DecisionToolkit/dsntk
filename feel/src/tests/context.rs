use crate::context::FeelContext;
use crate::names::Name;
use crate::qualified_names::QualifiedName;
use crate::values::{VALUE_FALSE, VALUE_TRUE, Value};
use crate::{ToFeelString, value_number};
use dsntk_common::Jsonify;
use std::slice;

#[test]
fn test_context_default() {
  let ctx: FeelContext = Default::default();
  assert_eq!("{}", ctx.to_string());
  assert_eq!("FeelContext({})", format!("{ctx:?}"));
  assert_eq!(None, ctx.get_first());
  assert_eq!(0, ctx.len());
  assert!(ctx.is_empty());
}

#[test]
fn test_context_to_string() {
  let name_a = Name::from("a");
  let name_x_y = Name::new(&["x", "y"]);
  let name_k_plus_l_minus_m = Name::new(&["k", "+", "l", "-", "m"]);
  let mut ctx: FeelContext = Default::default();
  ctx.set_entry(&name_a, value_number!(10));
  assert_eq!(r#"{a: 10}"#, ctx.to_string());
  ctx.set_entry(&name_x_y, Value::Boolean(true));
  assert_eq!(r#"{a: 10, x y: true}"#, ctx.to_string());
  ctx.set_entry(&name_k_plus_l_minus_m, Value::String("KLM".to_string()));
  assert_eq!(r#"{a: 10, k+l-m: "KLM", x y: true}"#, ctx.to_string());
}

#[test]
fn test_context_to_feel_string() {
  let name_a = Name::from("a");
  let name_x_y = Name::new(&["x", "y"]);
  let name_k_plus_l_minus_m = Name::new(&["k", "+", "l", "-", "m"]);
  let mut ctx: FeelContext = Default::default();
  ctx.set_entry(&name_a, value_number!(10));
  assert_eq!(r#"{a: 10}"#, ctx.to_feel_string());
  ctx.set_entry(&name_x_y, Value::Boolean(true));
  assert_eq!(r#"{a: 10, x y: true}"#, ctx.to_feel_string());
  ctx.set_entry(&name_k_plus_l_minus_m, Value::String("KLM".to_string()));
  assert_eq!(r#"{a: 10, k+l-m: "KLM", x y: true}"#, ctx.to_feel_string());
  let mut ctx: FeelContext = Default::default();
  let name_left_bracket = Name::from("{");
  ctx.set_entry(&name_left_bracket, value_number!(1));
  assert_eq!(r#"{"{": 1}"#, ctx.to_feel_string());
  let name_right_bracket = Name::from("}");
  ctx.set_entry(&name_right_bracket, value_number!(2));
  assert_eq!(r#"{"{": 1, "}": 2}"#, ctx.to_feel_string());
  let name_colon = Name::from(":");
  ctx.set_entry(&name_colon, value_number!(3));
  assert_eq!(r#"{":": 3, "{": 1, "}": 2}"#, ctx.to_feel_string());
  let name_comma = Name::from(",");
  ctx.set_entry(&name_comma, value_number!(4));
  assert_eq!(r#"{",": 4, ":": 3, "{": 1, "}": 2}"#, ctx.to_feel_string());
  let name_double_quote = Name::from("\"");
  ctx.set_entry(&name_double_quote, value_number!(5));
  assert_eq!(r#"{"\"": 5, ",": 4, ":": 3, "{": 1, "}": 2}"#, ctx.to_feel_string());
}

#[test]
fn test_context_to_json() {
  let name_a = Name::from("a");
  let name_x_y = Name::new(&["x", "y"]);
  let name_k_plus_l_minus_m = Name::new(&["k", "+", "l", "-", "m"]);
  let mut ctx: FeelContext = Default::default();
  ctx.set_entry(&name_a, value_number!(10));
  assert_eq!(r#"{"a": 10}"#, ctx.jsonify());
  ctx.set_entry(&name_x_y, Value::Boolean(true));
  assert_eq!(r#"{"a": 10, "x y": true}"#, ctx.jsonify());
  ctx.set_entry(&name_k_plus_l_minus_m, Value::String("KLM".to_string()));
  assert_eq!(r#"{"a": 10, "k+l-m": "KLM", "x y": true}"#, ctx.jsonify());
}

#[test]
fn test_context_one_level() {
  let name_a = Name::from("a");
  let name_a_b = Name::new(&["a", "b"]);
  let name_a_b_c = Name::new(&["a", "b", "c"]);
  let qname_a = QualifiedName::new(&[&name_a]);
  let qname_a_b = QualifiedName::new(&[&name_a_b]);
  let qname_a_b_c = QualifiedName::new(&[&name_a_b_c]);
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(10));
  assert_eq!("{a: 10}", ctx_a.to_string());
  assert!(ctx_a.contains_entry(&name_a));
  assert!(ctx_a.contains_entries(&qname_a));
  assert!(!ctx_a.contains_entry(&name_a_b));
  assert!(!ctx_a.contains_entries(&qname_a_b));
  assert!(!ctx_a.contains_entry(&name_a_b_c));
  assert!(!ctx_a.contains_entries(&qname_a_b_c));
  assert_eq!("10", ctx_a.get_entry(&name_a).unwrap().to_string().as_str());
}

#[test]
fn test_context_two_levels() {
  let name_married = Name::from("married");
  let name_age = Name::from("age");
  let name_b = Name::from("b");
  let name_x_y = Name::new(&["x", "y"]);
  let qname_married = QualifiedName::new(&[&name_married]);
  let qname_age = QualifiedName::new(&[&name_age]);
  let qname_b = QualifiedName::new(&[&name_b]);
  let qname_b_married = QualifiedName::new(&[&name_b, &name_married]);
  let qname_b_married_age = QualifiedName::new(&[&name_b, &name_married, &name_age]);
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_married, Value::Boolean(true));
  assert_eq!("{married: true}", ctx_b.to_string());
  assert!(ctx_b.contains_entry(&name_married));
  assert!(ctx_b.contains_entries(&qname_married));
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_age, value_number!(49));
  ctx_a.set_entry(&name_x_y, Value::Boolean(true));
  ctx_a.set_entry(&name_b, ctx_b.into());
  assert_eq!("{age: 49, b: {married: true}, x y: true}", ctx_a.to_string());
  assert!(ctx_a.contains_entry(&name_age));
  assert!(ctx_a.contains_entry(&name_b));
  assert!(ctx_a.contains_entry(&name_x_y));
  assert!(ctx_a.contains_entries(&qname_age));
  assert!(ctx_a.contains_entries(&qname_b));
  assert!(ctx_a.contains_entries(&qname_b_married));
  assert!(!ctx_a.contains_entries(&qname_b_married_age));
  assert_eq!("49", ctx_a.get_entry(&name_age).unwrap().to_string().as_str());
  assert_eq!("{married: true}", ctx_a.get_entry(&name_b).unwrap().to_string().as_str());
}

#[test]
fn test_context_three_levels() {
  let name_car = Name::from("car");
  let name_married = Name::from("married");
  let name_age = Name::from("age");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_car, Value::String("opel".to_string()));
  assert_eq!(r#"{car: "opel"}"#, ctx_c.to_string());
  assert!(ctx_c.contains_entry(&name_car));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_married, Value::Boolean(true));
  ctx_b.set_entry(&name_c, ctx_c.into());
  assert_eq!(r#"{c: {car: "opel"}, married: true}"#, ctx_b.to_string());
  assert!(ctx_b.contains_entry(&name_married));
  assert!(ctx_b.contains_entry(&name_c));
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_age, value_number!(49));
  ctx_a.set_entry(&name_b, ctx_b.into());
  assert_eq!(r#"{age: 49, b: {c: {car: "opel"}, married: true}}"#, ctx_a.to_string());
  assert!(ctx_a.contains_entry(&name_age));
  assert!(ctx_a.contains_entry(&name_b));
}

#[test]
fn test_context_search_entry() {
  let name_married = Name::from("married");
  let name_age = Name::from("age");
  let name_b = Name::from("b");
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_married, Value::Boolean(true));
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_age, value_number!(49));
  ctx_a.set_entry(&name_b, ctx_b.into());
  let qn_empty = QualifiedName::new(&[]);
  assert!(ctx_a.search_entry(&qn_empty).is_none());
  let qn_b = QualifiedName::new(&[&name_b]);
  assert_eq!("{married: true}", ctx_a.search_entry(&qn_b).unwrap().to_string().as_str());
  let qn_b_married = QualifiedName::new(&[&name_b, &name_married]);
  assert_eq!("true", ctx_a.search_entry(&qn_b_married).unwrap().to_string().as_str());
}

#[test]
fn test_context_search_entries() {
  // prepare names
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let name_e = Name::from("e");
  // prepare qualified names
  let qn_empty = QualifiedName::new(&[]);
  let qn_a = QualifiedName::new(&[&name_a]);
  let qn_b = QualifiedName::new(&[&name_b]);
  let qn_c = QualifiedName::new(&[&name_c]);
  let qn_a_b = QualifiedName::new(&[&name_a, &name_b]);
  let qn_a_c = QualifiedName::new(&[&name_a, &name_c]);
  let qn_b_c = QualifiedName::new(&[&name_b, &name_c]);
  let qn_b_e = QualifiedName::new(&[&name_b, &name_e]);
  let qn_b_d = QualifiedName::new(&[&name_b, &name_d]);
  let qn_b_d_a = QualifiedName::new(&[&name_b, &name_d, &name_a]);
  let qn_b_d_e = QualifiedName::new(&[&name_b, &name_d, &name_e]);
  // prepare contexts
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_e, Value::String("e".to_string()));
  assert_eq!(r#"{e: "e"}"#, ctx_c.to_string());
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_c, Value::String("c".to_string()));
  ctx_b.set_entry(&name_d, ctx_c.into());
  assert_eq!(r#"{c: "c", d: {e: "e"}}"#, ctx_b.to_string());
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, Value::String("a".to_string()));
  ctx_a.set_entry(&name_b, ctx_b.into());
  assert_eq!(r#"{a: "a", b: {c: "c", d: {e: "e"}}}"#, ctx_a.to_string());
  // test searching entries
  assert!(!ctx_a.contains_entries(&qn_empty));
  assert!(ctx_a.contains_entries(&qn_a));
  assert!(ctx_a.contains_entries(&qn_b));
  assert!(!ctx_a.contains_entries(&qn_c));
  assert!(!ctx_a.contains_entries(&qn_a_b));
  assert!(!ctx_a.contains_entries(&qn_a_c));
  assert!(ctx_a.contains_entries(&qn_b_c));
  assert!(!ctx_a.contains_entries(&qn_b_e));
  assert!(ctx_a.contains_entries(&qn_b_d));
  assert!(!ctx_a.contains_entries(&qn_b_d_a));
  assert!(ctx_a.contains_entries(&qn_b_d_e));
}

#[test]
fn test_context_create_entry() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let qn_a = QualifiedName::new(&[&name_a]);
  let qn_b = QualifiedName::new(&[&name_b]);
  let qn_a_b = QualifiedName::new(&[&name_a, &name_b]);
  let qn_a_c = QualifiedName::new(&[&name_a, &name_c]);
  let qn_a_d = QualifiedName::new(&[&name_a, &name_d]);
  let qn_c_d = QualifiedName::new(&[&name_c, &name_d]);
  let qn_a_b_c = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  let qn_a_b_c_d = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d]);
  let mut ctx: FeelContext = Default::default();
  ctx.create_entry(&qn_a_b_c_d, Value::Boolean(true));
  assert_eq!("{a: {b: {c: {d: true}}}}", ctx.to_string());
  assert_eq!("{b: {c: {d: true}}}", ctx.search_entry(&qn_a).unwrap().to_string());
  assert_eq!("{c: {d: true}}", ctx.search_entry(&qn_a_b).unwrap().to_string());
  assert_eq!("{d: true}", ctx.search_entry(&qn_a_b_c).unwrap().to_string());
  assert_eq!("true", ctx.search_entry(&qn_a_b_c_d).unwrap().to_string());
  let mut ctx: FeelContext = Default::default();
  ctx.create_entry(&qn_a, Value::Boolean(true));
  ctx.create_entry(&qn_b, Value::Boolean(false));
  ctx.create_entry(&qn_c_d, Value::String("deep".to_string()));
  assert_eq!(r#"{a: true, b: false, c: {d: "deep"}}"#, ctx.to_string());
  let mut ctx: FeelContext = Default::default();
  ctx.create_entry(&qn_a_b, Value::String("b".to_string()));
  ctx.create_entry(&qn_a_c, Value::String("c".to_string()));
  ctx.create_entry(&qn_a_d, Value::String("d".to_string()));
  assert_eq!(r#"{a: {b: "b", c: "c", d: "d"}}"#, ctx.to_string());
}

#[test]
fn test_context_create_entries() {
  let name_a: Name = "a".into();
  let name_b: Name = "b".into();
  let name_c: Name = "c".into();
  let name_d: Name = "d".into();
  let v_a = VALUE_TRUE;
  let v_b = VALUE_FALSE;
  let mut ctx: FeelContext = Default::default();
  ctx.create_entries(&[], v_a.clone());
  assert_eq!("{}", ctx.to_string());
  ctx.create_entries(std::slice::from_ref(&name_a), v_a.clone());
  assert_eq!("{a: true}", ctx.to_string());
  ctx.create_entries(&[name_a.clone(), name_b.clone()], v_a.clone());
  assert_eq!("{a: {b: true}}", ctx.to_string());
  ctx.create_entries(&[name_a.clone(), name_b.clone(), name_c.clone()], v_a.clone());
  assert_eq!("{a: {b: {c: true}}}", ctx.to_string());
  ctx.create_entries(&[name_a.clone(), name_b.clone(), name_c, name_d], v_a);
  assert_eq!("{a: {b: {c: {d: true}}}}", ctx.to_string());
  ctx.create_entries(&[name_a, name_b], v_b);
  assert_eq!("{a: {b: false}}", ctx.to_string());
}

#[test]
fn test_context_apply_entries() {
  let name_a: Name = "a".into();
  let name_b: Name = "b".into();
  let name_c: Name = "c".into();
  let name_d: Name = "d".into();
  let v_a = VALUE_TRUE;
  let v_b = VALUE_FALSE;
  let v_c = Value::Context(FeelContext::new());
  let mut ctx = FeelContext::new();
  ctx.apply_entries(&[], v_a.clone()).unwrap();
  assert_eq!("{}", ctx.to_string());
  ctx.apply_entries(slice::from_ref(&name_a), v_c.clone()).unwrap();
  assert_eq!("{a: {}}", ctx.to_string());
  ctx.apply_entries(&[name_a.clone(), name_b.clone()], v_c.clone()).unwrap();
  assert_eq!("{a: {b: {}}}", ctx.to_string());
  ctx.apply_entries(&[name_a.clone(), name_b.clone(), name_c.clone()], v_c.clone()).unwrap();
  assert_eq!("{a: {b: {c: {}}}}", ctx.to_string());
  ctx.apply_entries(&[name_a.clone(), name_b.clone(), name_c.clone(), name_d.clone()], v_a.clone()).unwrap();
  assert_eq!("{a: {b: {c: {d: true}}}}", ctx.to_string());
  ctx.apply_entries(&[name_a.clone(), name_b.clone(), name_c.clone()], v_b).unwrap();
  assert_eq!("{a: {b: {c: false}}}", ctx.to_string());
  // net operation should fail, because `c` is not a context
  ctx.apply_entries(&[name_a, name_b, name_c, name_d], v_a).unwrap_err();
}

#[test]
fn test_context_set_null() {
  let name_a = Name::from("a");
  let mut ctx: FeelContext = Default::default();
  ctx.set_null(name_a);
  assert_eq!(r#"{a: null}"#, ctx.to_string());
}

#[test]
fn test_get_entries() {
  let mut ctx: FeelContext = Default::default();
  assert_eq!(0, ctx.get_entries().len());
  let name_a = Name::new(&["a"]);
  ctx.set_entry(&name_a, value_number!(1));
  assert_eq!(1, ctx.get_entries().len());
  assert_eq!(vec![(&name_a, &value_number!(1))], ctx.get_entries());
  let name_b = Name::new(&["b"]);
  ctx.set_entry(&name_b, value_number!(2));
  assert_eq!(2, ctx.get_entries().len());
  assert_eq!(vec![(&name_a, &value_number!(1)), (&name_b, &value_number!(2))], ctx.get_entries());
}

#[test]
fn test_zip() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  ctx_a.set_entry(&name_b, value_number!(2));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_c, value_number!(3));
  ctx_a.zip(&ctx_b);
  assert_eq!(
    vec![(&name_a, &value_number!(1)), (&name_b, &value_number!(2)), (&name_c, &value_number!(3))],
    ctx_a.get_entries()
  );
}

#[test]
fn test_overwrite() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  ctx_a.set_entry(&name_b, value_number!(2));
  ctx_a.set_entry(&name_c, value_number!(3));
  ctx_a.set_entry(&name_d, value_number!(4));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_b, value_number!(20));
  ctx_b.set_entry(&name_c, value_number!(30));
  ctx_a.overwrite(&ctx_b);
  assert_eq!(
    vec![
      (&name_a, &value_number!(1)),
      (&name_b, &value_number!(20)),
      (&name_c, &value_number!(30)),
      (&name_d, &value_number!(4))
    ],
    ctx_a.get_entries()
  );
}

#[test]
fn test_try_from_value_context() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  ctx_a.set_entry(&name_b, value_number!(2));
  ctx_a.set_entry(&name_c, value_number!(3));
  ctx_a.set_entry(&name_d, value_number!(4));
  let v = Value::Context(ctx_a);
  let ctx_b = FeelContext::try_from(v).unwrap();
  assert_eq!(
    vec![
      (&name_a, &value_number!(1)),
      (&name_b, &value_number!(2)),
      (&name_c, &value_number!(3)),
      (&name_d, &value_number!(4))
    ],
    ctx_b.get_entries()
  );
}

#[test]
fn test_deref() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  ctx_a.set_entry(&name_b, value_number!(2));
  ctx_a.set_entry(&name_c, value_number!(3));
  ctx_a.set_entry(&name_d, value_number!(4));
  let entries = &(*ctx_a);
  entries.contains_key(&name_a);
  assert_eq!(value_number!(1), entries.get(&name_a).unwrap().clone());
  entries.contains_key(&name_b);
  assert_eq!(value_number!(2), entries.get(&name_b).unwrap().clone());
  entries.contains_key(&name_c);
  assert_eq!(value_number!(3), entries.get(&name_c).unwrap().clone());
  entries.contains_key(&name_d);
  assert_eq!(value_number!(4), entries.get(&name_d).unwrap().clone());
}

#[test]
fn test_clone() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(10));
  ctx_a.set_entry(&name_b, value_number!(20));
  ctx_a.set_entry(&name_c, value_number!(30));
  ctx_a.set_entry(&name_d, value_number!(40));
  let ctx_b = ctx_a.clone();
  assert_eq!(
    vec![
      (&name_a, &value_number!(10)),
      (&name_b, &value_number!(20)),
      (&name_c, &value_number!(30)),
      (&name_d, &value_number!(40))
    ],
    ctx_b.get_entries()
  );
}

#[test]
fn test_value_is_not_a_context() {
  assert_eq!(
    "<ContextError> 'true' is not a value containing context",
    <Value as TryInto<FeelContext>>::try_into(Value::Boolean(true)).err().unwrap().to_string()
  );
}
