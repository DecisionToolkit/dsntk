//! # Decision table model

use crate::errors::err_invalid_hit_policy;
use dsntk_common::DsntkError;
use std::fmt;
use std::fmt::Display;

/// Represents a decision table.
#[derive(Debug, Clone)]
pub struct DecisionTable {
    /// Information item name.
    pub information_item_name: Option<String>,
    /// List of instances of input clause that compose this decision table.
    pub input_clauses: Vec<InputClause>,
    /// List of instances of output clause that compose this decision table.
    pub output_clauses: Vec<OutputClause>,
    /// List of instances of rule annotation clause that compose this decision table.
    pub annotation_clauses: Vec<AnnotationClause>,
    /// List of instances of decision rule that compose this decision table.
    pub rules: Vec<DecisionRule>,
    /// Hit policy associated with the instance of the decision table.
    pub hit_policy: HitPolicy,
    /// Optional aggregation type when the hit policy is [HitPolicy::Collect].
    pub aggregation: Option<BuiltinAggregator>,
    /// Preferred orientation representation of the instance of the decision table.
    pub preferred_orientation: DecisionTableOrientation,
    /// Optional output label for the description of the decision table output.
    pub output_label: Option<String>,
}

impl DecisionTable {
    /// Creates a new decision table.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        information_item_name: Option<String>,
        input_clauses: Vec<InputClause>,
        output_clauses: Vec<OutputClause>,
        annotation_clauses: Vec<AnnotationClause>,
        rules: Vec<DecisionRule>,
        hit_policy: HitPolicy,
        aggregation: Option<BuiltinAggregator>,
        preferred_orientation: DecisionTableOrientation,
        output_label: Option<String>,
    ) -> Self {
        Self {
            information_item_name,
            input_clauses,
            output_clauses,
            annotation_clauses,
            rules,
            hit_policy,
            aggregation,
            preferred_orientation,
            output_label,
        }
    }
}

/// Represents an input clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputClause {
    /// The subject of this input clause, text representation of unary tests.
    pub input_expression: String,
    /// Optional unary tests that constrain the result of input expression of this input clause.
    pub allowed_input_values: Option<String>,
}

/// Represents an output clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputClause {
    /// The name of the output component when the decision table contains more than one output clause.
    pub output_component_name: Option<String>,
    /// Unary tests that constrain the result of output entries corresponding to output clause.
    pub allowed_output_values: Option<String>,
    /// Default output expression, selected in incomplete table when no rules match for the decision table.
    pub default_output_value: Option<String>,
}

/// Represents a rule annotation clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationClause {
    /// Text that is used as the name of the rule annotation column
    /// of the containing decision table.
    pub name: String,
}

/// Represents a decision rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRule {
    /// Ordered list of input entries that compose decision rule.
    pub input_entries: Vec<InputEntry>,
    /// Ordered list of output entries that compose decision rule.
    pub output_entries: Vec<OutputEntry>,
    /// Ordered list of rule annotations that compose decision rule.
    pub annotation_entries: Vec<AnnotationEntry>,
}

/// Represents an input entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputEntry {
    /// Text representation of unary test that composes recognized input entry.
    pub text: String,
}

/// Represents an output entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEntry {
    /// Text representation of literal expression that composes recognized output entry.
    pub text: String,
}

/// Represents a rule annotation entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationEntry {
    /// Text representation of the rule annotation.
    pub text: String,
}

/// Enumeration of hit policies.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HitPolicy {
    /// `UNIQUE` hit policy.
    Unique,
    /// `ANY` hit policy.
    Any,
    /// `PRIORITY` hit policy.
    Priority,
    /// `FIRST` hit policy.
    First,
    /// `COLLECT` hit policy.
    Collect(BuiltinAggregator),
    /// `OUTPUT ORDER` hit policy.
    OutputOrder,
    /// `RULE ORDER` hit policy.
    RuleOrder,
}

impl HitPolicy {
    /// Returns optional aggregation associated with this hit policy.
    pub fn aggregation(&self) -> Option<BuiltinAggregator> {
        match self {
            HitPolicy::Collect(aggregation) => Some(*aggregation),
            _ => None,
        }
    }
}

impl Display for HitPolicy {
    /// Converts hit policy into text.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HitPolicy::Unique => "U",
                HitPolicy::Any => "A",
                HitPolicy::Priority => "P",
                HitPolicy::First => "F",
                HitPolicy::RuleOrder => "R",
                HitPolicy::OutputOrder => "O",
                HitPolicy::Collect(aggregator) => match aggregator {
                    BuiltinAggregator::List => "C",
                    BuiltinAggregator::Sum => "C+",
                    BuiltinAggregator::Count => "C#",
                    BuiltinAggregator::Min => "C<",
                    BuiltinAggregator::Max => "C>",
                },
            }
        )
    }
}

impl TryFrom<&str> for HitPolicy {
    type Error = DsntkError;
    /// Creates a hit policy from [str].
    fn try_from(value: &str) -> dsntk_common::Result<Self, Self::Error> {
        match value.trim() {
            "U" => Ok(HitPolicy::Unique),
            "A" => Ok(HitPolicy::Any),
            "P" => Ok(HitPolicy::Priority),
            "F" => Ok(HitPolicy::First),
            "R" => Ok(HitPolicy::RuleOrder),
            "O" => Ok(HitPolicy::OutputOrder),
            "C" => Ok(HitPolicy::Collect(BuiltinAggregator::List)),
            "C+" => Ok(HitPolicy::Collect(BuiltinAggregator::Sum)),
            "C#" => Ok(HitPolicy::Collect(BuiltinAggregator::Count)),
            "C<" => Ok(HitPolicy::Collect(BuiltinAggregator::Min)),
            "C>" => Ok(HitPolicy::Collect(BuiltinAggregator::Max)),
            other => Err(err_invalid_hit_policy(other)),
        }
    }
}

impl TryFrom<&String> for HitPolicy {
    type Error = DsntkError;
    /// Creates a hit policy from reference to [String].
    fn try_from(value: &String) -> dsntk_common::Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// Enumeration of built-in aggregators for [HitPolicy].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinAggregator {
    /// The result is a list of matching output entries.
    List,
    /// The result is the number of matching outputs.
    Count,
    /// The result is the sum of all matching outputs.
    Sum,
    /// The result is the smallest value of matching outputs.
    Min,
    /// The result is the largest value of matching outputs.
    Max,
}

/// Enumeration of preferred decision table orientations.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DecisionTableOrientation {
    /// Decision table is presented horizontally, rules are presented as rows.
    RulesAsRows,
    /// Decision table is presented vertically, rules are presented as columns.
    RulesAsColumns,
    /// Decision table is presented as crosstab, rules are composed of two dimensions.
    CrossTable,
}

impl Display for DecisionTableOrientation {
    /// Converts decision table orientation into text.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DecisionTableOrientation::RulesAsRows => "rules-as-rows",
                DecisionTableOrientation::RulesAsColumns => "rules-as-columns",
                DecisionTableOrientation::CrossTable => "cross-table",
            }
        )
    }
}
