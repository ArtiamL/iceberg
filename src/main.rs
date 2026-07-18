use clap::Parser;
use iceberg::{Cli, CliError, Commands, Entry};

// TODO: Do I need Result from main?
fn main() -> Result<(), CliError> {
    let mut entries: Vec<Entry> = Vec::new();

    let cli = Cli::parse();

    let Some(command) = cli.command else {
        // TODO: Loading screen if loading from file is long
        // println!("Loading dashbord:");
        load_dashboard();
        return Ok(());
    };

    match command {
        Commands::Add {
            title,
            info,
            is_complete,
        } => {
            let new_entry = Entry::new(title, info, is_complete)?;
            println!("Added new item:");
            // println!("{new_entry:#?}");
            println!("{new_entry}");
            entries.push(new_entry);

            Ok(())
        }
        Commands::Remove { title } => {
            println!("Removing title: {title}");
            Ok(())
        }
        Commands::List => {
            view_entries(&entries);
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
