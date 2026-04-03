//! `FEEL` and `DMN` evaluator.

use crate::FeelScope;
use crate::values::Value;

/// Type alias of the function that evaluates `FEEL` expression or `DMN` model into [Value].
pub type Evaluator = Box<dyn Fn(&FeelScope) -> Value + Send + Sync>;

#[cfg(test)]
mod tests {
  use crate::values::Value;
  use crate::{Evaluator, FeelNumber, FeelScope, value_number};

  #[test]
  fn _0001() {
    let scope = FeelScope::default();
    let evaluator: Evaluator = Box::new(|_: &FeelScope| value_number!(123, 1));
    assert_eq!("12.3", evaluator(&scope).to_string());
  }
}
