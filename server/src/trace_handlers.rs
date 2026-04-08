//! Trace-enabled evaluation handlers for the evaluation viewer.

use crate::data::ApplicationData;
use actix_web::{get, post, web, HttpResponse};
use dsntk_common::Jsonify;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelNumber, Name};
use dsntk_workspace::Workspaces;
use serde::Deserialize;
use std::borrow::Borrow;
use std::str::FromStr;

const CONTENT_TYPE: &str = "application/json";

/// Request body for traced evaluation.
#[derive(Deserialize)]
struct EvaluateTraceRequest {
  /// The invocable path (e.g. "io/dsntk/my-model/my-decision").
  path: String,
  /// Input values as a JSON object.
  inputs: serde_json::Value,
}

/// Response model info for GET /api/v1/models.
#[derive(serde::Serialize)]
struct ModelInfo {
  path: String,
  workspace: String,
  model_namespace: String,
  model_name: String,
  invocable_name: String,
  inputs: Vec<String>,
}

/// GET /api/v1/models - returns available models and their metadata.
#[get("/api/v1/models")]
async fn list_models(data: web::Data<ApplicationData>) -> HttpResponse {
  let workspaces: &Workspaces = data.workspaces.borrow();
  let models: Vec<ModelInfo> = workspaces
    .list_invocables()
    .map(|(path, (workspace, ns, model, invocable))| ModelInfo {
      path: path.clone(),
      workspace: workspace.clone(),
      model_namespace: ns.clone(),
      model_name: model.clone(),
      invocable_name: invocable.clone(),
      inputs: vec![],
    })
    .collect();
  HttpResponse::Ok()
    .content_type(CONTENT_TYPE)
    .body(serde_json::to_string(&models).unwrap_or_else(|_| "[]".to_string()))
}

/// POST /api/v1/evaluate-trace - evaluates an invocable with full trace.
#[post("/api/v1/evaluate-trace")]
async fn evaluate_trace(body: web::Json<EvaluateTraceRequest>, data: web::Data<ApplicationData>) -> HttpResponse {
  let workspaces: &Workspaces = data.workspaces.borrow();
  let input_context = match json_to_feel_context(&body.inputs) {
    Some(ctx) => ctx,
    None => {
      return HttpResponse::BadRequest()
        .content_type(CONTENT_TYPE)
        .body(r#"{"errors":[{"detail":"inputs must be a JSON object"}]}"#);
    }
  };
  match workspaces.evaluate_traced(&body.path, &input_context) {
    Ok((value, trace)) => {
      let result = serde_json::json!({
        "data": serde_json::from_str::<serde_json::Value>(&value.jsonify()).unwrap_or(serde_json::Value::Null),
        "trace": trace,
      });
      HttpResponse::Ok().content_type(CONTENT_TYPE).body(result.to_string())
    }
    Err(reason) => HttpResponse::Ok().content_type(CONTENT_TYPE).body(format!(r#"{{"errors":[{{"detail":"{reason}"}}]}}"#)),
  }
}

/// Converts a JSON object into a FEEL context.
fn json_to_feel_context(value: &serde_json::Value) -> Option<FeelContext> {
  let obj = value.as_object()?;
  let mut ctx = FeelContext::default();
  for (key, val) in obj {
    let name: Name = key.clone().into();
    let feel_value = json_to_feel_value(val);
    ctx.set_entry(&name, feel_value);
  }
  Some(ctx)
}

/// Converts a JSON value into a FEEL value.
fn json_to_feel_value(value: &serde_json::Value) -> Value {
  match value {
    serde_json::Value::Null => Value::Null(None),
    serde_json::Value::Bool(b) => Value::Boolean(*b),
    serde_json::Value::Number(n) => {
      if let Some(i) = n.as_i64() {
        Value::Number(FeelNumber::from(i))
      } else {
        // Parse via string representation to preserve precision.
        FeelNumber::from_str(&n.to_string()).map(Value::Number).unwrap_or(Value::Null(None))
      }
    }
    serde_json::Value::String(s) => Value::String(s.clone()),
    serde_json::Value::Array(arr) => Value::List(arr.iter().map(json_to_feel_value).collect()),
    serde_json::Value::Object(obj) => {
      let mut ctx = FeelContext::default();
      for (key, val) in obj {
        let name: Name = key.clone().into();
        ctx.set_entry(&name, json_to_feel_value(val));
      }
      Value::Context(ctx)
    }
  }
}
