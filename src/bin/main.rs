//! A simple core command

use tree::core::Cli;
fn main() {
    let cli = Cli::new();

    cli.run();
}
