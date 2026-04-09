//! # Types of range intervals.

/// Enumeration of range interval types.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IntervalType {
  /// Open-end interval with defined value, like `(1..`, `..10)`, `]1..` or `..10[`.
  Opened,
  /// Open-end interval without value, like `(..`, `..)`, `]..` or `..[`.
  OpenedUndef,
  /// Closed-end interval with defined value, like `[1..` or `..10]`.
  Closed,
  /// Closed-end interval without value, like `[..` or `..]`.
  ClosedUndef,
}

impl std::fmt::Display for IntervalType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        IntervalType::Opened => "opened",
        IntervalType::OpenedUndef => "opened,undefined",
        IntervalType::Closed => "closed",
        IntervalType::ClosedUndef => "closed,undefined",
      }
    )
  }
}

impl IntervalType {
  /// Return `true` when the interval is opened.
  pub fn opened(&self) -> bool {
    matches!(self, IntervalType::Opened | IntervalType::OpenedUndef)
  }

  /// Return `true` when the interval is closed.
  pub fn closed(&self) -> bool {
    matches!(self, IntervalType::Closed | IntervalType::ClosedUndef)
  }

  /// Return `true` when the interval has no value defined.
  pub fn undefined(&self) -> bool {
    matches!(self, IntervalType::OpenedUndef | IntervalType::ClosedUndef)
  }
}
