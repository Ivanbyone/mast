use std::{
    fs::{metadata, write},
    io::{Error, ErrorKind, Result},
};

pub use clap::{Parser, Subcommand};

/// Constants
static DEFAULT_PATH: &str = "./mast.toml";
static DEFAULT_CONFIGURATION: &str = r#"[options]
logging = false

[target.build]
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
}

/// ## Initialize configuration file
///
/// Generate mast.toml
///
/// ### Arguments
/// * `force`
/// * `empty`
///
pub fn init(force: &bool, empty: &bool) -> Result<()> {
    println!("Creating mast.toml...");

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

    Ok(())
}
