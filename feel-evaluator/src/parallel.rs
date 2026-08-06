//! Decides whether the body of a `for` expression can run in parallel.
//!
//! # Why an analysis is needed at all
//!
//! `for` binds the implicit variable `partial` to the results of all previous iterations.
//! An iteration that reads `partial` depends on every earlier iteration. It cannot be
//! reordered or run concurrently with them. A body that never reads `partial` has no such
//! dependency.
//!
//! Deciding that statically is **not** as simple as looking for the name. An earlier
//! attempt to do exactly that produced a silently wrong answer. `eval_function_definition`
//! pushes the closure and parameter contexts onto the **live** scope stack. `FEEL` name
//! resolution inside a called function is therefore dynamic. A function defined outside
//! the loop can read the loop's `partial`, even though the body never mentions it.
//!
//! ```text
//! { f: function(z) count(partial), r: for x in [1,2,3] return f(1) }.r  ->  [0, 1, 2]
//! ```
//!
//! The body here is just `f(1)`.
//!
//! # The rule
//!
//! A body qualifies only when **all** of these hold. Each one closes a way for a hidden
//! `partial` read to exist.
//!
//! 1. No name anywhere in the body is `partial`. The direct case.
//! 2. No [AstNode::FunctionDefinition] anywhere in the body. A function defined in the body
//!    could be invoked with the live scope on the stack.
//! 3. Every [AstNode::FunctionInvocation] calls a plain name that parses as a built-in
//!    function. A user-defined function or a `BKM` invocation reaches the live scope. A
//!    built-in does not.
//! 4. No name bound inside the body parses as a built-in function name. Without this, a
//!    context entry such as `{ contains: someOuterFunction, v: contains(1) }` would satisfy
//!    rule 3 while actually calling a user-defined function.
//! 5. No [AstNode::Filter] in the body has a predicate that invokes anything. Evaluating a
//!    filter pushes the filtered element's **own entries** onto the live scope. So when an
//!    element is a context, its keys become names the predicate can call. Rules 1 to 4 read
//!    the body as written and [ParallelSafety::is_safe_in] reads the scope as it stands when
//!    the `for` begins. Neither can see a binding that arrives later, from data.
//!
//! 6. No loop variable of the `for` shares a name with a callee invoked in the body. This one
//!    is checked in `build_for` rather than here, because [analyze] only ever sees the body.
//!    The loop variables come from the `for` expression's left side. See
//!    [ParallelSafety::forbid_names].
//!
//! Rule 5 is what makes rules 3 and 4 hold up. Without it, a list element such as
//! `{ sum: function(x) count(partial), v: 1 }` binds `sum` to a function that reads `partial`.
//! A predicate calling `sum(item)` then passes rule 3 as a built-in call. The serial path
//! binds `partial` and the parallel path does not, so the two disagree. `count(null)`
//! returns 1, and the predicate becomes true for every element.
//!
//! Rule 3 is sound for built-ins. Only three of them evaluate a `FEEL` expression at
//! all — `list replace`, `sort` and `range`. Each of the three builds a **fresh** scope
//! (`let scope: FeelScope = ctx.into()`), so none of them can observe the caller's stack.
//! Reading `bifs/core.rs` confirms this.
//!
//! Rule 3 still leaves one hole that no static rule can close. A name that parses as a
//! built-in may be bound to a function **in an enclosing scope** rather than in the body.
//! [ParallelSafety::callees] carries the invoked names, so the caller can check the live
//! scope once per `for` evaluation. [ParallelSafety::is_safe_in] performs that check.
//!
//! Everything here is conservative: when in doubt the answer is "not safe" and the serial
//! path runs.

use dsntk_feel::bif::Bif;
use dsntk_feel::{FeelScope, Name};
use dsntk_feel_parser::AstNode;
use std::collections::BTreeSet;
use std::str::FromStr;

/// Returns `true` when `name` is the name of a built-in function.
fn is_builtin(name: &Name) -> bool {
  Bif::from_str(&name.to_string()).is_ok()
}

/// Returns `true` when `node` contains a function invocation anywhere below it.
///
/// Used by rule 5. The callee is not inspected: a name that resolves to a built-in when read
/// statically may resolve to an element's entry once the filter pushes it.
fn contains_invocation(node: &AstNode) -> bool {
  matches!(node, AstNode::FunctionInvocation(..)) || children(node).into_iter().any(contains_invocation)
}

/// The result of analyzing the body of a `for` expression.
#[derive(Debug, Clone, Default)]
pub struct ParallelSafety {
  /// `true` when the static rules 1 to 4 all hold.
  statically_safe: bool,
  /// Names invoked as functions in the body. All of them parse as built-in function names,
  /// so each must also be unbound in the live scope, or the call is not a built-in call.
  callees: BTreeSet<Name>,
}

impl ParallelSafety {
  /// Refuses parallel evaluation when a loop variable shares a name with an invoked callee.
  ///
  /// Rule 6, and the reason it cannot live in [analyze]: the loop variables come from the
  /// `for` expression's left side, which [analyze] never sees. It reads only the body.
  ///
  /// A loop variable binds a **data value** to a name the model author chose, so that name
  /// can spell a built-in. `for sum in fs return sum(1)` satisfies rules 1 to 5: `sum`
  /// parses as a built-in, nothing inside the body binds it, and there is no filter. It
  /// also satisfies [Self::is_safe_in], because at loop entry `sum` is not bound yet. Then
  /// each iteration binds it to an element. If that element is a function reading
  /// `partial`, the serial path binds `partial` and the parallel path does not, and the
  /// two disagree.
  ///
  /// The intersection is exact rather than conservative. Rule 3 already forces every
  /// callee to be a plain name. So the only way to invoke a loop-bound value is through a
  /// callee name equal to a loop-variable name.
  pub fn forbid_names(&mut self, names: &std::collections::HashSet<Name>) {
    if self.callees.iter().any(|callee| names.contains(callee)) {
      self.statically_safe = false;
    }
  }

  /// Returns `true` when this body can run in parallel in the given scope.
  ///
  /// This checks the static rules only once, at evaluator build time. This adds one
  /// check that needs a scope: an enclosing scope must not shadow any of the invoked
  /// built-in names. It runs once per `for` evaluation, not once per element.
  pub fn is_safe_in(&self, scope: &FeelScope) -> bool {
    self.statically_safe && !self.callees.iter().any(|name| scope.contains_name(name))
  }
}

/// Analyzes the body of a `for` expression.
pub fn analyze(body: &AstNode) -> ParallelSafety {
  let mut state = Analysis {
    unsafe_found: false,
    callees: BTreeSet::new(),
    bound: BTreeSet::new(),
  };
  state.walk(body);
  // Rule 4: no name bound inside the body may parse as a built-in function name.
  let shadows_builtin = state.bound.iter().any(is_builtin);
  ParallelSafety {
    statically_safe: !state.unsafe_found && !shadows_builtin,
    callees: state.callees,
  }
}

struct Analysis {
  /// Set when the body violates rule 1, 2 or 3.
  unsafe_found: bool,
  callees: BTreeSet<Name>,
  bound: BTreeSet<Name>,
}

impl Analysis {
  fn note_name(&mut self, name: &Name) {
    // Rule 1.
    if name.to_string() == "partial" {
      self.unsafe_found = true;
    }
  }

  fn walk(&mut self, node: &AstNode) {
    if self.unsafe_found {
      return;
    }
    match node {
      // Rule 2. A function defined in the body may be invoked with the live scope pushed.
      AstNode::FunctionDefinition(..) => {
        self.unsafe_found = true;
        return;
      }
      // Rule 3 allows only a call to a plain built-in name.
      AstNode::FunctionInvocation(callee, arguments) => {
        match callee.as_ref() {
          AstNode::Name(name) if is_builtin(name) => {
            self.callees.insert(name.clone());
          }
          // Anything else is a user-defined function, a BKM, or a computed callee.
          _ => {
            self.unsafe_found = true;
            return;
          }
        }
        self.walk(arguments);
        return;
      }
      // Rule 5. A filter pushes the filtered element's own entries onto the live scope. So a
      // predicate that calls anything may be calling a name that arrived from data. The list
      // being filtered is still walked: only the predicate sees the element's entries.
      AstNode::Filter(list, predicate) => {
        if contains_invocation(predicate) {
          self.unsafe_found = true;
          return;
        }
        self.walk(list);
        self.walk(predicate);
        return;
      }
      // Names that a body binds. Collected for rule 4.
      AstNode::ContextEntryKey(name) | AstNode::ContextTypeEntryKey(name) | AstNode::ParameterName(name) => {
        self.note_name(name);
        self.bound.insert(name.clone());
        return;
      }
      // Iteration and quantifier variables bind their first child name.
      AstNode::IterationContextSingle(variable, _) | AstNode::IterationContextInterval(variable, _, _) | AstNode::QuantifiedContext(variable, _) => {
        if let AstNode::Name(name) = variable.as_ref() {
          self.bound.insert(name.clone());
        }
      }
      // A formal parameter binds its first child name.
      AstNode::FormalParameter(variable, _) => {
        if let AstNode::Name(name) = variable.as_ref() {
          self.bound.insert(name.clone());
        }
      }
      // Plain name references.
      AstNode::Name(name) | AstNode::QualifiedNameSegment(name) => {
        self.note_name(name);
        return;
      }
      _ => {}
    }
    for child in children(node) {
      self.walk(child);
      if self.unsafe_found {
        return;
      }
    }
  }
}

/// Returns every child node of `node`.
///
/// This match is deliberately **exhaustive with no catch-all arm**. If a future version of
/// the grammar adds an [AstNode] variant, this function stops compiling. That is the point.
/// A future maintainer must classify a new variant by hand, rather than let it silently
/// default to having no children. An unclassified variant would make the analysis above
/// unsound.
fn children(node: &AstNode) -> Vec<&AstNode> {
  match node {
    // two children
    AstNode::Add(a, b)
    | AstNode::And(a, b)
    | AstNode::ContextEntry(a, b)
    | AstNode::ContextTypeEntry(a, b)
    | AstNode::Div(a, b)
    | AstNode::Eq(a, b)
    | AstNode::Every(a, b)
    | AstNode::Exp(a, b)
    | AstNode::Filter(a, b)
    | AstNode::For(a, b)
    | AstNode::FormalParameter(a, b)
    | AstNode::FunctionDefinition(a, b)
    | AstNode::FunctionInvocation(a, b)
    | AstNode::FunctionType(a, b)
    | AstNode::Ge(a, b)
    | AstNode::Gt(a, b)
    | AstNode::In(a, b)
    | AstNode::InstanceOf(a, b)
    | AstNode::IterationContextSingle(a, b)
    | AstNode::Le(a, b)
    | AstNode::Lt(a, b)
    | AstNode::Mul(a, b)
    | AstNode::NamedParameter(a, b)
    | AstNode::Nq(a, b)
    | AstNode::Or(a, b)
    | AstNode::Out(a, b)
    | AstNode::Path(a, b)
    | AstNode::QuantifiedContext(a, b)
    | AstNode::Range(a, b)
    | AstNode::Some(a, b)
    | AstNode::Sub(a, b) => vec![a, b],
    // three children
    AstNode::Between(a, b, c) | AstNode::If(a, b, c) | AstNode::IterationContextInterval(a, b, c) => vec![a, b, c],
    // one child
    AstNode::EvaluatedExpression(a)
    | AstNode::FunctionBody(a, _)
    | AstNode::IntervalEnd(a, _)
    | AstNode::IntervalStart(a, _)
    | AstNode::ListType(a)
    | AstNode::Neg(a)
    | AstNode::RangeType(a)
    | AstNode::Satisfies(a)
    | AstNode::UnaryEq(a)
    | AstNode::UnaryGe(a)
    | AstNode::UnaryGt(a)
    | AstNode::UnaryLe(a)
    | AstNode::UnaryLt(a)
    | AstNode::UnaryNe(a) => vec![a],
    // a list of children
    AstNode::CommaList(items)
    | AstNode::Context(items)
    | AstNode::ContextType(items)
    | AstNode::ExpressionList(items)
    | AstNode::FormalParameters(items)
    | AstNode::IterationContexts(items)
    | AstNode::List(items)
    | AstNode::NamedParameters(items)
    | AstNode::NegatedList(items)
    | AstNode::ParameterTypes(items)
    | AstNode::PositionalParameters(items)
    | AstNode::QualifiedName(items)
    | AstNode::QuantifiedContexts(items) => items.iter().collect(),
    // leaves
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
    | AstNode::String(_) => vec![],
  }
}
