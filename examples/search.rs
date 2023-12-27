//! search
//!
//! A simple search
//!
//! You can in terminal run:
//!     cargo run --example search -- -p src
//!
//! You can in terminal run:
//! Only search directory
//!     cargo run --example search -- -p src -d
//!
//! You can in terminal run:
//! Set search depth
//!     cargo run --example search -- -p src -m 1
//!

use tree::core::Cli;

fn main() {
    let cli = Cli::new();

    cli.run();
}