use super::*;
use crate::AstNode;
use dsntk_feel::{FeelType, IntervalType};
use std::collections::BTreeMap;

#[test]
fn test_equal() {
  let node_a = AstNode::Add(b_num!(1), b_num!(2));
  let node_b = AstNode::Add(b_num!(1), b_num!(2));
  let result = node_a == node_b;
  assert!(result);
}

#[test]
fn test_clone() {
  let node_a = AstNode::Add(b_num!(1), b_num!(2));
  eqs(
    r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#,
    &node_a.clone(),
  );
}

#[test]
fn test_display() {
  assert_eq!(
    r#" Add
 ├─ Numeric
 │  └─ `1`
 └─ Numeric
    └─ `2`
"#,
    format!("{}", AstNode::Add(b_num!(1), b_num!(2))),
  );
}

#[test]
fn test_node_add() {
  let node = &AstNode::Add(b_num!(1, 23), b_num!(2, 34));
  eqd(r#"Add(Numeric("1", "23", '+', ""), Numeric("2", "34", '+', ""))"#, node);
  eqs(
    r#"
       Add
       ├─ Numeric
       │  └─ `1.23`
       └─ Numeric
          └─ `2.34`
    "#,
    node,
  );
}

#[test]
fn test_node_add_string() {
  let node = &AstNode::Add(b_num!(1, 23), Box::new(AstNode::String("12".to_string())));
  eqd(r#"Add(Numeric("1", "23", '+', ""), String("12"))"#, node);
  eqs(
    r#"
       Add
       ├─ Numeric
       │  └─ `1.23`
       └─ String
          └─ `12`
    "#,
    node,
  );
}

#[test]
fn test_node_and() {
  let node = &AstNode::And(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  eqd(r#"And(Boolean(true), Boolean(false))"#, node);
  eqs(
    r#"
       And
       ├─ Boolean
       │  └─ `true`
       └─ Boolean
          └─ `false`
    "#,
    node,
  );
}

#[test]
fn test_node_at() {
  let node = &AstNode::At("2022-09-26".to_string());
  eqd(r#"At("2022-09-26")"#, node);
  eqs(
    r#"
       At
       └─ `2022-09-26`
    "#,
    node,
  );
}

#[test]
fn test_node_between() {
  let node = &AstNode::Between(Box::new(AstNode::Name("x".into())), b_num!(s!(1), s!()), b_num!(s!(10), s!()));
  eqd(r#"Between(Name(Name("x")), Numeric("1", "", '+', ""), Numeric("10", "", '+', ""))"#, node);
  eqs(
    r#"
       Between
       ├─ Name
       │  └─ `x`
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `10`
    "#,
    node,
  );
}

#[test]
fn test_node_boolean() {
  let node = &AstNode::Boolean(true);
  eqd(r#"Boolean(true)"#, node);
  eqs(
    r#"
       Boolean
       └─ `true`
    "#,
    node,
  );
}

#[test]
fn test_node_comma_list() {
  let node = &AstNode::CommaList(vec![_num!("1"), _num!("2"), _num!("3")]);
  eqd(r#"CommaList([Numeric("1", "", '+', ""), Numeric("2", "", '+', ""), Numeric("3", "", '+', "")])"#, node);
  eqs(
    r#"
       CommaList
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       └─ Numeric
          └─ `3`
    "#,
    node,
  );
}

#[test]
fn test_node_context() {
  let node = &AstNode::Context(vec![
    AstNode::ContextEntry(Box::new(AstNode::ContextEntryKey(s!("count").into())), b_num!(s!(1), s!())),
    AstNode::ContextEntry(Box::new(AstNode::ContextEntryKey(s!("amount").into())), b_num!(s!(99), s!(99))),
  ]);
  eqd(
    r#"Context([ContextEntry(ContextEntryKey(Name("count")), Numeric("1", "", '+', "")), ContextEntry(ContextEntryKey(Name("amount")), Numeric("99", "99", '+', ""))])"#,
    node,
  );
  eqs(
    r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `count`
       │  └─ Numeric
       │     └─ `1`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `amount`
          └─ Numeric
             └─ `99.99`
    "#,
    node,
  );
}

#[test]
fn test_node_context_entry() {
  let node = &AstNode::ContextEntry(Box::new(AstNode::ContextEntryKey(s!("count").into())), b_num!(s!(1), s!()));
  eqd(r#"ContextEntry(ContextEntryKey(Name("count")), Numeric("1", "", '+', ""))"#, node);
  eqs(
    r#"
       ContextEntry
       ├─ ContextEntryKey
       │  └─ `count`
       └─ Numeric
          └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_context_entry_key() {
  let node = &AstNode::ContextEntryKey(s!("count").into());
  eqd(r#"ContextEntryKey(Name("count"))"#, node);
  eqs(
    r#"
       ContextEntryKey
       └─ `count`
    "#,
    node,
  );
}

#[test]
fn test_node_context_type() {
  let node = &AstNode::ContextType(vec![]);
  eqd(r#"ContextType([])"#, node);
  eqs(
    r#"
       ContextType
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::ContextType(vec![
    AstNode::ContextTypeEntry(Box::new(AstNode::ContextTypeEntryKey(__name!(count))), Box::new(AstNode::FeelType(FeelType::Number))),
    AstNode::ContextTypeEntry(b_name!(amount), Box::new(AstNode::FeelType(FeelType::Number))),
  ]);
  eqd(
    r#"ContextType([ContextTypeEntry(ContextTypeEntryKey(Name("count")), FeelType(Number)), ContextTypeEntry(Name(Name("amount")), FeelType(Number))])"#,
    node,
  );
  let mut types: BTreeMap<Name, FeelType> = BTreeMap::new();
  types.insert(__name!(count), FeelType::Number);
  types.insert(__name!(amount), FeelType::Number);
  eqs(
    r#"
       ContextType
       ├─ ContextTypeEntry
       │  ├─ Name
       │  │  └─ `count`
       │  └─ FeelType
       │     └─ number
       └─ ContextTypeEntry
          ├─ Name
          │  └─ `amount`
          └─ FeelType
             └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_context_type_entry() {
  let node = &AstNode::ContextTypeEntry(Box::new(AstNode::ContextTypeEntryKey(s!("count").into())), Box::new(AstNode::FeelType(FeelType::Number)));
  eqd(r#"ContextTypeEntry(ContextTypeEntryKey(Name("count")), FeelType(Number))"#, node);
  eqs(
    r#"
       ContextTypeEntry
       ├─ Name
       │  └─ `count`
       └─ FeelType
          └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_context_type_entry_key() {
  let node = &AstNode::ContextTypeEntryKey(s!("count").into());
  eqd(r#"ContextTypeEntryKey(Name("count"))"#, node);
  eqs(
    r#"
       Name
       └─ `count`
    "#,
    node,
  );
}

#[test]
fn test_node_div() {
  let node = &AstNode::Div(b_num!(s!(1), s!(23)), b_num!(s!(2), s!(34)));
  eqd(r#"Div(Numeric("1", "23", '+', ""), Numeric("2", "34", '+', ""))"#, node);
  eqs(
    r#"
       Div
       ├─ Numeric
       │  └─ `1.23`
       └─ Numeric
          └─ `2.34`
    "#,
    node,
  );
}

#[test]
fn test_node_eq() {
  let node = &AstNode::Eq(Box::new(AstNode::String(s!("a"))), Box::new(AstNode::String(s!("b"))));
  eqd(r#"Eq(String("a"), String("b"))"#, node);
  eqs(
    r#"
       Eq
       ├─ String
       │  └─ `a`
       └─ String
          └─ `b`
    "#,
    node,
  );
}

#[test]
fn test_node_evaluated_expression() {
  let node = &AstNode::EvaluatedExpression(Box::new(AstNode::Name(s!("amount").into())));
  eqd(r#"EvaluatedExpression(Name(Name("amount")))"#, node);
  eqs(
    r#"
       EvaluatedExpression
       └─ Name
          └─ `amount`
    "#,
    node,
  );
}

#[test]
fn test_node_every() {
  let node = &AstNode::Every(b_num!(s!(1), s!()), Box::new(AstNode::List(vec![_num!(s!(1), s!())])));
  eqd(r#"Every(Numeric("1", "", '+', ""), List([Numeric("1", "", '+', "")]))"#, node);
  eqs(
    r#"
       Every
       ├─ Numeric
       │  └─ `1`
       └─ List
          └─ Numeric
             └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_exp() {
  let node = &AstNode::Exp(b_num!(s!(1), s!(23)), b_num!(s!(2), s!(34)));
  eqd(r#"Exp(Numeric("1", "23", '+', ""), Numeric("2", "34", '+', ""))"#, node);
  eqs(
    r#"
       Exp
       ├─ Numeric
       │  └─ `1.23`
       └─ Numeric
          └─ `2.34`
    "#,
    node,
  );
}

#[test]
fn test_node_expression_list() {
  let node = &AstNode::ExpressionList(vec![]);
  eqd(r#"ExpressionList([])"#, node);
  eqs(
    r#"
       ExpressionList
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::ExpressionList(vec![_num!(s!(1), s!())]);
  eqd(r#"ExpressionList([Numeric("1", "", '+', "")])"#, node);
  eqs(
    r#"
       ExpressionList
       └─ Numeric
          └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_filter() {
  let node = &AstNode::Filter(
    Box::new(AstNode::List(vec![_num!(s!(1), s!()), _num!(s!(2), s!())])),
    Box::new(AstNode::Gt(Box::new(AstNode::Name(s!("count").into())), b_num!(s!(1), s!()))),
  );
  eqd(
    r#"Filter(List([Numeric("1", "", '+', ""), Numeric("2", "", '+', "")]), Gt(Name(Name("count")), Numeric("1", "", '+', "")))"#,
    node,
  );
  eqs(
    r#"
       Filter
       ├─ List
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `2`
       └─ Gt
          ├─ Name
          │  └─ `count`
          └─ Numeric
             └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_for() {
  let node = &AstNode::For(Box::new(AstNode::Name(s!("i").into())), Box::new(AstNode::List(vec![_num!(1), _num!(2)])));
  eqd(r#"For(Name(Name("i")), List([Numeric("1", "", '+', ""), Numeric("2", "", '+', "")]))"#, node);
  eqs(
    r#"
       For
       ├─ Name
       │  └─ `i`
       └─ List
          ├─ Numeric
          │  └─ `1`
          └─ Numeric
             └─ `2`
    "#,
    node,
  );
}

#[test]
fn test_node_formal_parameter() {
  let node = &AstNode::FormalParameter(Box::new(AstNode::Name(s!("amount").into())), Box::new(AstNode::FeelType(FeelType::Number)));
  eqd(r#"FormalParameter(Name(Name("amount")), FeelType(Number))"#, node);
  eqs(
    r#"
       FormalParameter
       ├─ Name
       │  └─ `amount`
       └─ FeelType
          └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_formal_parameters() {
  let node = &AstNode::FormalParameters(vec![]);
  eqd(r#"FormalParameters([])"#, node);
  eqs(
    r#"
       FormalParameters
       └─ (empty)
    "#,
    node,
  );
}

#[test]
fn test_node_function_body() {
  let node = &AstNode::FunctionBody(b_num!(1), false);
  eqd(r#"FunctionBody(Numeric("1", "", '+', ""), false)"#, node);
  eqs(
    r#"
       FunctionBody
       └─ Numeric
          └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_function_definition() {
  let node = &AstNode::FunctionDefinition(Box::new(AstNode::FormalParameters(vec![])), Box::new(AstNode::FunctionBody(b_num!(1), false)));
  eqd(r#"FunctionDefinition(FormalParameters([]), FunctionBody(Numeric("1", "", '+', ""), false))"#, node);
  eqs(
    r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ (empty)
       └─ FunctionBody
          └─ Numeric
             └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_function_invocation() {
  let node = &AstNode::FunctionInvocation(b_name!(calculate), Box::new(AstNode::FormalParameters(vec![])));
  eqd(r#"FunctionInvocation(Name(Name("calculate")), FormalParameters([]))"#, node);
  eqs(
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `calculate`
       └─ FormalParameters
          └─ (empty)
    "#,
    node,
  );
}

#[test]
fn test_node_function_irrelevant() {
  let node = &AstNode::Irrelevant;
  eqd(r#"Irrelevant"#, node);
  eqs(
    r#"
       Irrelevant
    "#,
    node,
  );
}

#[test]
fn test_node_function_type() {
  let node = &AstNode::FunctionType(
    Box::new(AstNode::ParameterTypes(vec![AstNode::FeelType(FeelType::Number)])),
    Box::new(AstNode::FeelType(FeelType::Boolean)),
  );
  eqd(r#"FunctionType(ParameterTypes([FeelType(Number)]), FeelType(Boolean))"#, node);
  eqs(
    r#"
       FunctionType
       ├─ ParameterTypes
       │  └─ FeelType
       │     └─ number
       └─ FeelType
          └─ boolean
    "#,
    node,
  );
  let node = &AstNode::FunctionType(Box::new(AstNode::FeelType(FeelType::Boolean)), Box::new(AstNode::FeelType(FeelType::Boolean)));
  eqd(r#"FunctionType(FeelType(Boolean), FeelType(Boolean))"#, node);
  eqs(
    r#"
       FunctionType
       ├─ FeelType
       │  └─ boolean
       └─ FeelType
          └─ boolean
    "#,
    node,
  );
}

#[test]
fn test_node_ge() {
  let node = &AstNode::Ge(Box::new(AstNode::String(s!("a"))), Box::new(AstNode::String(s!("b"))));
  eqd(r#"Ge(String("a"), String("b"))"#, node);
  eqs(
    r#"
       Ge
       ├─ String
       │  └─ `a`
       └─ String
          └─ `b`
    "#,
    node,
  );
}

#[test]
fn test_node_gt() {
  let node = &AstNode::Gt(Box::new(AstNode::String(s!("a"))), Box::new(AstNode::String(s!("b"))));
  eqd(r#"Gt(String("a"), String("b"))"#, node);
  eqs(
    r#"
       Gt
       ├─ String
       │  └─ `a`
       └─ String
          └─ `b`
    "#,
    node,
  );
}

#[test]
fn test_node_if() {
  let node = &AstNode::If(Box::new(AstNode::Gt(b_num!(1), b_num!(2))), b_bool!(true), b_bool!(false));
  eqd(r#"If(Gt(Numeric("1", "", '+', ""), Numeric("2", "", '+', "")), Boolean(true), Boolean(false))"#, node);
  eqs(
    r#"
       If
       ├─ Gt
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `2`
       ├─ Boolean
       │  └─ `true`
       └─ Boolean
          └─ `false`
    "#,
    node,
  );
}

#[test]
fn test_node_in() {
  let node = &AstNode::In(b_num!(1), Box::new(AstNode::List(vec![_num!(1), _num!(2), _num!(3)])));
  eqd(
    r#"In(Numeric("1", "", '+', ""), List([Numeric("1", "", '+', ""), Numeric("2", "", '+', ""), Numeric("3", "", '+', "")]))"#,
    node,
  );
  eqs(
    r#"
       In
       ├─ Numeric
       │  └─ `1`
       └─ List
          ├─ Numeric
          │  └─ `1`
          ├─ Numeric
          │  └─ `2`
          └─ Numeric
             └─ `3`
    "#,
    node,
  );
}

#[test]
fn test_node_instance_of() {
  let node = &AstNode::InstanceOf(b_num!(1), Box::new(AstNode::FeelType(FeelType::Number)));
  eqd(r#"InstanceOf(Numeric("1", "", '+', ""), FeelType(Number))"#, node);
  eqs(
    r#"
       InstanceOf
       ├─ Numeric
       │  └─ `1`
       └─ FeelType
          └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_interval_end() {
  let node = &AstNode::IntervalEnd(b_num!(1), IntervalType::Opened);
  eqd(r#"IntervalEnd(Numeric("1", "", '+', ""), Opened)"#, node);
  eqs(
    r#"
       IntervalEnd (opened)
       └─ Numeric
          └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_interval_start() {
  let node = &AstNode::IntervalStart(b_num!(100), IntervalType::Closed);
  eqd(r#"IntervalStart(Numeric("100", "", '+', ""), Closed)"#, node);
  eqs(
    r#"
       IntervalStart (closed)
       └─ Numeric
          └─ `100`
    "#,
    node,
  );
}

#[test]
fn test_node_iteration_contexts() {
  let node = &AstNode::IterationContexts(vec![AstNode::Range(b_num!(1), b_num!(10)), AstNode::Range(b_num!(100), b_num!(110))]);
  eqd(
    r#"IterationContexts([Range(Numeric("1", "", '+', ""), Numeric("10", "", '+', "")), Range(Numeric("100", "", '+', ""), Numeric("110", "", '+', ""))])"#,
    node,
  );
  eqs(
    r#"
       IterationContexts
       ├─ Range
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `10`
       └─ Range
          ├─ Numeric
          │  └─ `100`
          └─ Numeric
             └─ `110`
    "#,
    node,
  );
}

#[test]
fn test_node_iteration_context_list() {
  let node = &AstNode::IterationContextSingle(b_name!(i), Box::new(AstNode::List(vec![_num!(1), _num!(2), _num!(3)])));
  eqd(
    r#"IterationContextSingle(Name(Name("i")), List([Numeric("1", "", '+', ""), Numeric("2", "", '+', ""), Numeric("3", "", '+', "")]))"#,
    node,
  );
  eqs(
    r#"
       IterationContextSingle
       ├─ Name
       │  └─ `i`
       └─ List
          ├─ Numeric
          │  └─ `1`
          ├─ Numeric
          │  └─ `2`
          └─ Numeric
             └─ `3`
    "#,
    node,
  );
}

#[test]
fn test_node_iteration_context_range() {
  let node = &AstNode::IterationContextInterval(b_name!(i), b_num!(1), b_num!(10));
  eqd(r#"IterationContextInterval(Name(Name("i")), Numeric("1", "", '+', ""), Numeric("10", "", '+', ""))"#, node);
  eqs(
    r#"
       IterationContextInterval
       ├─ Name
       │  └─ `i`
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `10`
    "#,
    node,
  );
}

#[test]
fn test_node_le() {
  let node = &AstNode::Le(Box::new(AstNode::String(s!("a"))), Box::new(AstNode::String(s!("b"))));
  eqd(r#"Le(String("a"), String("b"))"#, node);
  eqs(
    r#"
       Le
       ├─ String
       │  └─ `a`
       └─ String
          └─ `b`
    "#,
    node,
  );
}

#[test]
fn test_node_list() {
  let node = &AstNode::List(vec![]);
  eqd(r#"List([])"#, node);
  eqs(
    r#"
       List
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::List(vec![_num!(1), _num!(2), _num!(3)]);
  eqd(r#"List([Numeric("1", "", '+', ""), Numeric("2", "", '+', ""), Numeric("3", "", '+', "")])"#, node);
  eqs(
    r#"
       List
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       └─ Numeric
          └─ `3`
    "#,
    node,
  );
}

#[test]
fn test_node_list_type() {
  let node = &AstNode::ListType(Box::new(AstNode::FeelType(FeelType::Number)));
  eqd(r#"ListType(FeelType(Number))"#, node);
  eqs(
    r#"
       ListType
       └─ FeelType
          └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_lt() {
  let node = &AstNode::Lt(Box::new(AstNode::String(s!("a"))), Box::new(AstNode::String(s!("b"))));
  eqd(r#"Lt(String("a"), String("b"))"#, node);
  eqs(
    r#"
       Lt
       ├─ String
       │  └─ `a`
       └─ String
          └─ `b`
    "#,
    node,
  );
}

#[test]
fn test_node_mul() {
  let node = &AstNode::Mul(b_num!(s!(1), s!(23)), b_num!(s!(2), s!(34)));
  eqd(r#"Mul(Numeric("1", "23", '+', ""), Numeric("2", "34", '+', ""))"#, node);
  eqs(
    r#"
       Mul
       ├─ Numeric
       │  └─ `1.23`
       └─ Numeric
          └─ `2.34`
    "#,
    node,
  );
}

#[test]
fn test_node_name() {
  let node = &AstNode::Name(__name!(a));
  eqd(r#"Name(Name("a"))"#, node);
  eqs(
    r#"
       Name
       └─ `a`
    "#,
    node,
  );
  let node = &AstNode::Name(__name!(number));
  eqd(r#"Name(Name("number"))"#, node);
  eqs(
    r#"
       Name
       └─ `number`
    "#,
    node,
  );
}

#[test]
fn test_node_named_parameter() {
  let node = &AstNode::NamedParameter(Box::new(AstNode::Name(s!("count").into())), Box::new(AstNode::FeelType(FeelType::Number)));
  eqd(r#"NamedParameter(Name(Name("count")), FeelType(Number))"#, node);
  eqs(
    r#"
       NamedParameter
       ├─ Name
       │  └─ `count`
       └─ FeelType
          └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_named_parameters() {
  let node = &AstNode::NamedParameters(vec![]);
  eqd(r#"NamedParameters([])"#, node);
  eqs(
    r#"
       NamedParameters
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::NamedParameters(vec![AstNode::NamedParameter(
    Box::new(AstNode::Name(s!("count").into())),
    Box::new(AstNode::FeelType(FeelType::Number)),
  )]);
  eqd(r#"NamedParameters([NamedParameter(Name(Name("count")), FeelType(Number))])"#, node);
  eqs(
    r#"
       NamedParameters
       └─ NamedParameter
          ├─ Name
          │  └─ `count`
          └─ FeelType
             └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_neg() {
  let node = &AstNode::Neg(b_num!(s!(1), s!(99)));
  eqd(r#"Neg(Numeric("1", "99", '+', ""))"#, node);
  eqs(
    r#"
       Neg
       └─ Numeric
          └─ `1.99`
    "#,
    node,
  );
}

#[test]
fn test_node_negated_list() {
  let node = &AstNode::NegatedList(vec![]);
  eqd(r#"NegatedList([])"#, node);
  eqs(
    r#"
       NegatedList
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::NegatedList(vec![_num!(s!(1), s!())]);
  eqd(r#"NegatedList([Numeric("1", "", '+', "")])"#, node);
  eqs(
    r#"
       NegatedList
       └─ Numeric
          └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_nq() {
  let node = &AstNode::Nq(Box::new(AstNode::String(s!("a"))), Box::new(AstNode::String(s!("b"))));
  eqd(r#"Nq(String("a"), String("b"))"#, node);
  eqs(
    r#"
       Nq
       ├─ String
       │  └─ `a`
       └─ String
          └─ `b`
    "#,
    node,
  );
}

#[test]
fn test_node_number() {
  let node = &_num!(s!(123), s!(456));
  eqd(r#"Numeric("123", "456", '+', "")"#, node);
  eqs(
    r#"
       Numeric
       └─ `123.456`
    "#,
    node,
  );
}

#[test]
fn test_node_null() {
  let node = &AstNode::Null;
  eqd(r#"Null"#, node);
  eqs(
    r#"
       Null
    "#,
    node,
  );
}

#[test]
fn test_node_or() {
  let node = &AstNode::Or(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  eqd(r#"Or(Boolean(true), Boolean(false))"#, node);
  eqs(
    r#"
       Or
       ├─ Boolean
       │  └─ `true`
       └─ Boolean
          └─ `false`
    "#,
    node,
  );
}

#[test]
fn test_node_out() {
  let node = &AstNode::Out(b_num!(s!(1)), b_num!(s!(2)));
  eqd(r#"Out(Numeric("1", "", '+', ""), Numeric("2", "", '+', ""))"#, node);
  eqs(
    r#"
       Out
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#,
    node,
  );
}

#[test]
fn test_node_path() {
  let node = &AstNode::Path(b_name!(alpha), b_name!(beta));
  eqd(r#"Path(Name(Name("alpha")), Name(Name("beta")))"#, node);
  eqs(
    r#"
       Path
       ├─ Name
       │  └─ `alpha`
       └─ Name
          └─ `beta`
    "#,
    node,
  );
}

#[test]
fn test_node_parameter_name() {
  let node = &AstNode::ParameterName(__name!(alpha));
  eqd(r#"ParameterName(Name("alpha"))"#, node);
  eqs(
    r#"
       ParameterName
       └─ `alpha`
    "#,
    node,
  );
}

#[test]
fn test_node_parameter_types() {
  let node = &AstNode::ParameterTypes(vec![]);
  eqd(r#"ParameterTypes([])"#, node);
  eqs(
    r#"
       ParameterTypes
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::ParameterTypes(vec![AstNode::FeelType(FeelType::Number)]);
  eqd(r#"ParameterTypes([FeelType(Number)])"#, node);
  eqs(
    r#"
       ParameterTypes
       └─ FeelType
          └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_positional_parameters() {
  let node = &AstNode::PositionalParameters(vec![]);
  eqd(r#"PositionalParameters([])"#, node);
  eqs(
    r#"
       PositionalParameters
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::PositionalParameters(vec![_num!(1), _num!(2)]);
  eqd(r#"PositionalParameters([Numeric("1", "", '+', ""), Numeric("2", "", '+', "")])"#, node);
  eqs(
    r#"
       PositionalParameters
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#,
    node,
  );
}

#[test]
fn test_node_qualified_name() {
  let node = &AstNode::QualifiedName(vec![]);
  eqd(r#"QualifiedName([])"#, node);
  eqs(
    r#"
       QualifiedName
       └─ (empty)
    "#,
    node,
  );
  let node = &AstNode::QualifiedName(vec![_name!(a), _name!(b), _name!(c)]);
  eqd(r#"QualifiedName([Name(Name("a")), Name(Name("b")), Name(Name("c"))])"#, node);
  eqs(
    r#"
       QualifiedName
       ├─ Name
       │  └─ `a`
       ├─ Name
       │  └─ `b`
       └─ Name
          └─ `c`
    "#,
    node,
  );
  let node = &AstNode::QualifiedName(vec![_name!(count)]);
  eqd(r#"QualifiedName([Name(Name("count"))])"#, node);
  eqs(
    r#"
       QualifiedName
       └─ Name
          └─ `count`
    "#,
    node,
  );
}

#[test]
fn test_node_qualified_name_segment() {
  let node = &AstNode::QualifiedNameSegment(__name!(a));
  eqd(r#"QualifiedNameSegment(Name("a"))"#, node);
  eqs(
    r#"
       Name
       └─ `a`
    "#,
    node,
  );
}

#[test]
fn test_node_quantified_context() {
  let node = &AstNode::QuantifiedContext(b_name!(a), b_num!(10));
  eqd(r#"QuantifiedContext(Name(Name("a")), Numeric("10", "", '+', ""))"#, node);
  eqs(
    r#"
       QuantifiedContext
       ├─ Name
       │  └─ `a`
       └─ Numeric
          └─ `10`
    "#,
    node,
  );
}

#[test]
fn test_node_quantified_contexts() {
  let node = &AstNode::QuantifiedContexts(vec![AstNode::QuantifiedContext(b_name!(a), b_num!(10)), AstNode::QuantifiedContext(b_name!(b), b_num!(20))]);
  eqd(
    r#"QuantifiedContexts([QuantifiedContext(Name(Name("a")), Numeric("10", "", '+', "")), QuantifiedContext(Name(Name("b")), Numeric("20", "", '+', ""))])"#,
    node,
  );
  eqs(
    r#"
       QuantifiedContexts
       ├─ QuantifiedContext
       │  ├─ Name
       │  │  └─ `a`
       │  └─ Numeric
       │     └─ `10`
       └─ QuantifiedContext
          ├─ Name
          │  └─ `b`
          └─ Numeric
             └─ `20`
    "#,
    node,
  );
}

#[test]
fn test_node_range() {
  let node = &AstNode::Range(b_num!(1), b_num!(10));
  eqd(r#"Range(Numeric("1", "", '+', ""), Numeric("10", "", '+', ""))"#, node);
  eqs(
    r#"
       Range
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `10`
    "#,
    node,
  );
}

#[test]
fn test_node_range_type() {
  let node = &AstNode::RangeType(Box::new(AstNode::FeelType(FeelType::Number)));
  eqd(r#"RangeType(FeelType(Number))"#, node);
  eqs(
    r#"
       RangeType
       └─ FeelType
          └─ number
    "#,
    node,
  );
}

#[test]
fn test_node_satisfies() {
  let node = &AstNode::Satisfies(Box::new(AstNode::Boolean(true)));
  eqd(r#"Satisfies(Boolean(true))"#, node);
  eqs(
    r#"
       Satisfies
       └─ Boolean
          └─ `true`
    "#,
    node,
  );
}

#[test]
fn test_node_some() {
  let node = &AstNode::Some(b_num!(s!(1), s!()), Box::new(AstNode::List(vec![_num!(s!(1), s!())])));
  eqd(r#"Some(Numeric("1", "", '+', ""), List([Numeric("1", "", '+', "")]))"#, node);
  eqs(
    r#"
       Some
       ├─ Numeric
       │  └─ `1`
       └─ List
          └─ Numeric
             └─ `1`
    "#,
    node,
  );
}

#[test]
fn test_node_string() {
  let node = &AstNode::String(s!("alpha"));
  eqd(r#"String("alpha")"#, node);
  eqs(
    r#"
       String
       └─ `alpha`
    "#,
    node,
  );
}

#[test]
fn test_node_sub() {
  let node = &AstNode::Sub(b_num!(s!(1), s!(23)), b_num!(s!(2), s!(34)));
  eqd(r#"Sub(Numeric("1", "23", '+', ""), Numeric("2", "34", '+', ""))"#, node);
  eqs(
    r#"
       Sub
       ├─ Numeric
       │  └─ `1.23`
       └─ Numeric
          └─ `2.34`
    "#,
    node,
  );
}

#[test]
fn test_node_unary_ge() {
  let node = &AstNode::UnaryGe(Box::new(AstNode::String(s!("a"))));
  eqd(r#"UnaryGe(String("a"))"#, node);
  eqs(
    r#"
       UnaryGe
       └─ String
          └─ `a`
    "#,
    node,
  );
}

#[test]
fn test_node_unary_gt() {
  let node = &AstNode::UnaryGt(Box::new(AstNode::String(s!("a"))));
  eqd(r#"UnaryGt(String("a"))"#, node);
  eqs(
    r#"
       UnaryGt
       └─ String
          └─ `a`
    "#,
    node,
  );
}

#[test]
fn test_node_unary_le() {
  let node = &AstNode::UnaryLe(Box::new(AstNode::String(s!("a"))));
  eqd(r#"UnaryLe(String("a"))"#, node);
  eqs(
    r#"
       UnaryLe
       └─ String
          └─ `a`
    "#,
    node,
  );
}

#[test]
fn test_node_unary_lt() {
  let node = &AstNode::UnaryLt(Box::new(AstNode::String(s!("a"))));
  eqd(r#"UnaryLt(String("a"))"#, node);
  eqs(
    r#"
       UnaryLt
       └─ String
          └─ `a`
    "#,
    node,
  );
}

#[test]
fn test_node_unary_eq() {
  let node = &AstNode::UnaryEq(Box::new(AstNode::String(s!("a"))));
  eqd(r#"UnaryEq(String("a"))"#, node);
  eqs(
    r#"
       UnaryEq
       └─ String
          └─ `a`
    "#,
    node,
  );
}

#[test]
fn test_node_unary_ne() {
  let node = &AstNode::UnaryNe(Box::new(AstNode::String(s!("a"))));
  eqd(r#"UnaryNe(String("a"))"#, node);
  eqs(
    r#"
       UnaryNe
       └─ String
          └─ `a`
    "#,
    node,
  );
}
