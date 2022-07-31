use clap::Parser;

pub(crate) mod cli;
pub(crate) mod components;
pub(crate) mod format;

fn main() {
    let cli = cli::Cli::parse();
    cli.run().unwrap();
}
