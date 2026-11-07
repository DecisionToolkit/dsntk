//! # Decision table builder

use crate::errors::*;
use crate::recognizer::Recognizer;
use crate::utils::get_allowed_and_default_output_values;
use crate::{
    AnnotationClause, AnnotationEntry, DecisionRule, DecisionTable, HitPolicy, InputClause,
    InputEntry, OutputClause, OutputEntry,
};
use dsntk_common::Result;

/// Combines sizes of individual components of the decision table.
struct Size {
    input_clauses_count: usize,
    input_values_count: usize,
    output_clauses_count: usize,
    output_components_count: usize,
    output_values_count: usize,
    annotation_clauses_count: usize,
    rule_count: usize,
}

/// Validates the size of the individual components of the decision table in relation to each other.
fn validate_size(recognizer: &Recognizer) -> Result<Size> {
    // decision table may have zero or more input clauses
    let input_clauses_count = recognizer.input_clause_count;
    // number of input expressions must be equal to the number of input clauses
    let input_expression_count = recognizer.input_expressions.len();
    if input_expression_count != input_clauses_count {
        return Err(err_invalid_size(&format!(
            "number of input expressions ({input_expression_count}) must be equal to the number of input clauses ({input_clauses_count})"
        )));
    }
    // when input values are present, then the number of input values must be equal to the number input expressions
    let input_values_count = recognizer.allowed_input_values.len();
    if input_values_count > 0 && input_values_count != input_clauses_count {
        return Err(err_invalid_size(&format!(
            "number of input values ({input_values_count}) must be equal to the number of input clauses ({input_clauses_count})"
        )));
    }
    // decision table must have minimum one output clause
    let output_clauses_count = recognizer.output_clause_count;
    if output_clauses_count == 0 {
        return Err(err_invalid_size(
            "decision table must have minimum one output clause",
        ));
    }
    // the number of the output components must be equal to the number of output clauses when more than one output clause is present
    let output_components_count = recognizer.output_components.len();
    if output_clauses_count > 1 {
        if output_components_count != output_clauses_count {
            return Err(err_invalid_size(&format!(
                "number of output components ({output_components_count}) must be equal to the number of output clauses ({output_clauses_count})"
            )));
        }
    } else if output_components_count != 0 {
        return Err(err_invalid_size("number of output components must be zero"));
    }
    // when output values are present, then the number of output values must be equal to the number of output clauses
    let output_values_count = recognizer.allowed_output_values.len();
    if output_values_count > 0 && output_values_count != output_clauses_count {
        return Err(err_invalid_size(&format!(
            "number of output values ({output_values_count}) must be equal to the number of output clauses ({output_clauses_count})"
        )));
    }
    // decision table must contain minimum one rule
    let rule_count = recognizer.rule_count;
    if rule_count == 0 {
        return Err(err_invalid_size(
            "decision table must contain minimum one rule",
        ));
    }
    // number of rows of input entries must be equal to the number of rules
    let input_entries_row_count = recognizer.input_entries.len();
    if input_entries_row_count != rule_count {
        return Err(err_invalid_size(&format!(
            "number of input entries ({input_entries_row_count}) must be equal to the number of rules ({rule_count})",
        )));
    }
    // number of input entries in each row must be equal to the number of input clauses
    for (row_index, row) in recognizer.input_entries.iter().enumerate() {
        if row.len() != input_clauses_count {
            return Err(err_invalid_size(&format!(
                "number of input entries ({}) must be equal to the number of input clauses ({input_clauses_count}) in row {row_index}",
                row.len(),
            )));
        }
    }
    // number of rows of output entries must be equal to the number of rules
    let output_entries_row_count = recognizer.output_entries.len();
    if output_entries_row_count != rule_count {
        return Err(err_invalid_size(&format!(
            "number of output entries ({output_entries_row_count}) must be equal to the number of rules ({rule_count})"
        )));
    }
    // number of output entries in each row must be equal to the number of output clauses
    for (row_index, row) in recognizer.output_entries.iter().enumerate() {
        if row.len() != output_clauses_count {
            return Err(err_invalid_size(&format!(
                "number of output entries ({}) must be equal to the number of output clauses ({output_clauses_count}) in row {row_index}",
                row.len()
            )));
        }
    }
    // decision table may contain some annotations
    let annotation_clauses_count = recognizer.annotation_clause_count;
    if annotation_clauses_count > 0 {
        // number of rows of annotation entries must be equal to the number of rules
        let annotation_entries_row_count = recognizer.annotation_entries.len();
        if annotation_entries_row_count != rule_count {
            return Err(err_invalid_size(&format!(
                "number of annotation entries ({annotation_entries_row_count}) must be equal to the number of rules ({rule_count})"
            )));
        }
        // number of annotation entries in each row must be equal to the number of annotation clauses
        for (row_index, row) in recognizer.annotation_entries.iter().enumerate() {
            if row.len() != annotation_clauses_count {
                return Err(err_invalid_size(&format!(
                    "number of annotation entries ({}) must be equal to the number of annotation clauses ({}) in row {}",
                    row.len(),
                    annotation_clauses_count,
                    row_index
                )));
            }
        }
    }
    Ok(Size {
        input_clauses_count,
        input_values_count,
        output_clauses_count,
        output_components_count,
        output_values_count,
        annotation_clauses_count,
        rule_count,
    })
}

/// Recognizes a decision table defined as Unicode text.
pub fn from_unicode(text: &str, trace: bool) -> Result<DecisionTable> {
    // Recognize the decision table.
    let recognizer = Recognizer::recognize(text, trace)?;

    // Validate the size of the individual parts of the decision table.
    let size = validate_size(&recognizer)?;

    // Prepare the information item name.
    let information_item_name = recognizer.information_item_name.clone();

    // Prepare the hit policy.
    let hit_policy = recognizer.hit_policy;

    // Prepare the aggregation.
    let aggregation = if let HitPolicy::Collect(built_in_aggregator) = hit_policy {
        Some(built_in_aggregator)
    } else {
        None
    };

    // Prepare the preferred orientation.
    let preferred_orientation = recognizer.orientation;

    // Prepare the output label
    let output_label = recognizer.output_label.clone();

    // Prepare the input clauses.
    let mut input_clauses = vec![];
    for i in 0..size.input_clauses_count {
        input_clauses.push(InputClause {
            input_expression: recognizer.input_expressions[i].clone(),
            allowed_input_values: if size.input_values_count > 0 {
                recognizer.allowed_input_values[i].clone()
            } else {
                None
            },
        });
    }

    // Prepare output clauses.
    let mut output_clauses = vec![];
    for i in 0..size.output_clauses_count {
        let output_component_name = if size.output_components_count > 0 {
            recognizer.output_components[i].clone()
        } else {
            output_label.clone()
        };
        let (allowed_output_values, default_output_value) =
            get_allowed_and_default_output_values(&if size.output_values_count > 0 {
                recognizer.allowed_output_values[i].clone()
            } else {
                None
            });
        output_clauses.push(OutputClause {
            output_component_name,
            allowed_output_values,
            default_output_value,
        });
    }

    // Prepare annotations.
    let mut annotations = vec![];
    for i in 0..recognizer.annotation_clause_count {
        annotations.push(AnnotationClause {
            name: recognizer.annotations[i].clone(),
        });
    }

    // Prepare rules.
    let mut rules = vec![];
    for rule_index in 0..size.rule_count {
        let mut input_entries = vec![];
        for column_index in 0..size.input_clauses_count {
            let input_entry = InputEntry {
                text: recognizer.input_entries[rule_index][column_index].clone(),
            };
            input_entries.push(input_entry);
        }
        let mut output_entries = vec![];
        for column_index in 0..size.output_clauses_count {
            let output_entry = OutputEntry {
                text: recognizer.output_entries[rule_index][column_index].clone(),
            };
            output_entries.push(output_entry);
        }
        let mut annotation_entries = vec![];
        for column_index in 0..size.annotation_clauses_count {
            let annotation_entry = AnnotationEntry {
                text: recognizer.annotation_entries[rule_index][column_index].clone(),
            };
            annotation_entries.push(annotation_entry);
        }
        rules.push(DecisionRule {
            input_entries,
            output_entries,
            annotation_entries,
        });
    }

    // Return the recognized decision table.
    Ok(DecisionTable::new(
        information_item_name,
        input_clauses,
        output_clauses,
        annotations,
        rules,
        hit_policy,
        aggregation,
        preferred_orientation,
        output_label,
    ))
}
