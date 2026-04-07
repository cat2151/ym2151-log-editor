use clap::Parser;

use crate::cli::{Cli, Commands, StartupMode};

#[test]
fn parse_editor_file_argument() {
    let cli = Cli::try_parse_from(["ym2151-log-editor", "test_data/sample.json"]).unwrap();

    assert_eq!(cli.command, None);
    assert!(!cli.clipboard);
    assert_eq!(cli.file.as_deref(), Some("test_data/sample.json"));
    assert_eq!(
        StartupMode::from(cli),
        StartupMode::Editor {
            use_clipboard: false,
            file_arg: Some("test_data/sample.json".to_string()),
        }
    );
}

#[test]
fn parse_editor_file_argument_named_like_subcommand_with_explicit_path() {
    let cli = Cli::try_parse_from(["ym2151-log-editor", "./check"]).unwrap();

    assert_eq!(cli.command, None);
    assert!(!cli.clipboard);
    assert_eq!(cli.file.as_deref(), Some("./check"));
    assert_eq!(
        StartupMode::from(cli),
        StartupMode::Editor {
            use_clipboard: false,
            file_arg: Some("./check".to_string()),
        }
    );
}

#[test]
fn parse_editor_file_argument_named_like_update_subcommand_with_explicit_path() {
    let cli = Cli::try_parse_from(["ym2151-log-editor", "./update"]).unwrap();

    assert_eq!(cli.command, None);
    assert!(!cli.clipboard);
    assert_eq!(cli.file.as_deref(), Some("./update"));
    assert_eq!(
        StartupMode::from(cli),
        StartupMode::Editor {
            use_clipboard: false,
            file_arg: Some("./update".to_string()),
        }
    );
}

#[test]
fn parse_editor_clipboard_flag() {
    let cli = Cli::try_parse_from(["ym2151-log-editor", "--clipboard"]).unwrap();

    assert_eq!(cli.command, None);
    assert!(cli.clipboard);
    assert_eq!(cli.file, None);
    assert_eq!(
        StartupMode::from(cli),
        StartupMode::Editor {
            use_clipboard: true,
            file_arg: None,
        }
    );
}

#[test]
fn parse_update_subcommand() {
    let cli = Cli::try_parse_from(["ym2151-log-editor", "update"]).unwrap();

    assert_eq!(cli.command, Some(Commands::Update));
    assert_eq!(StartupMode::from(cli), StartupMode::Update);
}

#[test]
fn parse_check_subcommand() {
    let cli = Cli::try_parse_from(["ym2151-log-editor", "check"]).unwrap();

    assert_eq!(cli.command, Some(Commands::Check));
    assert_eq!(StartupMode::from(cli), StartupMode::Check);
}

#[test]
fn subcommand_rejects_extra_file_argument() {
    let result = Cli::try_parse_from(["ym2151-log-editor", "check", "test_data/sample.json"]);

    assert!(result.is_err());
}
