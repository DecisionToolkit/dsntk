//! # FEEL definitions

#[macro_use]
extern crate dsntk_macros;

pub mod bif;
pub mod bip;
pub mod closure;
pub mod context;
pub mod dto;
mod errors;
mod evaluator;
mod function;
mod names;
mod qualified_names;
mod scope;
mod strings;
mod types;
pub mod values;

mod ranges;
#[cfg(test)]
mod tests;

pub use dsntk_feel_number::FeelNumber;
pub use evaluator::Evaluator;
pub use function::FunctionBody;
pub use names::Name;
pub use qualified_names::QualifiedName;
pub use ranges::IntervalType;
pub use scope::FeelScope;
pub use strings::ToFeelString;
pub use types::*;
