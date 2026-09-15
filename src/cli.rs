use std::fmt::{Display, Formatter, Result};

pub use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "iceberg",
    version = "0.1.0", 
    about = "That's just the tip of the iceberg!",
    long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        title: String,

        #[arg(short, long, help = "requires information (\"\")")]
        info: Option<String>,
        // TODO:
        // #[arg(short, long)]
        // date: Option<T>,
        //
    },
    #[command(visible_alias = "rm")]
    Remove {
        id: u32,
        all: bool,
        #[arg(requires = "all")]
        no_confirm: bool,
    },
    List,
    Complete {
        id: u32,
    },
}

#[derive(Debug)]
pub enum CliError {
    ValidationError(String),
    AlreadyExists(String),
    EntryNotFound(u32),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ConfigDirNotFound,
    NoConfirmError,
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::JsonError(err)
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CliError::ValidationError(err) => write!(f, "Validation Error: {err}"),
            CliError::AlreadyExists(title) => {
                write!(f, "An entry with the title: '{title}' already exists")
            }
            CliError::EntryNotFound(id) => {
                write!(f, "The entry with id: {id} was not found!")
            }
            CliError::IoError(err) => write!(f, "I/O Error: {err}"),
            CliError::ConfigDirNotFound => write!(f, "The config directory was not found!"),
            CliError::JsonError(err) => write!(f, "Json conversion error: {err}"),
            CliError::NoConfirmError => write!(f, "--no-confirm must be used with --all"),
        }
    }
}
