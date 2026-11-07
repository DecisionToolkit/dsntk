//! Trait for converting FEEL artifact into FEEL string.

/// Trait for converting FEEL artifact into FEEL string.
pub trait ToFeelString {
    /// Converts FEEL artifact into FEEL string.
    fn to_feel_string(&self) -> String;
}
