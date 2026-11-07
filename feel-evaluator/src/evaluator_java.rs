//! # Evaluator for Java external functions

use dsntk_feel::dto::ValueDto;
use dsntk_feel::value_null;
use dsntk_feel::values::Value;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(reqwest::blocking::Client::new);

const JAVA_RPC_SERVER_URL: &str = "http://127.0.0.1:22023/api/rest/v1/rpc/evaluate";

#[derive(Serialize)]
struct RequestDto {
    /// Name of the Java class, where called method is defined.
    #[serde(rename = "className")]
    class_name: String,
    /// Name of the static method to be called.
    #[serde(rename = "methodName")]
    method_name: String,
    /// List of parameter types of the called method.
    #[serde(rename = "parameterTypes")]
    parameter_types: Vec<String>,
    /// Arguments to be passed to called method.
    #[serde(rename = "arguments")]
    argument_values: Vec<ValueDto>,
}

#[derive(Deserialize)]
struct ResponseDto {
    /// Response payload when calling external function succeeds.
    #[serde(rename = "data")]
    data: Option<ValueDto>,
    /// Error message on failure.
    #[serde(rename = "error")]
    error: Option<String>,
}

/// Evaluates external Java function.
pub fn evaluate_external_java_function(
    class_name: &str,
    method_signature: &str,
    arguments: &[Value],
) -> Value {
    let mut parts = method_signature.trim().split('(');
    let Some(method_name) = parts.next() else {
        return value_null!("no method name in method signature");
    };
    let Some(parameter_type_names) = parts.next() else {
        return value_null!("no parameter types in method signature");
    };
    let parameter_types: Vec<String> = parameter_type_names
        .trim()
        .trim_end_matches(')')
        .split(',')
        .filter_map(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        })
        .collect();
    if parameter_types.len() != arguments.len() {
        return value_null!(
            "the number of parameter types ({}) differs from the number of arguments ({})",
            parameter_types.len(),
            arguments.len()
        );
    }
    let mut argument_values = vec![];
    for argument in arguments {
        match ValueDto::try_from(argument) {
            Ok(value_dto) => argument_values.push(value_dto),
            Err(reason) => return value_null!("{}", reason.to_string()),
        };
    }
    let request_dto = RequestDto {
        class_name: class_name.to_string(),
        method_name: method_name.to_string(),
        parameter_types,
        argument_values,
    };
    match CLIENT.post(JAVA_RPC_SERVER_URL).json(&request_dto).send() {
        Ok(response) => match response.json::<ResponseDto>() {
            Ok(response_dto) => {
                if let Some(reason) = response_dto.error {
                    value_null!("{}", reason)
                } else if let Some(value_dto) = response_dto.data {
                    match Value::try_from(&value_dto) {
                        Ok(value) => value,
                        Err(reason) => value_null!("{}", reason),
                    }
                } else {
                    value_null!("no data in response")
                }
            }
            Err(reason) => value_null!("{}", reason),
        },
        Err(reason) => value_null!("{}", reason),
    }
}
