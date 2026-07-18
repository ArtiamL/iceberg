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
        #[arg(long)]
        is_complete: bool,
    },
    Remove {
        title: String,
    },
    List,
}

#[derive(Debug)]
pub enum CliError {
    ValidationError(String),
    IoError(std::io::Error),
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CliError::ValidationError(msg) => write!(f, "Validation Error: {msg}"),
            CliError::IoError(err) => write!(f, "I/O Error: {err}"),
        }
    }
}
