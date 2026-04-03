//! # Implementation of `for`, `some` and `every` expressions

use crate::errors::*;
use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{VALUE_FALSE, VALUE_TRUE, Value, Values};
use dsntk_feel::{Evaluator, FEEL_TYPE_NAME_DATE, FEEL_TYPE_NAME_NUMBER, FeelNumber, FeelScope, Name, value_null};
use dsntk_feel_temporal::FeelDate;

/// Common interface for all iteration state types.
trait IterationState {
  /// Assigns the iteration value pointed by binding variable.
  fn bind_value(&mut self, _ctx: &FeelContext) {}

  /// Sets the current value of the iteration in provided context.
  fn set_value(&mut self, _ctx: &mut FeelContext) {}

  /// Iterates to next value.
  fn next(&mut self) -> bool {
    false
  }

  /// Checks if there is a next value to iterate to.
  fn has_next(&self) -> bool {
    false
  }

  /// Check if there are values to iterate over.
  fn is_empty(&self) -> bool {
    false
  }

  /// Checks if the iteration state is a bounded variable.
  fn is_variable(&self) -> bool {
    false
  }
}

/// Iteration stepping direction.
#[derive(Debug)]
enum IterationDirection {
  Ascending,
  Descending,
}

/// Iteration state for number intervals.
struct NumberIntervalState {
  /// Name of the bound variable in this iterator state.
  variable: Name,
  /// Iteration starting value.
  start: FeelNumber,
  /// Iteration ending value.
  end: FeelNumber,
  /// Iteration step.
  step: FeelNumber,
  /// Iteration step direction.
  direction: IterationDirection,
  /// Current iteration value.
  current: FeelNumber,
}

impl NumberIntervalState {
  fn init(variable: Name, start: FeelNumber, end: FeelNumber) -> Box<dyn IterationState> {
    Box::new(Self {
      variable,
      start,
      end,
      step: FeelNumber::one(),
      direction: if start <= end { IterationDirection::Ascending } else { IterationDirection::Descending },
      current: start,
    })
  }
}

impl IterationState for NumberIntervalState {
  fn set_value(&mut self, ctx: &mut FeelContext) {
    ctx.set_entry(&self.variable, Value::Number(self.current));
  }

  fn next(&mut self) -> bool {
    match self.direction {
      IterationDirection::Ascending => {
        if self.current + self.step <= self.end {
          self.current += self.step;
          true
        } else {
          self.current = self.start;
          false
        }
      }
      IterationDirection::Descending => {
        if self.current - self.step >= self.end {
          self.current -= self.step;
          true
        } else {
          self.current = self.start;
          false
        }
      }
    }
  }

  fn has_next(&self) -> bool {
    match self.direction {
      IterationDirection::Ascending => self.current + self.step <= self.end,
      IterationDirection::Descending => self.current - self.step >= self.end,
    }
  }
}

/// Iteration state for date intervals.
struct DateIntervalState {
  /// Name of the bound variable in this iterator state.
  variable: Name,
  /// Iteration starting value.
  start: FeelDate,
  /// Iteration ending value.
  end: FeelDate,
  /// Iteration step.
  step: u64,
  /// Iteration step direction.
  direction: IterationDirection,
  /// Current iteration value.
  current: FeelDate,
}

impl DateIntervalState {
  fn init(variable: Name, start: FeelDate, end: FeelDate) -> Box<dyn IterationState> {
    let direction = if start <= end { IterationDirection::Ascending } else { IterationDirection::Descending };
    let current = start.clone();
    Box::new(Self {
      variable,
      start,
      end,
      step: 1,
      direction,
      current,
    })
  }
}

impl IterationState for DateIntervalState {
  fn set_value(&mut self, ctx: &mut FeelContext) {
    ctx.set_entry(&self.variable, Value::Date(self.current.clone()));
  }

  fn next(&mut self) -> bool {
    match self.direction {
      IterationDirection::Ascending => {
        let Some(date) = self.current.add_days(self.step) else { return false };
        if date <= self.end {
          self.current = date;
          true
        } else {
          self.current = self.start.clone();
          false
        }
      }
      IterationDirection::Descending => {
        let Some(date) = self.current.sub_days(self.step) else { return false };

        if date >= self.end {
          self.current = date;
          true
        } else {
          self.current = self.start.clone();
          false
        }
      }
    }
  }

  fn has_next(&self) -> bool {
    match self.direction {
      IterationDirection::Ascending => self.current.add_days(self.step).is_some_and(|date| date <= self.end),
      IterationDirection::Descending => self.current.sub_days(self.step).is_some_and(|date| date >= self.end),
    }
  }
}

struct ListState {
  /// Name of the bound variable in this iterator state.
  variable: Name,
  /// Values to iterate over.
  values: Values,
  /// Iteration step.
  step: usize,
  /// Current iteration index.
  current: usize,
}

impl ListState {
  fn init(variable: Name, value: Value) -> Box<dyn IterationState> {
    Box::new(Self {
      variable,
      values: match value {
        Value::List(values) => values,
        single => vec![single],
      },
      step: 1,
      current: 0,
    })
  }
}

impl IterationState for ListState {
  fn set_value(&mut self, ctx: &mut FeelContext) {
    if !self.values.is_empty() {
      ctx.set_entry(&self.variable, self.values[self.current].clone());
    }
  }

  fn next(&mut self) -> bool {
    if self.current + self.step < self.values.len() {
      self.current += self.step;
      true
    } else {
      self.current = 0;
      false
    }
  }

  fn has_next(&self) -> bool {
    self.current < self.values.len().saturating_sub(1)
  }

  fn is_empty(&self) -> bool {
    self.values.is_empty()
  }
}

struct VariableState {
  /// Name of the bound variable in another iteration state.
  bound_variable: Name,
  /// List iteration state.
  list_state: ListState,
}

impl VariableState {
  fn init(variable: Name, bound_variable: Name) -> Box<dyn IterationState> {
    Box::new(Self {
      bound_variable,
      list_state: ListState {
        variable,
        values: vec![],
        step: 1,
        current: 0,
      },
    })
  }
}

impl IterationState for VariableState {
  fn bind_value(&mut self, ctx: &FeelContext) {
    if self.list_state.current == 0
      && let Some(value) = ctx.get(&self.bound_variable)
    {
      self.list_state.values = match value {
        Value::List(values) => values.clone(),
        single => vec![single.clone()],
      };
      self.list_state.step = 1;
      self.list_state.current = 0;
    }
  }

  fn set_value(&mut self, ctx: &mut FeelContext) {
    self.list_state.set_value(ctx);
  }

  fn next(&mut self) -> bool {
    self.list_state.next()
  }

  fn has_next(&self) -> bool {
    self.list_state.has_next()
  }

  fn is_variable(&self) -> bool {
    true
  }
}

/// Iterator built from multiple iteration states.
#[derive(Default)]
pub struct FeelIterator {
  states: Vec<Box<dyn IterationState>>,
}

impl FeelIterator {
  /// Creates a new, empty iterator.
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_interval(&mut self, variable: Name, start: Value, end: Value, strict: bool) -> Result<()> {
    match start {
      Value::Number(start) => match end {
        Value::Number(end) => {
          if strict && start > end {
            Err(err_invalid_range())
          } else {
            self.states.push(NumberIntervalState::init(variable, start, end));
            Ok(())
          }
        }
        other => Err(err_invalid_interval_end(FEEL_TYPE_NAME_NUMBER, &other.type_of().to_string())),
      },
      Value::Date(start) => match end {
        Value::Date(end) => {
          if strict && start > end {
            Err(err_invalid_range())
          } else {
            self.states.push(DateIntervalState::init(variable, start, end));
            Ok(())
          }
        }
        other => Err(err_invalid_interval_end(FEEL_TYPE_NAME_DATE, &other.type_of().to_string())),
      },
      other => Err(err_invalid_interval_start(&other.type_of().to_string())),
    }
  }

  pub fn add_range(&mut self, variable: Name, start: Value, end: Value) -> Result<()> {
    match start {
      Value::IntervalStart(start, true) => match end {
        Value::IntervalEnd(end, true) => self.add_interval(variable, *start, *end, true),
        other => Err(err_invalid_range_end(&other.to_string())),
      },
      other => Err(err_invalid_range_start(&other.to_string())),
    }
  }

  pub fn add_list(&mut self, variable: Name, value: Value) {
    self.states.push(ListState::init(variable, value));
  }

  pub fn add_variable(&mut self, variable: Name, bound_variable: Name) {
    self.states.push(VariableState::init(variable, bound_variable));
  }

  /// Iterates over all iteration states.
  pub fn iterate<F>(&mut self, mut handler: F)
  where
    F: FnMut(&FeelContext),
  {
    if self.states.is_empty() {
      return;
    }
    let mut ctx = FeelContext::new();
    let last_state_index = self.states.len() - 1;
    loop {
      for state in self.states.iter_mut().rev().filter(|state| !state.is_variable()) {
        state.set_value(&mut ctx);
      }
      for state in self.states.iter_mut().filter(|state| state.is_variable()) {
        state.bind_value(&ctx);
        state.set_value(&mut ctx);
      }
      if !self.states.iter().any(|state| state.is_empty()) {
        handler(&ctx);
      }
      for (state_index, state) in self.states.iter_mut().rev().enumerate() {
        if state_index == last_state_index && !state.has_next() {
          return;
        }
        if state.next() {
          break;
        }
      }
    }
  }
}

/// `for` expression evaluator.
pub struct ForExpressionEvaluator {
  iterator: FeelIterator,
  partial: Name,
}

impl Default for ForExpressionEvaluator {
  /// Implements [Default] trait for [ForExpressionEvaluator].
  fn default() -> Self {
    Self::new()
  }
}

impl ForExpressionEvaluator {
  /// Creates a new `for` expression evaluator.
  pub fn new() -> Self {
    Self {
      iterator: FeelIterator::default(),
      partial: "partial".into(),
    }
  }

  /// Adds an interval of values to iterate over.
  pub fn add_interval(&mut self, name: Name, start: Value, end: Value) -> Result<()> {
    self.iterator.add_interval(name, start, end, false)
  }

  /// Adds a range of values to iterate over.
  pub fn add_range(&mut self, name: Name, start: Value, end: Value) -> Result<()> {
    self.iterator.add_range(name, start, end)
  }

  /// Adds a list of elements to iterate over.
  pub fn add_list(&mut self, name: Name, value: Value) {
    self.iterator.add_list(name, value);
  }

  pub fn add_variable(&mut self, name: Name, variable: Name) {
    self.iterator.add_variable(name, variable);
  }

  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Values {
    let mut results = vec![];
    self.iterator.iterate(|ctx| {
      let mut iteration_context = ctx.clone();
      iteration_context.set_entry(&self.partial, Value::List(results.clone()));
      scope.push(iteration_context.clone());
      let iteration_value = evaluator(scope);
      scope.pop();
      results.push(iteration_value);
    });
    results
  }
}

/// `some` expression evaluator.
pub struct SomeExpressionEvaluator {
  iterator: FeelIterator,
}

impl Default for SomeExpressionEvaluator {
  /// Implements [Default] trait for [SomeExpressionEvaluator].
  fn default() -> Self {
    Self::new()
  }
}

impl SomeExpressionEvaluator {
  pub fn new() -> Self {
    Self { iterator: FeelIterator::new() }
  }

  pub fn add_list(&mut self, name: Name, value: Value) {
    self.iterator.add_list(name, value);
  }

  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = VALUE_FALSE;
    let mut skip = false;
    self.iterator.iterate(|ctx| {
      if !skip {
        scope.push(ctx.clone());
        let value = evaluator(scope);
        match value {
          Value::Boolean(true) => {
            result = VALUE_TRUE;
            skip = true;
          }
          Value::Boolean(false) => {}
          _ => {
            result = value_null!();
            skip = true;
          }
        }
        scope.pop();
      }
    });
    result
  }
}

/// `every` expression evaluator.
pub struct EveryExpressionEvaluator {
  iterator: FeelIterator,
}

impl Default for EveryExpressionEvaluator {
  /// Implements [Default] trait for [EveryExpressionEvaluator].
  fn default() -> Self {
    Self::new()
  }
}

impl EveryExpressionEvaluator {
  pub fn new() -> Self {
    Self { iterator: FeelIterator::new() }
  }

  pub fn add_list(&mut self, name: Name, value: Value) {
    self.iterator.add_list(name, value);
  }

  pub fn evaluate(&mut self, scope: &FeelScope, evaluator: &Evaluator) -> Value {
    let mut result = VALUE_TRUE;
    let mut skip = false;
    self.iterator.iterate(|ctx| {
      if !skip {
        scope.push(ctx.clone());
        let value = evaluator(scope);
        scope.pop();
        match value {
          Value::Boolean(false) => {
            result = VALUE_FALSE;
            skip = true;
          }
          Value::Boolean(true) => {}
          _ => {
            result = value_null!();
            skip = true
          }
        }
      }
    });
    result
  }
}
