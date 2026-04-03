use crate::data::ApplicationData;
use crate::utils;
use actix_web::{App, HttpResponse, HttpServer, get, post, web};
use antex::{ColorMode, StyledText, Text};
use dsntk_common::Jsonify;
use dsntk_feel::FeelScope;
use dsntk_workspace::Workspaces;
use std::borrow::Borrow;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::{env, io};

const DEFAULT_PORT: u16 = 22022;
const DEFAULT_HOST: &str = "0.0.0.0";
const VARIABLE_HOST: &str = "DSNTK_HOST";
const VARIABLE_PORT: &str = "DSNTK_PORT";
const VARIABLE_DIR: &str = "DSNTK_DIR";
const CONTENT_TYPE: &str = "application/json";

/// GET handler for evaluating the invocable.
#[get("/evaluate/{path:.*}")]
async fn evaluate_invocable_get(path: web::Path<String>, body: String, data: web::Data<ApplicationData>) -> HttpResponse {
  evaluate(path, body, data)
}

/// POST handler for evaluating the invocable.
#[post("/evaluate/{path:.*}")]
async fn evaluate_invocable_post(path: web::Path<String>, body: String, data: web::Data<ApplicationData>) -> HttpResponse {
  evaluate(path, body, data)
}

fn evaluate(path: web::Path<String>, request_body: String, data: web::Data<ApplicationData>) -> HttpResponse {
  let workspace: &Workspaces = data.workspaces.borrow();
  match dsntk_evaluator::evaluate_context(&FeelScope::default(), &request_body).and_then(|input_data| workspace.evaluate(&path, &input_data)) {
    Ok(value) => HttpResponse::Ok().content_type(CONTENT_TYPE).body(format!(r#"{{"data":{}}}"#, value.jsonify())),
    Err(reason) => HttpResponse::Ok().content_type(CONTENT_TYPE).body(format!(r#"{{"errors":[{{"detail":"{reason}"}}]}}"#)),
  }
}

/// Handler for 404 errors.
async fn not_found() -> HttpResponse {
  HttpResponse::NotFound().content_type(CONTENT_TYPE).body(r#"{"errors":[{"detail":"endpoint not found"}]}"#)
}

#[cfg(feature = "tck")]
fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(crate::tck::evaluate_tck_post);
}

#[cfg(not(feature = "tck"))]
fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(evaluate_invocable_get);
  cfg.service(evaluate_invocable_post);
}

/// Starts the server.
pub async fn start_server(opt_host: Option<String>, opt_port: Option<String>, dirs: Vec<String>, cm: ColorMode, verbose: bool) -> io::Result<()> {
  let application_data = web::Data::new(ApplicationData {
    workspaces: Arc::new(Workspaces::new(&resolve_search_paths(dirs), cm, verbose)),
  });
  let address = get_server_address(opt_host, opt_port);
  Text::new(cm).blue().s("dsntk").clear().space().yellow().s(&address).clear().println();
  HttpServer::new(move || {
    App::new()
      .app_data(application_data.clone())
      .app_data(web::PayloadConfig::new(4 * 1024 * 1024))
      .configure(config)
      .default_service(web::route().to(not_found))
  })
  .bind(address)?
  .run()
  .await
}

/// Returns the host address and the port number, the server will start to listen on.
///
/// The default host and port are defined by `DSNTK_DEFAULT_HOST` and `DSNTK_DEFAULT_PORT` constants.
/// When other values are given as parameters to this function, these will be the actual host and port.
/// Host and port may be also controlled using environment variables:
/// - `DSNTK_HOST` for the host name,
/// - `DSNTK_PORT` for the port name.
///
/// Priority (from highest to lowest):
/// - `opt_host` an `opt_port` parameters,
/// - `DSNTK_HOST` and `DSNTK_PORT` environment variables
/// - `DSNTK_DEFAULT_HOST` and `DSNTK_DEFAULT_PORT` constants.
///
fn get_server_address(opt_host: Option<String>, opt_port: Option<String>) -> String {
  // resolve IP address
  let mut host = DEFAULT_HOST.to_string();
  if let Ok(host_ip_address) = env::var(VARIABLE_HOST) {
    if is_valid_ip_address(&host_ip_address) {
      host = host_ip_address;
    } else {
      eprintln!("invalid host address specified in environment variable {}: {}", VARIABLE_HOST, host_ip_address);
    }
  }
  if let Some(host_ip_address) = opt_host {
    if is_valid_ip_address(&host_ip_address) {
      host = host_ip_address;
    } else {
      eprintln!("invalid host address given as command option: {}", host_ip_address);
    }
  }
  // resolve IP port
  let mut port: u16 = DEFAULT_PORT;
  if let Ok(p_str) = env::var(VARIABLE_PORT) {
    if let Ok(p) = u16::from_str(&p_str) {
      port = p;
    } else {
      eprintln!("invalid port number specified in environment variable {}: {}", VARIABLE_PORT, p_str);
    }
  }
  if let Some(p_str) = opt_port {
    if let Ok(p) = u16::from_str(&p_str) {
      port = p;
    } else {
      eprintln!("invalid port number specified as command option: {}", p_str);
    }
  }
  let server_address = format!("{host}:{port}");
  server_address
}

/// Checks if the specified IP address is correct.
///
/// This function may provide more detailed checks
/// when the [Ipv4Addr](std::net::Ipv4Addr)
/// and [Ipv6Addr](std::net::Ipv6Addr) stabilize.
fn is_valid_ip_address(ip: &str) -> bool {
  ip == "localhost" || ip.parse::<IpAddr>().is_ok()
}

/// Returns directories for loading workspaces.
fn resolve_search_paths(args: Vec<String>) -> Vec<PathBuf> {
  // PRIORITY 1: get search paths from the environment variable
  let paths = utils::paths_from_variable(VARIABLE_DIR);
  if !paths.is_empty() {
    return paths;
  }
  // PRIORITY 2: get search paths from the command line arguments
  let paths = utils::paths_from_arguments(args);
  if !paths.is_empty() {
    return paths;
  }
  // PRIORITY 3: the search path is the current directory
  utils::current_dir()
}
