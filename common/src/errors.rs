//! # Definition of the common result and error types

/// Common result type.
pub type Result<T, E = DsntkError> = std::result::Result<T, E>;

/// Common trait to be implemented by structs defining a specific error.
pub trait ToErrorMessage {
  /// Convert error definition to a message string.
  fn message(self) -> String;
}

/// Common error definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DsntkError(String);

impl std::fmt::Display for DsntkError {
  /// Implementation of [Display](std::fmt::Display) trait for [DsntkError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl DsntkError {
  /// Creates a new [DsntkError] with specified source name and error message.
  pub fn new(source: &str, message: &str) -> Self {
    Self(format!("<{source}> {message}"))
  }
}

impl<T> From<T> for DsntkError
where
  T: ToErrorMessage,
{
  /// Converts any type that implements [ToErrorMessage] trait to [DsntkError].
  fn from(value: T) -> Self {
    let error_type_name = std::any::type_name::<T>().split("::").last().unwrap_or("UnknownError");
    DsntkError::new(error_type_name, &value.message())
  }
}
