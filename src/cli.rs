use std::{
    fs::{metadata, write},
    io::{Error, ErrorKind},
    path::Path,
};

use ansi_term::Style;
pub use clap::{Parser, Subcommand};

use crate::config::{Config, ReaderError};

/// Globals
static DEFAULT_PATH: &str = "mast.toml";
static DEFAULT_CONFIGURATION: &str = r#"[options]
logging = false

[target.build]
about = "Build Rust project"
command = [
    "cargo build",
]
"#;

#[derive(Parser, Debug)]
#[command(
    name = "mast",
    version,
    about = "Modern, Rust-powered task executor and project builder",
    long_about = None,
    author = None,
    arg_required_else_help = true,
)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, default_value_t = false)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize mast configuration file (mast.toml)
    #[command(name = "init", visible_aliases = &["i"])]
    Init {
        /// Force overwrite existing configuration file
        #[arg(long, action, help_heading = "OPTIONS")]
        force: bool,

        /// Initialize empty configuration file without example settings
        #[arg(long, action, help_heading = "OPTIONS")]
        empty: bool,
    },

    #[command(name = "list", visible_aliases = &["l"])]
    List,

    #[command(name = "run", visible_aliases = &["r"])]
    Run { target: String },
}

/// ## Initialize configuration file
///
/// Generate mast.toml
///
/// ### Arguments
/// * `force` (&bool) -
/// * `empty` (&bool) -
///
pub fn init(force: &bool, empty: &bool) -> Result<(), Error> {
    if !force && metadata(DEFAULT_PATH).is_ok() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Configuration file (mast.toml) already exists. Use --force to overwrite.",
        ));
    }

    write(
        DEFAULT_PATH,
        if *empty {
            "# mast config file"
        } else {
            DEFAULT_CONFIGURATION
        },
    )?;

    let path = Path::new(DEFAULT_PATH).canonicalize()?;

    println!("Created configuration at: {}", path.display());

    Ok(())
}

pub fn list() -> Result<(), ReaderError> {
    let config_instance = Config::new();
    let configuration = config_instance.read_config(DEFAULT_PATH)?;

    println!(
        "\n{}\n\n{}",
        Style::new().bold().paint("Options:"),
        configuration.options,
    );
    println!("{}\n", Style::new().bold().paint("Targets:"),);
    for (name, target) in &configuration.target {
        println!("target = \"{}\"", name);
        println!("about = \"{}\"\n", target.about);
    }

    Ok(())
}

pub fn execute(target_name: &String) -> Result<(), ReaderError> {
    let config_instance = Config::new();
    let configuration = config_instance.read_config(DEFAULT_PATH)?;

    match configuration.target.get(target_name) {
        Some(target) => {
            println!("Starting: {}", Style::new().bold().paint(target_name));
            if let Err(e) = target.execute() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("Unknown target name: {}", target_name);
            let _ = list();
            std::process::exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    #[test]
    fn test_init_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        init(&false, &false).unwrap();
        assert!(Path::new(DEFAULT_PATH).exists());
    }
}
