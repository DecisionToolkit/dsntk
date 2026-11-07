use dsntk_common::{DsntkError, ToErrorMessage};
use dsntk_feel::FeelType;

/// Errors related to model evaluation.
#[derive(ToErrorMessage)]
struct ModelEvaluatorError(String);

pub fn err_business_knowledge_model_with_reference_not_found(
    namespace: &str,
    id: &str,
) -> DsntkError {
    ModelEvaluatorError(format!(
        "no business knowledge model with reference: '{namespace}#{id}'"
    ))
    .into()
}

pub fn err_empty_literal_expression() -> DsntkError {
    ModelEvaluatorError("empty literal expression".into()).into()
}

pub fn err_empty_encapsulated_logic() -> DsntkError {
    ModelEvaluatorError("empty encapsulated logic in business knowledge model".into()).into()
}

pub fn err_invalid_item_definition_type(s: &str) -> DsntkError {
    ModelEvaluatorError(format!("invalid item definition type for '{s}'")).into()
}

pub fn err_unsupported_feel_type(feel_type: FeelType, s: &str) -> DsntkError {
    ModelEvaluatorError(format!("unsupported FEEL type: {feel_type} in {s}")).into()
}

pub fn err_empty_feel_type() -> DsntkError {
    ModelEvaluatorError("empty FEEL type".into()).into()
}

pub fn err_empty_function_body() -> DsntkError {
    ModelEvaluatorError("empty function definition body".into()).into()
}
