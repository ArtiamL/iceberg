use clap::{Parser, Subcommand};

use iceberg::Entry;

#[derive(Parser)]
#[command(
    name = "iceberg",
    version = "0.1.0", 
    about = "That's just the tip of the iceberg!",
    long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String, info: Option<String> },
    Remove { title: String },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, info } => {
            let new_entry = Entry::new(title, info);
            println!("Added new item:");
            println!("{new_entry}");
        }
        Commands::Remove { title } => println!("Removing title: {title}"),
        Commands::List => print_list(),
    }
}

//stubs
fn print_list() {}
