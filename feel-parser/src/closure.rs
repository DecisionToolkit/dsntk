//! Implementation of the context for closures (lambdas).

use crate::AstNode;
use dsntk_feel::closure::Closure;
use dsntk_feel::{Name, QualifiedName};
use std::collections::BTreeSet;

/// Context for closures (lambdas).
#[derive(Default)]
pub struct ClosureBuilder {
    /// Collection of parameter names in function definitions.
    parameter_names: BTreeSet<QualifiedName>,
    /// Collection of names used in expressions.
    names: BTreeSet<QualifiedName>,
}

impl ClosureBuilder {
    pub fn from_node(node: &AstNode) -> Closure {
        let mut closure_context = ClosureBuilder::default();
        closure_context.visit_1(node, 0);
        closure_context
            .names
            .difference(&closure_context.parameter_names)
            .cloned()
            .collect::<Vec<QualifiedName>>()
            .into()
    }

    pub fn from_function_definition(lhs: &AstNode, rhs: &AstNode) -> Closure {
        let mut closure_context = ClosureBuilder::default();
        closure_context.visit_2(lhs, rhs, 0);
        closure_context
            .names
            .difference(&closure_context.parameter_names)
            .cloned()
            .collect::<Vec<QualifiedName>>()
            .into()
    }

    /// Returns an empty vector of names.
    fn visit_0(&mut self, _: usize) -> Vec<Name> {
        vec![]
    }

    /// Visits a single AST node.
    fn visit_1(&mut self, node: &AstNode, path_level: usize) -> Vec<Name> {
        match node {
            AstNode::Add(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::And(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::At(_) => self.visit_0(path_level),
            AstNode::Between(lhs, mhs, rhs) => self.visit_3(lhs, mhs, rhs, path_level),
            AstNode::Boolean(_) => self.visit_0(path_level),
            AstNode::CommaList(lhs) => self.visit_list(lhs, path_level),
            AstNode::Context(lhs) => self.visit_list(lhs, path_level),
            AstNode::ContextEntry(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::ContextEntryKey(_) => self.visit_0(path_level),
            AstNode::ContextType(lhs) => self.visit_list(lhs, path_level),
            AstNode::ContextTypeEntry(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::ContextTypeEntryKey(_) => self.visit_0(path_level),
            AstNode::Div(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Eq(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::EvaluatedExpression(lhs) => self.visit_1(lhs, path_level),
            AstNode::Every(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Exp(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::ExpressionList(lhs) => self.visit_list(lhs, path_level),
            AstNode::FeelType(_) => self.visit_0(path_level),
            AstNode::Filter(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::For(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::FormalParameter(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::FormalParameters(lhs) => self.visit_list(lhs, path_level),
            AstNode::FunctionBody(lhs, _) => self.visit_1(lhs, path_level),
            AstNode::FunctionDefinition(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::FunctionInvocation(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::FunctionType(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Ge(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Gt(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::If(lhs, mhs, rhs) => self.visit_3(lhs, mhs, rhs, path_level),
            AstNode::In(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::InstanceOf(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::IntervalEnd(lhs, _) => self.visit_1(lhs, path_level),
            AstNode::IntervalStart(lhs, _) => self.visit_1(lhs, path_level),
            AstNode::Irrelevant => self.visit_0(path_level),
            AstNode::IterationContexts(lhs) => self.visit_list(lhs, path_level),
            AstNode::IterationContextSingle(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::IterationContextInterval(lhs, mhs, rhs) => {
                self.visit_3(lhs, mhs, rhs, path_level)
            }
            AstNode::Le(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::List(lhs) => self.visit_list(lhs, path_level),
            AstNode::ListType(lhs) => self.visit_1(lhs, path_level),
            AstNode::Lt(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Mul(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Name(name) => {
                if path_level == 0 {
                    let closure_name = name.clone();
                    self.names.insert(closure_name.into());
                }
                vec![name.clone()]
            }
            AstNode::NamedParameter(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::NamedParameters(lhs) => self.visit_list(lhs, path_level),
            AstNode::Neg(lhs) => self.visit_1(lhs, path_level),
            AstNode::NegatedList(lhs) => self.visit_list(lhs, path_level),
            AstNode::Null => self.visit_0(path_level),
            AstNode::Numeric(_, _, _, _) => self.visit_0(path_level),
            AstNode::Nq(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Or(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::Out(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::ParameterName(name) => {
                let parameter_name = name.clone();
                self.parameter_names.insert(parameter_name.into());
                vec![name.clone()]
            }
            AstNode::ParameterTypes(lhs) => self.visit_list(lhs, path_level),
            AstNode::Path(lhs, rhs) => {
                let mut parts = self.visit_1(rhs, path_level + 1);
                let mut name = self.visit_1(lhs, path_level + 1);
                parts.append(&mut name);
                if path_level == 0 {
                    let mut closure_parts = parts.clone();
                    closure_parts.reverse();
                    self.names.insert(closure_parts.into());
                }
                parts
            }
            AstNode::PositionalParameters(lhs) => self.visit_list(lhs, path_level),
            AstNode::QualifiedName(lhs) => self.visit_list(lhs, path_level),
            AstNode::QualifiedNameSegment(_name) => self.visit_0(path_level), //TODO verify if also must be used
            AstNode::QuantifiedContext(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::QuantifiedContexts(lhs) => self.visit_list(lhs, path_level),
            AstNode::Range(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::RangeType(lhs) => self.visit_1(lhs, path_level),
            AstNode::Some(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::String(_) => self.visit_0(path_level),
            AstNode::Sub(lhs, rhs) => self.visit_2(lhs, rhs, path_level),
            AstNode::UnaryGe(lhs) => self.visit_1(lhs, path_level),
            AstNode::UnaryGt(lhs) => self.visit_1(lhs, path_level),
            AstNode::UnaryLe(lhs) => self.visit_1(lhs, path_level),
            AstNode::UnaryLt(lhs) => self.visit_1(lhs, path_level),
            AstNode::UnaryEq(lhs) => self.visit_1(lhs, path_level),
            AstNode::UnaryNe(lhs) => self.visit_1(lhs, path_level),
            AstNode::Satisfies(lhs) => self.visit_1(lhs, path_level),
        }
    }

    /// Visits two AST nodes.
    fn visit_2(&mut self, lhs: &AstNode, rhs: &AstNode, path_level: usize) -> Vec<Name> {
        self.visit_1(lhs, path_level);
        self.visit_1(rhs, path_level);
        vec![]
    }

    /// Visits three AST nodes.
    fn visit_3(
        &mut self,
        lhs: &AstNode,
        mhs: &AstNode,
        rhs: &AstNode,
        path_level: usize,
    ) -> Vec<Name> {
        self.visit_1(lhs, path_level);
        self.visit_1(mhs, path_level);
        self.visit_1(rhs, path_level);
        vec![]
    }

    /// Visits a list of AST nodes.
    fn visit_list(&mut self, lhs: &Vec<AstNode>, path_level: usize) -> Vec<Name> {
        for item in lhs {
            self.visit_1(item, path_level);
        }
        vec![]
    }
}
