//! # Trait for converting FEEL artifact to its JSON representation.

/// Trait for converting a FEEL artifact into its JSON representation.
pub trait Jsonify {
    /// Implementation should convert FEEL artifact to its JSON representation.
    fn jsonify(&self) -> String;
}
