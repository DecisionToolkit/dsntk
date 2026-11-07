//! # Evaluator for PMML external functions

use dsntk_feel::value_null;
use dsntk_feel::values::Value;

/// Evaluates external PMML function.
pub fn evaluate_external_pmml_function(
    document: &str,
    model_name: &str,
    _arguments: &[Value],
) -> Value {
    match (document, model_name) {
        ("", _) => value_null!("PMML document not specified"),
        (_, "") => value_null!("PMML model name not specified"),
        _ => Value::String(format!(
            "PMML, document = {document}, model name = {model_name}"
        )),
    }
}
