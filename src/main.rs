use std::process;

use clap::Parser;

// use iceberg::filesystem::{read, write};
use iceberg::{Cli, CliError, Commands, Entry};

// TODO: Do I need Result from main?
fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");

        process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let mut entries: Vec<Entry> = Vec::new();
    let next_id = entries.iter().map(|e| e.id).max().unwrap_or(0) + 1;

    let cli = Cli::parse();

    let Some(command) = cli.command else {
        // TODO: Loading screen if loading from file is long
        // println!("Loading dashbord:");
        load_dashboard();
        return Ok(());
    };

    match command {
        Commands::Add { title, info } => {
            let new_entry = Entry::new(next_id, title, info)?;
            println!("Added new item:");
            // println!("{new_entry:#?}");
            println!("{new_entry}");
            entries.push(new_entry);
        }
        Commands::Remove { title } => {
            println!("Removing title: {title}");
        }
        Commands::List => {
            view_entries(&entries);
        }
    }

    serialize_to_file(&entries)?;

    Ok(())
}

//stubs
fn view_entries(entries: &[Entry]) {
    if entries.is_empty() {
        println!("There are no entries to show!");
    }

    for entry in entries {
        println!("{entry}");
    }
}

fn load_dashboard() {
    todo!("Implement input reading");
    // let mut stdin = io::stdin();
    //
    // loop {
    //     input.clear()
    //     println!("Welcome to the tip of the Iceberg!");
    //     if
    //     continue;
    // }
}

fn serialize_to_file(entries: &[Entry]) -> Result<(), CliError> {
    let json = serde_json::to_string(&entries)?;

    todo!("Implement file saving");

    Ok(())
}
