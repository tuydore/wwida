use clap::Parser;

pub(crate) mod components;
pub(crate) mod format;
pub(crate) mod cli;

fn main() {
    let cli = cli::Cli::parse();
    cli.run().unwrap();
}
