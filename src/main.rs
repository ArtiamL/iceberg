use std::{
    fmt::format,
    fs::{self},
    io::{self, Write},
    process,
};

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
    let mut entries: Vec<Entry> = deserialize_from_file().unwrap_or_default();
    let next_id = entries.iter().map(|e| e.id).max().unwrap_or(0) + 1;

    let cli = Cli::parse();

    let Some(command) = cli.command else {
        // TODO: Loading screen if loading from file is long
        // println!("Loading dashbord:");
        load_dashboard();
        return Ok(());
    };

    match command {
        Commands::List => {
            view_entries(&entries);
            Ok(())
        }
        Commands::Add { title, info } => {
            let new_entry = Entry::new(next_id, title, info)?;
            println!("Added new item:");
            println!("{new_entry}");
            entries.push(new_entry);
            serialize_to_file(&entries)
        }
        Commands::Remove {
            id,
            all,
            no_confirm,
        } => {
            match (id, all, no_confirm) {
                (Some(id), _, _) => {
                    // id given, ignore everything else (remove only entry
                    // with id)
                    let Some(entry) = entries.iter().position(|e| e.id == id) else {
                        return Err(CliError::EntryNotFound(id));
                    };
                    let removed = entries.remove(entry); // Decide if elem order is needed - if not use
                    // swap_remove

                    serialize_to_file(&entries)?;

                    println!("Removed entry:\n{removed}");
                    Ok(())
                }
                (None, false, _) => {
                    // no id, no --all (err)
                    Err(CliError::NoIdGivenError)
                }
                (None, true, false) => {
                    // no id, --all given, --no-confirm not given (ask to confirm)
                    print!("Are you sure? [y/N]: ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let confirmed = matches!(input.trim().to_lowercase().as_str(), "y" | "yes");

                    if !confirmed {
                        println!("Cancelled.");
                        return Ok(());
                    }

                    entries.clear();

                    serialize_to_file(&entries)?;
                    println!("Cleared all entries!");
                    Ok(())
                }
                (None, true, true) => {
                    // no id, --all given, --no-confirm given (remove everything)
                    entries.clear();

                    serialize_to_file(&entries)?;
                    println!("Cleared all entries!");
                    Ok(())
                }
            }
        }
        Commands::Complete { id } => {
            let Some(entry) = entries.iter_mut().find(|e| e.id == id) else {
                return Err(CliError::EntryNotFound(id));
            };
            entry.is_complete = true;

            let msg = format!("Marked entry as complete:\n{entry}");
            serialize_to_file(&entries)?;

            println!("{msg}");
            Ok(())
        }
    }
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

    let data_dir = dirs::data_local_dir()
        .ok_or(CliError::ConfigDirNotFound)?
        .join("iceberg");

    std::fs::create_dir_all(&data_dir)?;

    fs::write(data_dir.join("entries.json"), json)?;

    Ok(())
}

fn deserialize_from_file() -> Result<Vec<Entry>, CliError> {
    let data_dir = dirs::data_local_dir()
        .ok_or(CliError::ConfigDirNotFound)?
        .join("iceberg");

    let file_data = fs::read_to_string(data_dir.join("entries.json"))?;
    Ok(serde_json::from_str(&file_data)?)
}
