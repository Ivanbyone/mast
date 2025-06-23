use std::{io::Result, process::ExitCode};

mod cli;

use cli::{CLI, Commands, Parser, init};

fn main() -> Result<ExitCode> {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Init { force, empty } => {
            init(force, empty)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}
