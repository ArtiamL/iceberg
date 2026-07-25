mod cli;
mod entry;
pub mod filesystem;

pub use cli::{Cli, CliError, Commands};
pub use entry::Entry;

#[cfg(test)]
mod tests {
    use super::*;

    use clap::Parser;

    const EXPECT_MSG: &str = "Parsing should have succeeded!";

    #[test]
    fn add_multi_word_title() {
        let args = vec!["iceberg", "add", "test title"];

        let parsed = Cli::try_parse_from(args).expect(EXPECT_MSG);

        assert!(matches!(
            parsed.command,
            Some(Commands::Add { title, .. }) if title == "test title"
        ));
    }

    #[test]
    fn add_single_word_title() {
        let args = vec!["iceberg", "add", "test"];

        let parsed = Cli::try_parse_from(args).expect(EXPECT_MSG);

        assert!(matches!(
                parsed.command,
                Some(Commands::Add { title, .. }) if title == "test"
        ));
    }

    #[ignore = "Clap test"]
    #[test]
    fn add_no_title() {
        let args = vec!["iceberg", "add"];

        let parsed = Cli::try_parse_from(args);
        assert!(parsed.is_err());

        let err = parsed.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn add_info() {
        let args = vec!["iceberg", "add", "title", "--info", "test info"];

        let parsed = Cli::try_parse_from(args).expect(EXPECT_MSG);

        assert!(matches!(
                parsed.command,
                Some(Commands::Add { info: Some(text), .. }) if text == "test info"
        ));
    }

    #[ignore = "Clap test"]
    #[test]
    fn add_no_info() {
        let args = vec!["iceberg", "add", "title", "--info"];

        let parsed = Cli::try_parse_from(args);
        assert!(parsed.is_err());

        let err = parsed.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::InvalidValue);
    }
}
