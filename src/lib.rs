pub mod cli;
pub mod entry;

pub use cli::{Cli, Commands};
pub use entry::Entry;

#[cfg(test)]
mod tests {
    use super::*;

    use clap::Parser;

    #[test]
    fn add_multi_word_title() {
        let args = vec!["iceberg", "add", "test title"];

        let parsed = Cli::try_parse_from(args).expect("Parsing should have succeeded!");

        assert!(matches!(
            parsed.command,
            Commands::Add {
                title,
                info: _,
                is_complete: _,
            } if title == "test title"
        ));
    }

    #[test]
    fn title_is_word() {
        let args = vec!["iceberg", "add", "test"];

        let parsed = Cli::try_parse_from(args).expect("Parsing should have succeeded!");

        assert!(matches!(
                parsed.command,
                Commands::Add {
                    title,
                    info: _,
                    is_complete: _,
                } if title == "test"
        ));
    }

    #[test]
    fn 
}
