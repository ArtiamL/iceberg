pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "iceberg",
    version = "0.1.0", 
    about = "That's just the tip of the iceberg!",
    long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
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
