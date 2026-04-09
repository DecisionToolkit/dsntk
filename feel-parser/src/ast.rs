//! Implementation of a node in Abstract Syntax Tree for `FEEL` grammar.

use antex::{Color, ColorMode, StyledText, TreeNode, leaf, node};
use dsntk_feel::{FeelType, IntervalType, Name};
use std::fmt;
use std::fmt::{Display, Write};

/// Default color of the tree arms.
const DEFAULT_COLOR: Color = Color::White;

/// Node of the Abstract Syntax Tree for `FEEL` grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
  /// Node representing an arithmetic operator `+` (addition).
  Add(Box<AstNode>, Box<AstNode>),

  /// Node representing a logical operator `and` (conjunction).
  And(Box<AstNode>, Box<AstNode>),

  /// Node representing `@` (at) literal.
  At(String),

  /// Node representing a comparison operator `between`.
  Between(Box<AstNode>, Box<AstNode>, Box<AstNode>),

  /// Node representing a value of type `boolean`.
  Boolean(bool),

  /// Node representing a comma separated list of AST nodes, used internally by parser.
  CommaList(Vec<AstNode>),

  /// Node representing a context.
  /// Context entries are stored in the order of appearance in definition.
  Context(Vec<AstNode>),

  /// Node representing single context entry; key-value pair.
  ContextEntry(Box<AstNode>, Box<AstNode>),

  /// Node representing the key of the context entry; the key in context entry
  /// may be a name or string literal. String literals are converted to one segment names
  /// containing exactly the value of the string.
  ContextEntryKey(Name),

  /// Node representing the type of the context. Context type is defined by names
  /// and types of all entries. This node contains a collection of types
  /// for all context entries in the order of appearance in context type definition.
  ContextType(Vec<AstNode>),

  /// Node representing single context type entry.
  ContextTypeEntry(
    /// Node representing entry name of the context key.
    Box<AstNode>,
    /// Node representing `FEEL` type of the context entry.
    Box<AstNode>,
  ),

  /// Node representing the key of the entry in context type definition.
  /// In context type definition, only `FEEL` name is allowed as an entry key.
  ContextTypeEntryKey(Name),

  /// Node representing arithmetic operator `/` (division).
  Div(Box<AstNode>, Box<AstNode>),

  /// Node representing `equal` comparison.
  Eq(Box<AstNode>, Box<AstNode>),

  /// Node representing an expression evaluated as a body of `for` expression.
  EvaluatedExpression(Box<AstNode>),

  /// Quantified expression `every`.
  Every(
    /// Node representing quantified contexts.
    Box<AstNode>,
    /// Node representing an expression after `satisfies` clause.
    Box<AstNode>,
  ),

  /// Node representing exponential function.
  Exp(Box<AstNode>, Box<AstNode>),

  /// Node representing a list of expressions.
  ExpressionList(Vec<AstNode>),

  /// Node representing `FEEL` type.
  FeelType(FeelType),

  /// Node representing filter expression.
  Filter(Box<AstNode>, Box<AstNode>),

  /// Node representing `for` expression.
  For(
    /// Node representing [iteration contexts](AstNode::IterationContexts).
    Box<AstNode>,
    /// Node representing an expression to be evaluated.
    Box<AstNode>,
  ),

  /// Node representing function's formal parameter.
  FormalParameter(
    /// Node representing the name of the parameter.
    Box<AstNode>,
    /// Node representing the `FEEL` type of the parameter.
    Box<AstNode>,
  ),

  /// Node representing a list of formal parameters.
  FormalParameters(Vec<AstNode>),

  /// Node representing function's body. This node holds mandatory function body
  /// and a flag indicating if the function is external.
  FunctionBody(Box<AstNode>, bool),

  /// Node representing function definition.
  /// This node holds function's formal parameter list and  function's body.
  FunctionDefinition(Box<AstNode>, Box<AstNode>),

  /// Node representing function invocation.
  FunctionInvocation(Box<AstNode>, Box<AstNode>),

  /// Node representing function type.
  FunctionType(
    /// Node representing function's parameter types as [AstNode::ParameterTypes].
    Box<AstNode>,
    /// Node representing function's result type.
    Box<AstNode>,
  ),

  /// Node representing `greater or equal` comparison.
  Ge(Box<AstNode>, Box<AstNode>),

  /// Node representing `greater than` comparison.
  Gt(Box<AstNode>, Box<AstNode>),

  /// Node representing `if` expression.
  If(
    /// Node representing the condition.
    Box<AstNode>,
    /// Node representing the expression to be evaluated when condition is `true`.
    Box<AstNode>,
    /// Node representing the expression to be evaluated when condition is `false`.
    Box<AstNode>,
  ),

  /// Node representing `in` operator.
  In(Box<AstNode>, Box<AstNode>),

  /// Node representing type checking function.
  InstanceOf(
    /// Node representing the tested value.
    Box<AstNode>,
    /// Node representing `FELL` type to be checked.
    Box<AstNode>,
  ),

  /// Node representing the interval end used in ranges.
  IntervalEnd(
    /// Value at the interval end.
    Box<AstNode>,
    /// Type of the interval end: opened or closed.
    IntervalType,
  ),

  /// Node representing the interval start used in ranges.
  IntervalStart(
    /// Value at the interval start.
    Box<AstNode>,
    /// Type of the interval end: opened or closed.
    IntervalType,
  ),

  /// Node representing the comparison operator `irrelevant`.
  Irrelevant,

  /// List of iteration contexts.
  IterationContexts(Vec<AstNode>),

  /// Node representing iteration context containing a single value to iterate over.
  IterationContextSingle(
    /// Node representing variable name used in this iteration context.
    Box<AstNode>,
    /// Node representing a single value to iterate over.
    Box<AstNode>,
  ),

  /// Node representing iteration context containing the variable name
  /// and a staring and ending value of the interval to iterate over.
  IterationContextInterval(
    /// Node representing variable name used in this iteration context.
    Box<AstNode>,
    /// Node representing the `start` value of the interval to iterate over.
    Box<AstNode>,
    /// Node representing the `end` value of the interval to iterate over.
    Box<AstNode>,
  ),

  /// Node representing `less or equal` comparison.
  Le(Box<AstNode>, Box<AstNode>),

  /// Node representing `less than` comparison.
  Lt(Box<AstNode>, Box<AstNode>),

  /// Node representing a list.
  List(Vec<AstNode>),

  /// Node representing a list type.
  ListType(Box<AstNode>),

  /// Node representing arithmetic operator `*` (multiplication).
  Mul(Box<AstNode>, Box<AstNode>),

  /// Node representing a `FEEL` name.
  Name(Name),

  /// Node representing single named parameter.
  NamedParameter(
    /// Node representing parameter name.
    Box<AstNode>,
    /// Node representing parameter type.
    Box<AstNode>,
  ),

  /// Node representing a collection of named parameters.
  NamedParameters(Vec<AstNode>),

  /// Node representing a negated list (used in negated unary tests).
  NegatedList(Vec<AstNode>),

  /// Node representing the unary arithmetic negation `-`.
  Neg(Box<AstNode>),

  /// Node representing `not equal` comparison.
  Nq(Box<AstNode>, Box<AstNode>),

  /// Node representing a value of type `Null`.
  Null,

  /// Node representing a value of type `number`.
  Numeric(
    /// Digits before decimal separator.
    String,
    /// Digits after decimal separator.
    String,
    /// Sign of the exponent.
    char,
    /// Digits of the exponent.
    String,
  ),

  /// Node representing a logical operator `or` (disjunction).
  Or(Box<AstNode>, Box<AstNode>),

  /// Node representing expression for selecting decision table's output value.
  Out(Box<AstNode>, Box<AstNode>),

  /// Node representing a name of the function's formal parameter.
  ParameterName(Name),

  /// Node representing a collection of function parameter types.
  ParameterTypes(Vec<AstNode>),

  /// Node representing a path expression.
  Path(Box<AstNode>, Box<AstNode>),

  /// Node representing a collection of positional parameters.
  PositionalParameters(Vec<AstNode>),

  /// Node representing a collection of names that constitute a qualified name.
  QualifiedName(Vec<AstNode>),

  /// Node representing a segment of a qualified name.
  QualifiedNameSegment(Name),

  /// List of quantified contexts.
  QuantifiedContexts(Vec<AstNode>),

  /// Quantified context containing variable name and evaluation expression.
  QuantifiedContext(
    /// Node representing variable name used in this quantified context.
    Box<AstNode>,
    /// Node representing evaluation expression.
    Box<AstNode>,
  ),

  /// Node representing a range of values.
  Range(Box<AstNode>, Box<AstNode>),

  /// Node representing range type.
  RangeType(Box<AstNode>),

  /// Node representing `satisfies` clause in quantified expression.
  Satisfies(Box<AstNode>),

  /// Node representing quantified expression `some`.
  Some(
    /// Node representing quantified contexts.
    Box<AstNode>,
    /// Node representing an expression after `satisfies` clause.
    Box<AstNode>,
  ),

  /// Node representing a value of type `string`.
  String(String),

  /// Node representing an arithmetic operator `-` (subtraction).
  Sub(Box<AstNode>, Box<AstNode>),

  /// Node representing unary comparison operator `>=`.
  UnaryGe(Box<AstNode>),

  /// Node representing unary comparison operator `>`.
  UnaryGt(Box<AstNode>),

  /// Node representing unary comparison operator `<=>`.
  UnaryLe(Box<AstNode>),

  /// Node representing unary comparison operator `<less than>`.
  UnaryLt(Box<AstNode>),

  /// Node representing unary comparison operator `=`.
  UnaryEq(Box<AstNode>),

  /// Node representing unary comparison operator `!=`.
  UnaryNe(Box<AstNode>),
}

impl Display for AstNode {
  /// Converts [AstNode] to its textual representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.tree(0, ColorMode::Off))
  }
}

impl AstNode {
  /// Converts [AstNode] to its textual representation with indentation and color mode.
  pub fn tree(&self, indent: usize, cm: ColorMode) -> String {
    let root = ast_node_to_tree(self, cm);
    let mut output = String::new();
    _ = write!(output, "{:indent$}", root);
    output
  }

  /// Returns AST as text for testing purposes.
  pub fn trace(&self) -> String {
    let root = ast_node_to_tree(self, ColorMode::Off);
    let mut output = String::new();
    _ = write!(output, "\n{:6}    ", root);
    output
  }
}

/// Converts [AstNode] into ASCII [TreeNode] node.
fn ast_node_to_tree(node: &AstNode, cm: ColorMode) -> TreeNode {
  match node {
    AstNode::Add(lhs, rhs) => ast_node_2("Add", lhs, rhs, cm),
    AstNode::And(lhs, rhs) => ast_node_2("And", lhs, rhs, cm),
    AstNode::At(mid) => ast_node_and_leaf("At", &format!("`{mid}`"), cm),
    AstNode::Between(lhs, mid, rhs) => ast_node_3("Between", lhs, mid, rhs, cm),
    AstNode::Boolean(mid) => ast_node_and_leaf("Boolean", &format!("`{mid}`"), cm),
    AstNode::CommaList(mid) => ast_node_n("CommaList", mid, cm),
    AstNode::Context(items) => ast_node_n("Context", items, cm),
    AstNode::ContextEntry(lhs, rhs) => ast_node_2("ContextEntry", lhs, rhs, cm),
    AstNode::ContextEntryKey(mid) => ast_node_and_leaf("ContextEntryKey", &format!("`{mid}`"), cm),
    AstNode::ContextType(items) => ast_node_n("ContextType", items, cm),
    AstNode::ContextTypeEntry(lhs, rhs) => ast_node_2("ContextTypeEntry", lhs, rhs, cm),
    AstNode::ContextTypeEntryKey(mid) => ast_node_and_leaf("Name", &format!("`{mid}`"), cm),
    AstNode::Div(lhs, rhs) => ast_node_2("Div", lhs, rhs, cm),
    AstNode::Eq(lhs, rhs) => ast_node_2("Eq", lhs, rhs, cm),
    AstNode::EvaluatedExpression(mid) => ast_node_1("EvaluatedExpression", mid, cm),
    AstNode::Every(lhs, rhs) => ast_node_2("Every", lhs, rhs, cm),
    AstNode::Exp(lhs, rhs) => ast_node_2("Exp", lhs, rhs, cm),
    AstNode::ExpressionList(items) => ast_node_n("ExpressionList", items, cm),
    AstNode::FeelType(lhs) => ast_node_and_leaf("FeelType", &lhs.to_string(), cm),
    AstNode::Filter(lhs, rhs) => ast_node_2("Filter", lhs, rhs, cm),
    AstNode::For(lhs, rhs) => ast_node_2("For", lhs, rhs, cm),
    AstNode::FormalParameter(lhs, rhs) => ast_node_2("FormalParameter", lhs, rhs, cm),
    AstNode::FormalParameters(items) => ast_node_n("FormalParameters", items, cm),
    AstNode::FunctionBody(lhs, external) => ast_node_and_conditional_label("FunctionBody", lhs, " (external)", "", *external, cm),
    AstNode::FunctionDefinition(lhs, rhs) => ast_node_2("FunctionDefinition", lhs, rhs, cm),
    AstNode::FunctionInvocation(lhs, rhs) => ast_node_2("FunctionInvocation", lhs, rhs, cm),
    AstNode::FunctionType(lhs, rhs) => ast_node_2("FunctionType", lhs, rhs, cm),
    AstNode::Ge(lhs, rhs) => ast_node_2("Ge", lhs, rhs, cm),
    AstNode::Gt(lhs, rhs) => ast_node_2("Gt", lhs, rhs, cm),
    AstNode::If(lhs, mid, rhs) => ast_node_3("If", lhs, mid, rhs, cm),
    AstNode::In(lhs, rhs) => ast_node_2("In", lhs, rhs, cm),
    AstNode::InstanceOf(lhs, rhs) => ast_node_2("InstanceOf", lhs, rhs, cm),
    AstNode::IntervalEnd(lhs, interval_type) => ast_node_and_label("IntervalEnd", lhs, interval_type.to_string(), cm),
    AstNode::IntervalStart(lhs, interval_type) => ast_node_and_label("IntervalStart", lhs, interval_type.to_string(), cm),
    AstNode::Irrelevant => ast_leaf("Irrelevant", cm),
    AstNode::IterationContexts(items) => ast_node_n("IterationContexts", items, cm),
    AstNode::IterationContextSingle(lhs, rhs) => ast_node_2("IterationContextSingle", lhs, rhs, cm),
    AstNode::IterationContextInterval(lhs, mid, rhs) => ast_node_3("IterationContextInterval", lhs, mid, rhs, cm),
    AstNode::Le(lhs, rhs) => ast_node_2("Le", lhs, rhs, cm),
    AstNode::List(mid) => ast_node_n("List", mid, cm),
    AstNode::ListType(lhs) => ast_node_1("ListType", lhs, cm),
    AstNode::Lt(lhs, rhs) => ast_node_2("Lt", lhs, rhs, cm),
    AstNode::Mul(lhs, rhs) => ast_node_2("Mul", lhs, rhs, cm),
    AstNode::Name(mid) => ast_node_and_leaf("Name", &format!("`{mid}`"), cm),
    AstNode::NamedParameter(lhs, rhs) => ast_node_2("NamedParameter", lhs, rhs, cm),
    AstNode::NamedParameters(items) => ast_node_n("NamedParameters", items, cm),
    AstNode::Neg(mid) => ast_node_1("Neg", mid, cm),
    AstNode::NegatedList(mid) => ast_node_n("NegatedList", mid, cm),
    AstNode::Nq(lhs, rhs) => ast_node_2("Nq", lhs, rhs, cm),
    AstNode::Null => ast_leaf("Null", cm),
    AstNode::Numeric(before, after, sign, exponent) => ast_node_and_leaf("Numeric", &numeric_to_string(before, after, sign, exponent), cm),
    AstNode::Or(lhs, rhs) => ast_node_2("Or", lhs, rhs, cm),
    AstNode::Out(lhs, rhs) => ast_node_2("Out", lhs, rhs, cm),
    AstNode::ParameterName(lhs) => ast_node_and_leaf("ParameterName", &format!("`{lhs}`"), cm),
    AstNode::ParameterTypes(items) => ast_node_n("ParameterTypes", items, cm),
    AstNode::Path(lhs, rhs) => ast_node_2("Path", lhs, rhs, cm),
    AstNode::PositionalParameters(items) => ast_node_n("PositionalParameters", items, cm),
    AstNode::QualifiedName(items) => ast_node_n("QualifiedName", items, cm),
    AstNode::QualifiedNameSegment(lhs) => ast_node_and_leaf("Name", &format!("`{lhs}`"), cm),
    AstNode::QuantifiedContext(lhs, rhs) => ast_node_2("QuantifiedContext", lhs, rhs, cm),
    AstNode::QuantifiedContexts(items) => ast_node_n("QuantifiedContexts", items, cm),
    AstNode::Range(lhs, rhs) => ast_node_2("Range", lhs, rhs, cm),
    AstNode::RangeType(lhs) => ast_node_1("RangeType", lhs, cm),
    AstNode::Satisfies(mid) => ast_node_1("Satisfies", mid, cm),
    AstNode::Some(lhs, rhs) => ast_node_2("Some", lhs, rhs, cm),
    AstNode::String(mid) => ast_node_and_leaf("String", &format!("`{mid}`"), cm),
    AstNode::Sub(lhs, rhs) => ast_node_2("Sub", lhs, rhs, cm),
    AstNode::UnaryGe(mid) => ast_node_1("UnaryGe", mid, cm),
    AstNode::UnaryGt(mid) => ast_node_1("UnaryGt", mid, cm),
    AstNode::UnaryLe(mid) => ast_node_1("UnaryLe", mid, cm),
    AstNode::UnaryLt(mid) => ast_node_1("UnaryLt", mid, cm),
    AstNode::UnaryEq(mid) => ast_node_1("UnaryEq", mid, cm),
    AstNode::UnaryNe(mid) => ast_node_1("UnaryNe", mid, cm),
  }
}

/// Creates a tree node with one child node.
fn ast_node_1(name: &str, mid: &AstNode, cm: ColorMode) -> TreeNode {
  node(DEFAULT_COLOR, cm).line().s(name).end().child(ast_node_to_tree(mid, cm)).end()
}

/// Creates a tree node with two child nodes.
fn ast_node_2(name: &str, lhs: &AstNode, rhs: &AstNode, cm: ColorMode) -> TreeNode {
  node(DEFAULT_COLOR, cm)
    .line()
    .s(name)
    .end()
    .child(ast_node_to_tree(lhs, cm))
    .child(ast_node_to_tree(rhs, cm))
    .end()
}

/// Creates a tree node with three child nodes.
fn ast_node_3(name: &str, lhs: &AstNode, mid: &AstNode, rhs: &AstNode, cm: ColorMode) -> TreeNode {
  node(DEFAULT_COLOR, cm)
    .line()
    .s(name)
    .end()
    .child(ast_node_to_tree(lhs, cm))
    .child(ast_node_to_tree(mid, cm))
    .child(ast_node_to_tree(rhs, cm))
    .end()
}

/// Creates a tree node with multiple child nodes.
fn ast_node_n(name: &str, items: &[AstNode], cm: ColorMode) -> TreeNode {
  let mut node_builder = node(DEFAULT_COLOR, cm).line().s(name).end();
  if items.is_empty() {
    node_builder.add_child(leaf(cm).line().s("(empty)").end().end());
  } else {
    for item in items {
      node_builder.add_child(ast_node_to_tree(item, cm));
    }
  }
  node_builder.end()
}

/// Creates a node with single leaf node.
fn ast_node_and_leaf(name: &str, leaf_caption: &str, cm: ColorMode) -> TreeNode {
  node(DEFAULT_COLOR, cm).line().s(name).end().child(leaf(cm).line().s(leaf_caption).end().end()).end()
}

/// Creates a single node with conditional label.
fn ast_node_and_conditional_label(name: &str, lhs: &AstNode, label_true: &str, label_false: &str, label_flag: bool, cm: ColorMode) -> TreeNode {
  let name_label = if label_flag { label_true } else { label_false };
  node(DEFAULT_COLOR, cm).line().s(name).s(name_label).end().child(ast_node_to_tree(lhs, cm)).end()
}

/// Creates a single node with label.
fn ast_node_and_label(name: &str, lhs: &AstNode, label: impl AsRef<str>, cm: ColorMode) -> TreeNode {
  let label = format!(" ({})", label.as_ref());
  node(DEFAULT_COLOR, cm).line().s(name).s(label).end().child(ast_node_to_tree(lhs, cm)).end()
}

/// Creates a leaf node.
fn ast_leaf(leaf_caption: &str, cm: ColorMode) -> TreeNode {
  leaf(cm).line().s(leaf_caption).end().end()
}

/// Converts a numeric AST node into string representation used in tree visualisation.
fn numeric_to_string(before: &str, after: &str, sign: &char, exponent: &str) -> String {
  let mut output = String::new();
  let _ = write!(&mut output, "`{before}");
  if !after.is_empty() {
    let _ = write!(&mut output, ".{after}");
  }
  if !exponent.is_empty() {
    let _ = write!(&mut output, "e{sign}{exponent}");
  }
  let _ = write!(&mut output, "`");
  output
}
