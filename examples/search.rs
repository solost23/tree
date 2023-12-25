//! folder search
//!
//! A simple folder search
//!
//! You can in terminal run:
//!     cargo run --example search -- -p src

use tree::core::Cli;

fn main() {
    let cli = Cli::new();

    cli.run();
}