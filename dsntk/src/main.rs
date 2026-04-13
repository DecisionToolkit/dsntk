//! # ÐecisionToolkit

#[macro_use]
extern crate dsntk_macros;

mod actions;
mod built_in_examples;
mod errors;

/// Main entrypoint of the application.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  actions::do_action().await
}
