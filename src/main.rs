use std::process::ExitCode;

mod cli;

use cli::{CLI, Commands, Parser, init};

fn main() -> ExitCode {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Init { force, empty } => {
            if let Err(e) = init(force, empty) {
                eprintln!("[{:?}]: {}", e.kind(), e);
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
    }
}
