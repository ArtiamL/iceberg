use clap::Parser;
use iceberg::{Cli, Commands, Entry};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            title,
            info,
            is_complete,
        } => {
            let new_entry = Entry::new(title, info, is_complete);
            println!("Added new item:");
            println!("{new_entry:#?}");
            println!("{new_entry}");
        }
        Commands::Remove { title } => println!("Removing title: {title}"),
        Commands::List => print_list(),
    }
}

//stubs
fn print_list() {}
