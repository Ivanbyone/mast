use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, fs::read_to_string, process::Command};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub options: Options,
    pub target: HashMap<String, Target>,
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"about\": \"{}\"\n", self.options,)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub logging: Option<bool>,
}

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"logging\": \"{}\"\n",
            self.logging
                .map(|v| v.to_string())
                .unwrap_or("null".to_string())
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub about: String,
    pub command: Vec<String>,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"about\": \"{}\"\n", self.about,)
    }
}

impl Target {
    pub fn execute(&self) -> Result<(), String> {
        if self.command.is_empty() {
            return Err("Empty target name".into());
        }

        for cmd_str in &self.command {
            let parts: Vec<&str> = cmd_str.split_whitespace().collect();
            if parts.is_empty() {
                return Err(format!("Unknown command: '{}'", cmd_str));
            }

            let (program, args) = parts.split_first().unwrap();

            let status = Command::new(program)
                .args(args)
                .status()
                .map_err(|e| format!("Executer error '{}': {}", cmd_str, e))?;

            // Проверяем статус
            if !status.success() {
                return Err(format!(
                    "Command '{}' exited with error (status code: {:?})",
                    cmd_str,
                    status.code()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ReaderError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl From<std::io::Error> for ReaderError {
    fn from(err: std::io::Error) -> Self {
        ReaderError::Io(err)
    }
}

impl From<toml::de::Error> for ReaderError {
    fn from(err: toml::de::Error) -> Self {
        ReaderError::Parse(err)
    }
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::Io(e) => write!(f, "[IOError]: {}", e),
            ReaderError::Parse(e) => write!(f, "[ParseError]: {}", e),
        }
    }
}

pub struct Config;

impl Config {
    pub fn new() -> Self {
        Config
    }

    pub fn read_config(&self, path: &str) -> Result<Configuration, ReaderError> {
        let str_config = read_to_string(path).map_err(ReaderError::Io)?;
        let parsed_config = toml::from_str(&str_config).map_err(ReaderError::Parse)?;

        Ok(parsed_config)
    }
}
