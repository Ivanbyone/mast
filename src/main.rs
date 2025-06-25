use std::process::ExitCode;

mod cli;
mod config;

use cli::{CLI, Commands, Parser, execute, init, list};

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

        Commands::List {} => {
            if let Err(e) = list() {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }

        Commands::Run { target } => {
            if let Err(e) = execute(target) {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
    }
}
