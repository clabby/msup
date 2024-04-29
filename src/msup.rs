use anyhow::Result;
use clap::{Parser, Subcommand};
use generate::generate_batch_definition;
use render::render_batch_doc;
use std::path::PathBuf;

mod generate;
mod render;
mod types;
mod util;

/// CLI args for `msup`
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommand,
}

/// The subcommands of the CLI
#[derive(Subcommand, Clone, Debug)]
enum SubCommand {
    /// Render a LaTeX document for the passed multisig batch definition.
    Render {
        /// The path to the input JSON file
        #[clap(short, long)]
        input: PathBuf,
        /// The path to the output LaTeX file
        #[clap(short, long)]
        output: PathBuf,
    },
    /// Generate a multisig upgrade batch definition.
    Generate {
        /// The path to the output JSON file
        #[clap(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    // Parse the command arguments
    let Args { cmd } = Args::parse();

    match cmd {
        SubCommand::Render { input, output } => render_batch_doc(&input, &output),
        SubCommand::Generate { output } => generate_batch_definition(&output),
    }
}
