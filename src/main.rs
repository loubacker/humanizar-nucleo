#![forbid(unsafe_code)]

use humanizar_nucleo::infrastructure::server;

#[tokio::main]
async fn main() {
    humanizar_nucleo::infrastructure::diagnostics::install_panic_hook();
    if let Err(error) = server::run().await {
        eprintln!("humanizar-nucleo: {}", error.message());
        std::process::exit(1);
    }
}
