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
        id: Option<u32>,
        #[arg(long)]
        all: bool,
        #[arg(long, requires = "all")]
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
    NoIdGivenError,
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ValidationError(err) => write!(f, "Validation Error: {err}"),
            Self::AlreadyExists(title) => {
                write!(f, "An entry with the title: '{title}' already exists")
            }
            Self::EntryNotFound(id) => {
                write!(f, "The entry with id: {id} was not found!")
            }
            Self::IoError(err) => write!(f, "I/O Error: {err}"),
            Self::ConfigDirNotFound => write!(f, "The config directory was not found!"),
            Self::JsonError(err) => write!(f, "Json conversion error: {err}"),
            Self::NoConfirmError => write!(f, "--no-confirm must be used with --all"),
            Self::NoIdGivenError => write!(f, "No ID was given! to remove all entries use --all"),
        }
    }
}
