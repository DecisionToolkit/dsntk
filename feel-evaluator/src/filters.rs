//! # FEEL filter implementation

use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values};
use dsntk_feel::{Evaluator, FeelNumber, FeelScope, Name, value_null};
use dsntk_feel_parser::AstNode;
use std::sync::LazyLock;

/// The name `item`, interned once.
///
/// `"item".into()` interns the text, and interning takes a read acquisition on the one
/// process-wide `RwLock` that guards the name table. The evaluator runs a filter once
/// per filter expression per row, from every Rayon worker. That acquisition is
/// therefore an atomic read-modify-write on a cache line every worker shares. A
/// `LazyLock` is an acquire load on a line that is read-only after the first use.
static NAME_ITEM: LazyLock<Name> = LazyLock::new(|| "item".into());

/// What a build-time walk proved about a filter's predicate, so that `L[i]` need not visit
/// every element of `L`.
///
/// # The cost this removes
///
/// `L[i]` and `L[i + 3]` are indexes, not filters, but a filter is the only machinery `FEEL`
/// has for them. [FilterExpressionEvaluator::evaluate] runs the predicate for **every** element
/// and only afterwards evaluates it once more to discover the result was an index. Measured at
/// 43 ns per element per index. So a prefix pass that indexes inside its own loop is cubic in
/// the list length. A two-stage prefix sum over 1,825 days measured **125.7 s**, where a
/// one-pass form measured 0.4 s.
///
/// # The one argument the fast path rests on
///
/// The loop's final evaluation already decides index against filter with **no** element context
/// pushed. So assume every per-element evaluation returns the same value as that final one,
/// and that value is a number. Two things follow. Each per-element evaluation returns a
/// number, which the loop discards, so the list it builds is empty and unused. And the index
/// it takes is the index the fast path takes. The two paths agree element for element and
/// message for message.
///
/// What could make a per-element evaluation differ is only what the loop binds:
///
/// 1. `item`, bound for every element.
/// 2. the element's own entries, pushed when the element is a `Value::Context`, so a bare name
///    in the predicate can resolve to a **field of the element**.
///
/// Both are name resolution. A predicate that reads no name at all is immune. A predicate
/// that reads names other than `item` is immune once no element is a context. This says
/// nothing about what the predicate computes. That is why it does not matter that `+` can
/// return a string, a date or a duration as well as a number.
///
/// **Nothing is skipped unless the evaluation returns a number.** Every other result falls
/// through to the loop, which then does exactly what it does today. That includes returning
/// an error null when a predicate yields a value a filter refuses.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndexShape {
  /// Nothing proved. Every element runs the predicate, exactly as before.
  Unknown,
  /// Reads no name, so neither `item` nor an element's fields can reach it. Holds whatever the
  /// elements are, so `fired[1]` needs no check at all.
  Closed,
  /// Reads only plain names, none of them `item`. Sound once no element is a context, which is
  /// one discriminant test per element with no allocation and no scope push.
  PlainNames,
}

/// Classifies a filter predicate. See [IndexShape] for what each answer licenses.
///
/// Enumerated, not excluded: a node kind absent from these arms answers [IndexShape::Unknown]
/// and keeps the element loop. This function admits only literals, plain names and the
/// arithmetic operators. So nothing in an admitted predicate can invoke a function, evaluate
/// a nested filter, build a context, or write through the scope. `EvaluatedExpression` is
/// transparent because `build_evaluated_expression` builds its child and calls it, adding
/// nothing.
pub fn index_shape(node: &AstNode) -> IndexShape {
  match node {
    AstNode::EvaluatedExpression(inner) | AstNode::Neg(inner) => index_shape(inner),
    AstNode::Numeric(..) => IndexShape::Closed,
    AstNode::Add(lhs, rhs) | AstNode::Sub(lhs, rhs) | AstNode::Mul(lhs, rhs) | AstNode::Div(lhs, rhs) | AstNode::Exp(lhs, rhs) => {
      match (index_shape(lhs), index_shape(rhs)) {
        (IndexShape::Unknown, _) | (_, IndexShape::Unknown) => IndexShape::Unknown,
        (IndexShape::Closed, IndexShape::Closed) => IndexShape::Closed,
        _ => IndexShape::PlainNames,
      }
    }
    // `item` is bound for every element, so a predicate reading it is genuinely per-element.
    AstNode::Name(name) => {
      if *name == *NAME_ITEM {
        IndexShape::Unknown
      } else {
        IndexShape::PlainNames
      }
    }
    _ => IndexShape::Unknown,
  }
}

/// Reads element `index` of `values`, counting from 1, or from the end when negative.
///
/// Lifted out of [FilterExpressionEvaluator::evaluate] unchanged so that the fast path and the
/// loop path share one copy, including every message.
fn index_into(values: &Values, index: FeelNumber) -> Value {
  if index.is_integer() {
    let list_size = values.len();
    if list_size > 0 {
      if !index.is_negative() {
        let n = {
          if let Ok(u_index) = usize::try_from(index) {
            u_index
          } else {
            return value_null!("index is out of range 1..2⁶⁴: {}", index.to_string());
          }
        };
        if n > 0 && n <= list_size {
          // unwrap below is safe, index `n` is checked above, `values` variable is immutable
          values.get(n - 1).unwrap().to_owned()
        } else {
          value_null!("index in filter is out of range [1..{}], actual index is {}", list_size, n)
        }
      } else {
        let n = {
          if let Ok(u_index) = usize::try_from(index.abs()) {
            u_index
          } else {
            return value_null!("index is out of range 1..2⁶⁴: {}", index.to_string());
          }
        };
        if n > 0 && n <= list_size {
          // unwrap below is safe, index `n` is checked above, `values` variable is immutable
          values.get(list_size - n).unwrap().to_owned()
        } else {
          value_null!("index in filter is out of range [-{}..-1], actual index is -{}", list_size, n)
        }
      }
    } else {
      // return null when the list is empty, no matter what value the index has
      value_null!()
    }
  } else {
    value_null!("index in filter must be an integer value, actual value is {}", index)
  }
}

pub struct FilterExpressionEvaluator {}

impl Default for FilterExpressionEvaluator {
  /// Implements [Default] trait for [FilterExpressionEvaluator].
  fn default() -> Self {
    Self::new()
  }
}

impl FilterExpressionEvaluator {
  pub fn new() -> Self {
    Self {}
  }

  pub fn evaluate(&self, scope: &FeelScope, value: Value, evaluator: &Evaluator, shape: IndexShape) -> Value {
    let name_item: &Name = &NAME_ITEM;
    match value {
      Value::List(values) => {
        // An index, not a filter: answer it without visiting an element. [IndexShape] carries
        // what the build-time walk proved and states why each answer is enough. An empty list is
        // left to the loop path, which costs nothing there and reports a bare null rather than an
        // out-of-range message.
        if !values.is_empty()
          && match shape {
            IndexShape::Unknown => false,
            IndexShape::Closed => true,
            // The one remaining way the loop could resolve a name differently is an element
            // that is itself a context. The loop pushes that element's entries over the name
            // being read.
            IndexShape::PlainNames => !values.iter().any(|value| matches!(value, Value::Context(_))),
          }
          && let Value::Number(index) = evaluator(scope)
        {
          return index_into(&values, index);
        }
        let mut filtered_values = vec![];
        for value in &values {
          let (added_local_context, has_item_entry) = if let Value::Context(local_context) = value {
            scope.push(local_context.clone());
            if local_context.contains_entry(name_item) { (true, true) } else { (true, false) }
          } else {
            (false, false)
          };
          if !has_item_entry {
            // One entry, built and then shared. Building it with `default()` and `set_entry`
            // instead runs `Arc::make_mut` on a static every thread shares. See
            // [FeelContext::from_entry].
            scope.push(FeelContext::from_entry(name_item, value.clone()));
          }
          let rhv = evaluator(scope);
          match rhv {
            Value::Boolean(true) => filtered_values.push(value.clone()),
            Value::Boolean(false) | Value::Number(_) | Value::Null(_) => {}
            _ => return value_null!("only number or boolean indexes are allowed in filters"),
          }
          if !has_item_entry {
            scope.pop();
          }
          if added_local_context {
            scope.pop();
          }
        }
        let rhv = evaluator(scope);
        match rhv {
          Value::Number(index) => index_into(&values, index),
          _ => Value::List(filtered_values),
        }
      }
      v @ Value::Number(_)
      | v @ Value::Boolean(_)
      | v @ Value::String(_)
      | v @ Value::Date(_)
      | v @ Value::DateTime(_)
      | v @ Value::Time(_)
      | v @ Value::DaysAndTimeDuration(_)
      | v @ Value::YearsAndMonthsDuration(_)
      | v @ Value::Context(_) => match evaluator(scope) {
        Value::Boolean(flag) => {
          if flag {
            Value::List(vec![v])
          } else {
            Value::List(Values::default())
          }
        }
        Value::Number(num) => {
          if num.is_one() || (-num).is_one() {
            v
          } else {
            value_null!("for singletons, only filter index with value 1 or -1 is accepted")
          }
        }
        _ => value_null!("only number or boolean indexes are allowed in filters"),
      },
      other => value_null!("unexpected value type in filter: {}", other.type_of()),
    }
  }
}
