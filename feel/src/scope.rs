//! Implementation of the `FEEL` scope.

use crate::context::FeelContext;
use crate::values::Value;
use crate::{Name, QualifiedName};
use dsntk_common::Jsonify;
use std::cell::RefCell;
use std::fmt;

/// This macro creates a default scope.
#[macro_export]
macro_rules! scope {
  () => {{ FeelScope::default() }};
}

/// The `FEEL` scope.
pub struct FeelScope {
  /// The stack of contexts.
  stack: RefCell<Vec<FeelContext>>,
}

impl Default for FeelScope {
  /// Creates a default [FeelScope] containing single default [FeelContext].
  fn default() -> Self {
    Self {
      stack: RefCell::new(vec![FeelContext::default()]),
    }
  }
}

impl From<FeelContext> for FeelScope {
  /// Creates a [FeelScope] from [FeelContext].
  fn from(ctx: FeelContext) -> Self {
    Self { stack: RefCell::new(vec![ctx]) }
  }
}

impl fmt::Display for FeelScope {
  /// Converts [FeelScope] to text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.stack.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl Jsonify for FeelScope {
  /// Converts this [FeelScope] to JSON text.
  fn jsonify(&self) -> String {
    format!("[{}]", self.stack.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl FeelScope {
  /// Temporary - remove
  pub fn contexts(&self) -> Vec<FeelContext> {
    self.stack.borrow().clone()
  }

  /// Creates a new and empty [FeelScope].
  pub fn new() -> Self {
    Self { stack: RefCell::new(vec![]) }
  }

  /// Pushes a context on the top of the scope stack.
  pub fn push(&self, ctx: FeelContext) {
    self.stack.borrow_mut().push(ctx)
  }

  /// Returns the number of contexts on the scope stack.
  pub fn len(&self) -> usize {
    self.stack.borrow().len()
  }

  /// Returns `true` when there are no contexts on the scope stack.
  pub fn is_empty(&self) -> bool {
    self.stack.borrow().is_empty()
  }

  /// Appends the content of another scope at the end of this scope.
  pub fn append(&self, other: FeelScope) {
    self.stack.borrow_mut().append(&mut other.stack.borrow_mut());
  }

  /// Takes and returns a context from the top of the stack.
  pub fn pop(&self) -> Option<FeelContext> {
    self.stack.borrow_mut().pop()
  }

  /// Peeks a context from the top of the stack.
  /// If the stack is empty, the default context is returned.
  pub fn peek(&self) -> Option<FeelContext> {
    self.stack.borrow().last().cloned()
  }

  /// Builds a scope from a stack of contexts.
  ///
  /// Used to give each worker of a parallel `for` its own stack. The contexts are shared,
  /// not copied: [crate::context::FeelContext] keeps its entries behind an `Arc`, so
  /// cloning the stack costs one refcount bump per level. Pushing or popping on the copy
  /// does not affect the original. Mutating a context through it copies that context
  /// first, because every `FeelContext` mutation is copy-on-write.
  ///
  /// A [FeelScope] holds a `RefCell` and so is not `Sync`, but a `Vec<FeelContext>` is both
  /// `Send` and `Sync`. So the stack is what crosses a thread boundary, and each worker
  /// builds its own scope around a clone of it.
  pub fn from_contexts(contexts: Vec<FeelContext>) -> Self {
    Self { stack: RefCell::new(contexts) }
  }

  /// Returns `true` when any context on the stack binds `name`.
  ///
  /// Unlike [FeelScope::get_value] this clones nothing, so it is usable as a guard.
  pub fn contains_name(&self, name: &Name) -> bool {
    self.stack.borrow().iter().any(|context| context.contains_entry(name))
  }

  /// Returns a value of an entry with specified name.
  /// Entries are searched from the last to the first context,
  /// (from top to bottom of the stack).
  pub fn get_value(&self, name: &Name) -> Option<Value> {
    for context in self.stack.borrow().iter().rev() {
      if let Some(value) = context.get_entry(name) {
        return Some(value.clone());
      }
    }
    None
  }

  /// Returns a clone of the entry `field` taken from the context bound to `name`.
  ///
  /// This resolves the binding of `name` exactly like [FeelScope::get_value] resolves
  /// it. The search stops at the first context that binds `name`, so an inner binding
  /// always shadows an outer one. This returns `None` when `name` is not bound, when
  /// it is not bound to a context, or when that context has no `field` entry. In all
  /// three cases the caller must fall back to evaluating the base value.
  ///
  /// This clones only the requested entry. Evaluating the base value instead clones
  /// the whole context, which costs about 2 KB per field access on a typical record.
  pub fn get_context_entry(&self, name: &Name, field: &Name) -> Option<Value> {
    for context in self.stack.borrow().iter().rev() {
      if let Some(value) = context.get_entry(name) {
        return if let Value::Context(ctx) = value { ctx.get_entry(field).cloned() } else { None };
      }
    }
    None
  }

  /// Searches for a value under so called `qualified` name build from
  /// multiple names passed as an argument.
  pub fn search(&self, names: &[Name]) -> Option<Value> {
    for context in self.stack.borrow().iter().rev() {
      if let Some(value) = context.search_deep(names) {
        return Some(value.clone());
      }
    }
    None
  }

  /// Searches for a value of an entry pointed by specified qualified name.
  pub fn search_entry(&self, qname: &QualifiedName) -> Option<Value> {
    for context in self.stack.borrow().iter().rev() {
      if let Some(value) = context.search_entry(qname) {
        return Some(value.clone());
      }
    }
    None
  }

  /// Sets a specified value for entry name in [FeelContext] placed on the top of the scope stack.
  pub fn set_value(&self, name: &Name, value: Value) {
    if let Some(context) = self.stack.borrow_mut().last_mut() {
      context.set_entry(name, value);
    }
  }

  /// Sets a null value for entry name in [FeelContext] placed on the top of the scope stack.
  pub fn set_name(&self, name: Name) {
    if let Some(context) = self.stack.borrow_mut().last_mut() {
      context.set_null(name);
    }
  }
}
