use crate::{
    AnnotationEntry, BuiltinAggregator, DecisionRule, DecisionTable, DecisionTableOrientation,
    DmnId, HitPolicy, InputClause, InputEntry, OutputClause, OutputEntry, RuleAnnotationClause,
};
use dsntk_common::gen_id;
use dsntk_recognizer::{
    AnnotationClause as RecognizedRuleAnnotationClause,
    AnnotationEntry as RecognizedAnnotationEntry, BuiltinAggregator as RecognizedBuiltinAggregator,
    DecisionRule as RecognizedDecisionRule, DecisionTable as RecognizedDecisionTable,
    DecisionTableOrientation as RecognizedDecisionTableOrientation,
    HitPolicy as RecognizedHitPolicy, InputClause as RecognizedInputClause,
    InputEntry as RecognizedInputEntry, OutputClause as RecognizedOutputClause,
    OutputEntry as RecognizedOutputEntry,
};

impl From<RecognizedDecisionTable> for DecisionTable {
    fn from(value: RecognizedDecisionTable) -> Self {
        let RecognizedDecisionTable {
            information_item_name,
            mut input_clauses,
            mut output_clauses,
            annotation_clauses: mut annotations,
            mut rules,
            hit_policy,
            aggregation,
            preferred_orientation: orientation,
            output_label,
        } = value;
        Self {
            information_item_name,
            input_clauses: input_clauses.drain(..).map(Into::into).collect(),
            output_clauses: output_clauses.drain(..).map(Into::into).collect(),
            annotations: annotations.drain(..).map(Into::into).collect(),
            rules: rules.drain(..).map(Into::into).collect(),
            hit_policy: hit_policy.into(),
            aggregation: aggregation.map(Into::into),
            preferred_orientation: orientation.into(),
            output_label,
            namespace: "".to_string(),
            model_name: "".to_string(),
            id: DmnId::Generated(gen_id()),
            description: None,
            label: None,
            extension_elements: vec![],
            extension_attributes: vec![],
            type_ref: None,
        }
    }
}

impl From<RecognizedInputClause> for InputClause {
    fn from(value: RecognizedInputClause) -> Self {
        Self {
            input_expression: value.input_expression,
            allowed_input_values: value.allowed_input_values,
        }
    }
}

impl From<RecognizedOutputClause> for OutputClause {
    fn from(value: RecognizedOutputClause) -> Self {
        Self {
            type_ref: None,
            name: value.output_component_name,
            allowed_output_values: value.allowed_output_values,
            default_output_entry: value.default_output_value,
        }
    }
}

impl From<RecognizedRuleAnnotationClause> for RuleAnnotationClause {
    fn from(value: RecognizedRuleAnnotationClause) -> Self {
        Self {
            name: value.name.to_string(),
        }
    }
}

impl From<RecognizedDecisionRule> for DecisionRule {
    fn from(value: RecognizedDecisionRule) -> Self {
        let RecognizedDecisionRule {
            mut input_entries,
            mut output_entries,
            mut annotation_entries,
        } = value;
        Self {
            input_entries: input_entries.drain(..).map(Into::into).collect(),
            output_entries: output_entries.drain(..).map(Into::into).collect(),
            annotation_entries: annotation_entries.drain(..).map(Into::into).collect(),
        }
    }
}

impl From<RecognizedInputEntry> for InputEntry {
    fn from(value: RecognizedInputEntry) -> Self {
        Self { text: value.text }
    }
}

impl From<RecognizedOutputEntry> for OutputEntry {
    fn from(value: RecognizedOutputEntry) -> Self {
        Self { text: value.text }
    }
}

impl From<RecognizedAnnotationEntry> for AnnotationEntry {
    fn from(value: RecognizedAnnotationEntry) -> Self {
        Self { text: value.text }
    }
}

impl From<RecognizedHitPolicy> for HitPolicy {
    fn from(value: RecognizedHitPolicy) -> Self {
        match value {
            RecognizedHitPolicy::Unique => Self::Unique,
            RecognizedHitPolicy::Any => Self::Any,
            RecognizedHitPolicy::Priority => Self::Priority,
            RecognizedHitPolicy::First => Self::First,
            RecognizedHitPolicy::Collect(aggregator) => Self::Collect(aggregator.into()),
            RecognizedHitPolicy::OutputOrder => Self::OutputOrder,
            RecognizedHitPolicy::RuleOrder => Self::RuleOrder,
        }
    }
}

impl From<RecognizedBuiltinAggregator> for BuiltinAggregator {
    fn from(value: RecognizedBuiltinAggregator) -> Self {
        match value {
            RecognizedBuiltinAggregator::List => Self::List,
            RecognizedBuiltinAggregator::Count => Self::Count,
            RecognizedBuiltinAggregator::Sum => Self::Sum,
            RecognizedBuiltinAggregator::Min => Self::Min,
            RecognizedBuiltinAggregator::Max => Self::Max,
        }
    }
}

impl From<RecognizedDecisionTableOrientation> for DecisionTableOrientation {
    fn from(value: RecognizedDecisionTableOrientation) -> Self {
        match value {
            RecognizedDecisionTableOrientation::RulesAsRows => Self::RuleAsRow,
            RecognizedDecisionTableOrientation::RulesAsColumns => Self::RuleAsColumn,
            RecognizedDecisionTableOrientation::CrossTable => Self::CrossTable,
        }
    }
}
