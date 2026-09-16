use std::fmt::{Display, Formatter, Result};

pub use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "Iceberg",
    version = "0.1.0", 
    about = "That's just the tip of the Iceberg!",
    long_about = None,
    override_usage = 
        "iceberg                      # Launches the interactive dashboard
       iceberg [COMMAND] <args>     # Run a single action"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "List all Iceberg entries")]
    List,
    #[command(about = "Add an entry to the Iceberg")]
    Add {
        #[arg(help = "The title of the entry. This field is required")]
        title: String,

        #[arg(short, long, help = "Optional: If given, add information (\"\")")]
        info: Option<String>,
        // TODO:
        // #[arg(short, long)]
        // date: Option<T>,
        //
    },
    #[command(visible_alias = "rm", about = "Remove an entry from the Iceberg")]
    Remove {
        #[arg(help = "The ID of the entry to remove")]
        id: Option<u32>,
        #[arg(
            long,
            help = "Removes \x1b[1;4mALL\x1b[22;24m entries in the Iceberg - prompts for confirmation"
        )]
        all: bool,
        #[arg(
            long,
            requires = "all",
            help = "\x1b[31mDanger:\x1b[0m Removes \x1b[1;4mALL\x1b[22;24m entries in the Iceberg \x1b[1;4mwithout\x1b[22;24m confirmation"
        )]
        no_confirm: bool,
    },
    #[command(about = "Mark an Icerberg entry as complete")]
    Complete {
        #[arg(help = "The ID of the entry to mark as complete")]
        id: u32
    },
}

#[derive(Debug)]
pub enum CliError {
    ValidationError(String),
    AlreadyExists(String),
    EntryNotFound(u32),
    IoError(std::io::Error),
    JsonParseError(serde_json::Error),
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
        Self::JsonParseError(err)
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
            Self::JsonParseError(err) => write!(f, "Error parsing JSON: {err}"),
            Self::NoConfirmError => write!(f, "--no-confirm must be used with --all"),
            Self::NoIdGivenError => write!(f, "No ID was given! to remove all entries use --all"),
        }
    }
}
