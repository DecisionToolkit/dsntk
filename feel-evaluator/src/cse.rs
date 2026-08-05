//! Finds a repeated function call inside a filter predicate so it can be evaluated once.
//!
//! # What this is for
//!
//! A rule in the benchmark model writes its window test like this:
//!
//! ```text
//! wins[inSum(item) >= 6000 and outSum(item) >= 6000 and outSum(item) > 0
//!      and inSum(item) / outSum(item) >= 0.9 and inSum(item) / outSum(item) <= 1.1]
//! ```
//!
//! `inSum` is called three times and `outSum` four times for **every** window, and each call
//! filters the whole transaction list and folds it. Two evaluations answer the same question.
//! The Java reference computes exactly two sums per window.
//!
//! # Why a build-time transform and not a runtime cache
//!
//! A memo cache has to key on the argument **values**, so it has to hash a [Value]. Measured
//! per-variant clone costs put a `String` at 18.61 ns and an eleven-element `List` at 61.70 ns,
//! and hashing costs the same order of magnitude. A cache would therefore spend a large part
//! of what it saves. The repeated calls here are syntactically identical subtrees evaluated in
//! one unchanged scope. Binding each distinct subtree to a synthetic name once per predicate
//! evaluation therefore needs no hashing at all.
//!
//! # Purity, enumerated rather than excluded
//!
//! Hoisting a call from seven evaluations to two is only sound when the call is deterministic
//! and effect-free. Nothing here trusts a default. A subtree qualifies only when every rule
//! below holds, and each rule is a list of what is **allowed**:
//!
//! 1. The subtree is a call of a plain name that is **not** a built-in. That single test
//!    excludes `now`, `today` and every other clock-derived built-in at once, rather than
//!    relying on a list of things to avoid. `bif_now` is already known to be defective, so
//!    this analysis does not treat the built-in table as trustworthy.
//! 2. Every argument is a plain name or a literal, so evaluating the arguments cannot itself
//!    change anything.
//! 3. A sibling entry of the enclosing context binds the called name to a function
//!    definition that is **not external**. So no `Java` or `PMML` implementation is reached.
//! 4. That function's body contains only calls of built-ins on the allowed list below, or of
//!    other functions that pass this same test.
//! 5. Every occurrence sits on the predicate's non-binding spine. The walk refuses to enter
//!    `for`, `some`, `every`, a context literal, a nested filter or a function definition.
//!    So nothing hoists an occurrence out of a construct that binds a name it reads.

use dsntk_feel::Name;
use dsntk_feel::bif::Bif;
use dsntk_feel_parser::AstNode;
use std::collections::HashMap;
use std::str::FromStr;

/// Built-in functions allowed inside the body of a hoisted function.
///
/// Enumerated, so a built-in missing from this list blocks the hoist instead of being
/// silently permitted. `now` and `today` are absent because they read the clock. Three of
/// these evaluate a `FEEL` expression: `list replace`, `sort` and `range`. Each builds a
/// fresh scope rather than reading the caller's, so none can observe the scope the hoist
/// moves the call out of.
const ALLOWED_BUILTINS: &[&str] = &[
  "abs",
  "after",
  "all",
  "any",
  "append",
  "before",
  "ceiling",
  "coincides",
  "concatenate",
  "contains",
  "context",
  "context merge",
  "context put",
  "count",
  "date",
  "date and time",
  "day of week",
  "day of year",
  "decimal",
  "distinct values",
  "duration",
  "during",
  "ends with",
  "even",
  "exp",
  "finished by",
  "finishes",
  "flatten",
  "floor",
  "get entries",
  "get value",
  "includes",
  "index of",
  "is",
  "list contains",
  "list replace",
  "log",
  "lower case",
  "matches",
  "max",
  "mean",
  "median",
  "meets",
  "met by",
  "min",
  "mode",
  "modulo",
  "month of year",
  "not",
  "number",
  "odd",
  "overlaps",
  "overlaps after",
  "overlaps before",
  "product",
  "range",
  "remove",
  "replace",
  "reverse",
  "round down",
  "round half down",
  "round half up",
  "round up",
  "sort",
  "split",
  "sqrt",
  "started by",
  "starts",
  "starts with",
  "stddev",
  "string",
  "string join",
  "string length",
  "sublist",
  "substring",
  "substring after",
  "substring before",
  "sum",
  "time",
  "union",
  "upper case",
  "week of year",
  "years and months duration",
];

/// How deep the purity walk follows one function calling another before giving up.
const MAX_PURITY_DEPTH: usize = 8;

/// The name bound to hoisted slot `index`.
///
/// A `#` cannot appear in a `FEEL` name, so a slot can never shadow a name the author wrote.
pub fn slot_name(index: usize) -> Name {
  Name::from(format!("#cse{index}").as_str())
}

/// Sibling context entries that bind a name to a function definition.
pub type LocalFunctions<'b> = HashMap<Name, &'b AstNode>;

/// Records every sibling entry of a context literal that binds a name to a function
/// definition, so a call of that name inside a later entry can be checked for purity.
pub fn collect_local_functions<'b>(entries: &'b [AstNode], into: &mut LocalFunctions<'b>) {
  for entry in entries {
    if let AstNode::ContextEntry(key, value) = entry
      && let AstNode::ContextEntryKey(name) = key.as_ref()
      && matches!(value.as_ref(), AstNode::FunctionDefinition(..))
    {
      into.insert(name.clone(), value.as_ref());
    }
  }
}

/// Returns the repeated subtrees of `predicate` that the transform can evaluate once instead
/// of once per occurrence, in the order they first appear.
pub fn hoistable<'b>(predicate: &'b AstNode, locals: &LocalFunctions<'b>) -> Vec<&'b AstNode> {
  let mut found: Vec<(&'b AstNode, usize)> = vec![];
  collect_calls(predicate, &mut found);
  let mut hoists = vec![];
  for (node, count) in found {
    if count >= 2 && is_hoistable_call(node, locals) {
      hoists.push(node);
    }
  }
  hoists
}

/// Walks the non-binding spine of `predicate`, counting structurally equal calls.
///
/// The `_ => {}` arm is the safety property. This function does not enter any node kind not
/// listed here. So a construct that binds a name -- `for`, `some`, `every`, a context
/// literal, a nested filter, a function definition -- hides everything inside it from the
/// hoist.
fn collect_calls<'b>(node: &'b AstNode, found: &mut Vec<(&'b AstNode, usize)>) {
  if let AstNode::FunctionInvocation(..) = node {
    if let Some(slot) = found.iter_mut().find(|(seen, _)| *seen == node) {
      slot.1 += 1;
    } else {
      found.push((node, 1));
    }
    // The arguments of a call are names or literals when it qualifies, so there is nothing
    // below a call worth walking.
    return;
  }
  match node {
    AstNode::Add(lhs, rhs)
    | AstNode::And(lhs, rhs)
    | AstNode::Div(lhs, rhs)
    | AstNode::Eq(lhs, rhs)
    | AstNode::Exp(lhs, rhs)
    | AstNode::Ge(lhs, rhs)
    | AstNode::Gt(lhs, rhs)
    | AstNode::In(lhs, rhs)
    | AstNode::InstanceOf(lhs, rhs)
    | AstNode::Le(lhs, rhs)
    | AstNode::Lt(lhs, rhs)
    | AstNode::Mul(lhs, rhs)
    | AstNode::Nq(lhs, rhs)
    | AstNode::Or(lhs, rhs)
    | AstNode::Sub(lhs, rhs) => {
      collect_calls(lhs, found);
      collect_calls(rhs, found);
    }
    AstNode::Between(a, b, c) | AstNode::If(a, b, c) => {
      collect_calls(a, found);
      collect_calls(b, found);
      collect_calls(c, found);
    }
    AstNode::Neg(lhs) | AstNode::EvaluatedExpression(lhs) => collect_calls(lhs, found),
    AstNode::ExpressionList(items) | AstNode::CommaList(items) | AstNode::List(items) => {
      for item in items {
        collect_calls(item, found);
      }
    }
    _ => {}
  }
}

/// Returns `true` when this call may be hoisted. Rules 1 to 4 of the module note.
fn is_hoistable_call<'b>(node: &'b AstNode, locals: &LocalFunctions<'b>) -> bool {
  let AstNode::FunctionInvocation(callee, parameters) = node else {
    return false;
  };
  // Rule 1: a plain name that is not a built-in of any kind.
  let AstNode::Name(name) = callee.as_ref() else {
    return false;
  };
  if Bif::from_str(name.as_str()).is_ok() {
    return false;
  }
  // Rule 2: every argument is a plain name or a literal.
  let AstNode::PositionalParameters(arguments) = parameters.as_ref() else {
    return false;
  };
  if !arguments.iter().all(is_trivial_argument) {
    return false;
  }
  // Rules 3 and 4: the name is bound by a sibling entry to a pure, non-external function.
  let Some(definition) = locals.get(name) else {
    return false;
  };
  is_pure_function(definition, locals, 0)
}

/// Returns `true` for an argument whose evaluation cannot change anything.
fn is_trivial_argument(node: &AstNode) -> bool {
  matches!(node, AstNode::Name(_) | AstNode::Numeric(..) | AstNode::String(_) | AstNode::Boolean(_) | AstNode::Null)
}

/// Returns `true` when this function definition is not external and its body calls only
/// allowed built-ins and other pure functions.
fn is_pure_function<'b>(definition: &'b AstNode, locals: &LocalFunctions<'b>, depth: usize) -> bool {
  if depth > MAX_PURITY_DEPTH {
    return false;
  }
  let AstNode::FunctionDefinition(_, body) = definition else {
    return false;
  };
  let AstNode::FunctionBody(inner, external) = body.as_ref() else {
    return false;
  };
  // Rule 3: no external `Java` or `PMML` implementation.
  if *external {
    return false;
  }
  is_pure_expression(inner, locals, depth)
}

/// Returns `true` when every call anywhere inside `node` is allowed.
///
/// Unlike [collect_calls] this walks the whole subtree, including binding constructs. Here
/// the question is not where a call sits, but whether any call reachable from it is impure.
/// `AstNode::children` is not available, so this module writes the walk by hand over the
/// variants that can hold an expression.
fn is_pure_expression<'b>(node: &'b AstNode, locals: &LocalFunctions<'b>, depth: usize) -> bool {
  if let AstNode::FunctionInvocation(callee, parameters) = node {
    let allowed = match callee.as_ref() {
      AstNode::Name(name) => {
        if let Ok(_bif) = Bif::from_str(name.as_str()) {
          ALLOWED_BUILTINS.contains(&name.as_str())
        } else if let Some(definition) = locals.get(name) {
          is_pure_function(definition, locals, depth + 1)
        } else {
          // An unknown callee. It may be a parameter holding a function, or a name bound
          // further out than this context, and neither can be checked here.
          false
        }
      }
      // A call of anything other than a plain name cannot be checked.
      _ => false,
    };
    return allowed && is_pure_children(parameters, locals, depth);
  }
  is_pure_children(node, locals, depth)
}

/// Applies [is_pure_expression] to every child of `node`.
fn is_pure_children<'b>(node: &'b AstNode, locals: &LocalFunctions<'b>, depth: usize) -> bool {
  let mut pure = true;
  let mut check = |child: &'b AstNode| {
    if pure {
      pure = is_pure_expression(child, locals, depth);
    }
  };
  match node {
    // Leaves.
    AstNode::At(_)
    | AstNode::Boolean(_)
    | AstNode::ContextEntryKey(_)
    | AstNode::ContextTypeEntryKey(_)
    | AstNode::FeelType(_)
    | AstNode::Irrelevant
    | AstNode::Name(_)
    | AstNode::Null
    | AstNode::Numeric(..)
    | AstNode::ParameterName(_)
    | AstNode::QualifiedNameSegment(_)
    | AstNode::String(_) => {}
    // Two children.
    AstNode::Add(a, b)
    | AstNode::And(a, b)
    | AstNode::ContextEntry(a, b)
    | AstNode::Div(a, b)
    | AstNode::Eq(a, b)
    | AstNode::Exp(a, b)
    | AstNode::Filter(a, b)
    | AstNode::FunctionDefinition(a, b)
    | AstNode::FunctionInvocation(a, b)
    | AstNode::Ge(a, b)
    | AstNode::Gt(a, b)
    | AstNode::In(a, b)
    | AstNode::InstanceOf(a, b)
    | AstNode::Le(a, b)
    | AstNode::Lt(a, b)
    | AstNode::Mul(a, b)
    | AstNode::NamedParameter(a, b)
    | AstNode::Nq(a, b)
    | AstNode::Or(a, b)
    | AstNode::Path(a, b)
    | AstNode::Every(a, b)
    | AstNode::For(a, b)
    | AstNode::Some(a, b)
    | AstNode::IterationContextSingle(a, b)
    | AstNode::Out(a, b)
    | AstNode::QuantifiedContext(a, b)
    | AstNode::Sub(a, b) => {
      check(a);
      check(b);
    }
    // Three children.
    AstNode::Between(a, b, c) | AstNode::If(a, b, c) => {
      check(a);
      check(b);
      check(c);
    }
    // One child.
    AstNode::EvaluatedExpression(a)
    | AstNode::FormalParameter(a, _)
    | AstNode::FunctionBody(a, _)
    | AstNode::IntervalEnd(a, _)
    | AstNode::IntervalStart(a, _)
    | AstNode::Neg(a)
    | AstNode::Satisfies(a)
    | AstNode::UnaryEq(a)
    | AstNode::UnaryGe(a)
    | AstNode::UnaryGt(a)
    | AstNode::UnaryLe(a)
    | AstNode::UnaryLt(a)
    | AstNode::UnaryNe(a) => check(a),
    // A list of children.
    AstNode::CommaList(items)
    | AstNode::Context(items)
    | AstNode::ExpressionList(items)
    | AstNode::FormalParameters(items)
    | AstNode::IterationContexts(items)
    | AstNode::List(items)
    | AstNode::NamedParameters(items)
    | AstNode::NegatedList(items)
    | AstNode::PositionalParameters(items)
    | AstNode::QualifiedName(items)
    | AstNode::QuantifiedContexts(items) => {
      for item in items {
        check(item);
      }
    }
    // Anything else is a type expression or a shape this analysis does not model, so refuse.
    _ => return false,
  }
  pure
}
