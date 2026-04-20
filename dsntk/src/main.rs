/// ÐecisionToolkit entrypoint.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dsntk::actions::do_action().await
}
