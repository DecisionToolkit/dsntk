#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate dsntk_macros;

mod ast;
mod closure;
mod context;
mod errors;
mod lalr;
mod lexer;
mod parser;
mod scope;

#[cfg(test)]
mod tests;

pub use ast::AstNode;
pub use closure::ClosureBuilder;
pub use scope::ParsingScope;

use crate::errors::*;
use crate::lalr::TokenType;
use crate::parser::Parser;
use dsntk_common::Result;
use dsntk_feel::{FeelScope, Name};

/// Parses an `expression` as defined in grammar rule `1`.
pub fn parse_expression(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartExpression, input, trace).parse()
}

/// Parses a `textual expression` as defined in grammar rule `2`.
pub fn parse_textual_expression(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartTextualExpression, input, trace).parse()
}

/// Parses `textual expressions` as defined in grammar rule `3`.
pub fn parse_textual_expressions(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartTextualExpressions, input, trace).parse()
}

/// Parses a `simple expression` as defined in grammar rule `5`.
pub fn parse_simple_expression(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartSimpleExpression, input, trace).parse()
}

/// Parses `simple expressions` as defined in grammar rule `6`.
pub fn parse_simple_expressions(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartSimpleExpressions, input, trace).parse()
}

/// Parses `unary tests` as defined in grammar rule `15`.
pub fn parse_unary_tests(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartUnaryTests, input, trace).parse()
}

/// Parses a `simple value` as defined in grammar rule `17`.
pub fn parse_simple_value(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartSimpleValue, input, trace).parse()
}

/// Parses a `name` as defined grammar rule `25`.
pub fn parse_name(scope: &FeelScope, input: &str, trace: bool) -> Result<Name> {
  if let AstNode::Name(name) = Parser::new(&scope.into(), TokenType::StartTextualExpression, input, trace).parse()? {
    Ok(name)
  } else {
    Err(err_not_a_feel_name(input))
  }
}

/// Parses the longest valid `name` as defined in grammar rule `25`.
pub fn parse_longest_name(input: &str) -> Result<Name> {
  parse_name(&Default::default(), input, false)
}

/// Parses a `boxed expression` as defined in grammar rule `53`.
pub fn parse_boxed_expression(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartBoxedExpression, input, trace).parse()
}

/// Parses a `context` as defined in grammar rule `59`.
pub fn parse_context(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartBoxedExpression, input, trace).parse()
}

/// Parses a `range literal` as defined in grammar rule `66`.
pub fn parse_range_literal(scope: &FeelScope, input: &str, trace: bool) -> Result<AstNode> {
  Parser::new(&scope.into(), TokenType::StartRangeLiteral, input, trace).parse()
}
