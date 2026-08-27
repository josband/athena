use std::io::{self, IsTerminal};

use athena::engine::uci::UciEngine;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: Figure out why we can't find mate for white in "8/8/7p/5K1k/7b/6p1/6P1/7R w - - 2 2"
fn main() {
    if io::stdin().is_terminal() {
        println!("Athena {VERSION} by josband");
    }

    UciEngine::default().start();
}
