use std::io::{self, IsTerminal};

use athena::engine::uci::UciEngine;
use tracing::info;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    init_tracing();

    if io::stdin().is_terminal() {
        println!("Athena {VERSION} by josband");
    }

    info!("Starting Athena...");

    UciEngine::default().start();

    info!("Gracefully shutting down Athena...");
}

fn init_tracing() {
    tracing_subscriber::fmt().with_writer(io::stderr).init();
}
