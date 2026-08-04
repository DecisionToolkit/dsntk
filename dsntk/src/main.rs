/// Use mimalloc instead of the system allocator.
///
/// Evaluation spends about half its time in `malloc` and `free`, and macOS
/// `libmalloc` never returns freed small blocks to the operating system. So peak
/// resident memory holds every page the parse phase touched, long after the parse
/// data dies. mimalloc purges free pages, so the peak tracks live data instead.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// ÐecisionToolkit entrypoint.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dsntk::actions::do_action().await
}
