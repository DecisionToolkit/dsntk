use crate::iterations::{FeelIterator, ForExpressionEvaluator};
use crate::tests::te_scope;
use dsntk_feel::value_number;
use dsntk_feel::values::{Value, Values, values_to_string};

#[test]
fn _0001() {
  let mut iterator = FeelIterator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(3), false).unwrap();
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(3, actual.len());
  assert_eq!(r#"[{x: 1}, {x: 2}, {x: 3}]"#, values_to_string(&actual));
}

#[test]
fn _0002() {
  let mut iterator = FeelIterator::new();
  iterator.add_interval("x".into(), value_number!(3), value_number!(1), false).unwrap();
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(3, actual.len());
  assert_eq!(r#"[{x: 3}, {x: 2}, {x: 1}]"#, values_to_string(&actual));
}

#[test]
fn _0003() {
  let mut iterator = FeelIterator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(3), false).unwrap();
  iterator.add_interval("y".into(), value_number!(1), value_number!(5), false).unwrap();
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(15, actual.len());
  assert_eq!(
    r#"[{x: 1, y: 1}, {x: 1, y: 2}, {x: 1, y: 3}, {x: 1, y: 4}, {x: 1, y: 5}, {x: 2, y: 1}, {x: 2, y: 2}, {x: 2, y: 3}, {x: 2, y: 4}, {x: 2, y: 5}, {x: 3, y: 1}, {x: 3, y: 2}, {x: 3, y: 3}, {x: 3, y: 4}, {x: 3, y: 5}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0004() {
  let mut iterator = FeelIterator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(3), false).unwrap();
  iterator.add_interval("y".into(), value_number!(5), value_number!(1), false).unwrap();
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(15, actual.len());
  assert_eq!(
    r#"[{x: 1, y: 5}, {x: 1, y: 4}, {x: 1, y: 3}, {x: 1, y: 2}, {x: 1, y: 1}, {x: 2, y: 5}, {x: 2, y: 4}, {x: 2, y: 3}, {x: 2, y: 2}, {x: 2, y: 1}, {x: 3, y: 5}, {x: 3, y: 4}, {x: 3, y: 3}, {x: 3, y: 2}, {x: 3, y: 1}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0005() {
  let mut iterator = FeelIterator::new();
  iterator.add_interval("x".into(), value_number!(3), value_number!(1), false).unwrap();
  iterator.add_interval("y".into(), value_number!(5), value_number!(1), false).unwrap();
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(15, actual.len());
  assert_eq!(
    r#"[{x: 3, y: 5}, {x: 3, y: 4}, {x: 3, y: 3}, {x: 3, y: 2}, {x: 3, y: 1}, {x: 2, y: 5}, {x: 2, y: 4}, {x: 2, y: 3}, {x: 2, y: 2}, {x: 2, y: 1}, {x: 1, y: 5}, {x: 1, y: 4}, {x: 1, y: 3}, {x: 1, y: 2}, {x: 1, y: 1}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0006() {
  let mut iterator = FeelIterator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(2), false).unwrap();
  iterator.add_interval("y".into(), value_number!(1), value_number!(3), false).unwrap();
  iterator.add_interval("z".into(), value_number!(1), value_number!(4), false).unwrap();
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(24, actual.len());
  assert_eq!(
    r#"[{x: 1, y: 1, z: 1}, {x: 1, y: 1, z: 2}, {x: 1, y: 1, z: 3}, {x: 1, y: 1, z: 4}, {x: 1, y: 2, z: 1}, {x: 1, y: 2, z: 2}, {x: 1, y: 2, z: 3}, {x: 1, y: 2, z: 4}, {x: 1, y: 3, z: 1}, {x: 1, y: 3, z: 2}, {x: 1, y: 3, z: 3}, {x: 1, y: 3, z: 4}, {x: 2, y: 1, z: 1}, {x: 2, y: 1, z: 2}, {x: 2, y: 1, z: 3}, {x: 2, y: 1, z: 4}, {x: 2, y: 2, z: 1}, {x: 2, y: 2, z: 2}, {x: 2, y: 2, z: 3}, {x: 2, y: 2, z: 4}, {x: 2, y: 3, z: 1}, {x: 2, y: 3, z: 2}, {x: 2, y: 3, z: 3}, {x: 2, y: 3, z: 4}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0007() {
  let mut iterator = FeelIterator::new();
  let list = vec![Value::String("a".to_string()), Value::String("b".to_string()), Value::String("c".to_string())];
  iterator.add_list("x".into(), Value::List(list));
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(3, actual.len());
  assert_eq!(r#"[{x: "a"}, {x: "b"}, {x: "c"}]"#, values_to_string(&actual));
}

#[test]
fn _0008() {
  let mut iterator = FeelIterator::new();
  let list1 = vec![Value::String("a".to_string()), Value::String("b".to_string()), Value::String("c".to_string())];
  iterator.add_list("x".into(), Value::List(list1));
  let list2 = vec![value_number!(1), value_number!(2), value_number!(3)];
  iterator.add_list("y".into(), Value::List(list2));
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(9, actual.len());
  assert_eq!(
    r#"[{x: "a", y: 1}, {x: "a", y: 2}, {x: "a", y: 3}, {x: "b", y: 1}, {x: "b", y: 2}, {x: "b", y: 3}, {x: "c", y: 1}, {x: "c", y: 2}, {x: "c", y: 3}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0009() {
  let mut iterator = FeelIterator::new();
  let list_x = vec![Value::String("a".to_string()), Value::String("b".to_string())];
  let list_y = vec![value_number!(1), value_number!(2), value_number!(3)];
  let list_z = vec![value_number!(1), value_number!(2), value_number!(3), value_number!(4)];
  iterator.add_list("x".into(), Value::List(list_x));
  iterator.add_list("y".into(), Value::List(list_y));
  iterator.add_list("z".into(), Value::List(list_z));
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(24, actual.len());
  assert_eq!(
    r#"[{x: "a", y: 1, z: 1}, {x: "a", y: 1, z: 2}, {x: "a", y: 1, z: 3}, {x: "a", y: 1, z: 4}, {x: "a", y: 2, z: 1}, {x: "a", y: 2, z: 2}, {x: "a", y: 2, z: 3}, {x: "a", y: 2, z: 4}, {x: "a", y: 3, z: 1}, {x: "a", y: 3, z: 2}, {x: "a", y: 3, z: 3}, {x: "a", y: 3, z: 4}, {x: "b", y: 1, z: 1}, {x: "b", y: 1, z: 2}, {x: "b", y: 1, z: 3}, {x: "b", y: 1, z: 4}, {x: "b", y: 2, z: 1}, {x: "b", y: 2, z: 2}, {x: "b", y: 2, z: 3}, {x: "b", y: 2, z: 4}, {x: "b", y: 3, z: 1}, {x: "b", y: 3, z: 2}, {x: "b", y: 3, z: 3}, {x: "b", y: 3, z: 4}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0010() {
  let mut iterator = FeelIterator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(2), false).unwrap();
  iterator.add_list("y".into(), Value::List(vec![value_number!(1), value_number!(2), value_number!(3)]));
  let mut actual = vec![];
  iterator.iterate(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(6, actual.len());
  assert_eq!(
    r#"[{x: 1, y: 1}, {x: 1, y: 2}, {x: 1, y: 3}, {x: 2, y: 1}, {x: 2, y: 2}, {x: 2, y: 3}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0011() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(3)).unwrap();
  let scope = &te_scope(r#"{x:null}"#);
  let node = dsntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node);
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(3, actual.len());
  assert_eq!(r#"[2, 3, 4]"#, values_to_string(&actual));
}

#[test]
fn _0012() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(2)).unwrap();
  iterator.add_list("y".into(), Value::List(vec![value_number!(5), value_number!(6), value_number!(7)]));
  let scope = &te_scope(r#"{x:null,y:null}"#);
  let node = dsntk_feel_parser::parse_expression(scope, "x+y", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node);
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(6, actual.len());
  assert_eq!(r#"[6, 7, 8, 7, 8, 9]"#, values_to_string(&actual));
}

#[test]
fn _0013() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_list("x".into(), Value::List(Values::default()));
  let scope = &te_scope(r#"{x:null}"#);
  let node = dsntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node);
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(0, actual.len());
  assert_eq!(r#"[]"#, values_to_string(&actual));
}

#[test]
fn _0014() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_list("x".into(), value_number!(1));
  let scope = &te_scope(r#"{x:null}"#);
  let node = dsntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node);
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(1, actual.len());
  assert_eq!(r#"[2]"#, values_to_string(&actual));
}

#[test]
fn _0015() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_interval("x".into(), value_number!(1), value_number!(2)).unwrap();
  iterator.add_list("y".into(), Value::List(Values::default()));
  let scope = &te_scope(r#"{x:null,y:null}"#);
  let node = dsntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node);
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(0, actual.len());
  assert_eq!(r#"[]"#, values_to_string(&actual));
}

#[test]
fn _0016() {
  let mut iterator = ForExpressionEvaluator::new();
  let list = Value::List(vec![value_number!(1), value_number!(2), value_number!(3), value_number!(4)]);
  iterator.add_list("x".into(), list);
  iterator.add_variable("y".into(), "x".into());
  let scope = &te_scope(r#"{x:null,y:null}"#);
  let node = dsntk_feel_parser::parse_expression(scope, "y", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node);
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(4, actual.len());
  assert_eq!(r#"[1, 2, 3, 4]"#, values_to_string(&actual));
}
