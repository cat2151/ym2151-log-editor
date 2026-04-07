use cat_self_update_lib::{check_remote_commit, self_update};
use clap::{Parser, Subcommand};

const BUILD_COMMIT_HASH: &str = env!("BUILD_COMMIT_HASH");
const REPO_OWNER: &str = "cat2151";
const REPO_NAME: &str = "ym2151-log-editor";
const MAIN_BRANCH: &str = "main";

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(name = "ym2151-log-editor")]
#[command(about = "YM2151 Event Log Editor")]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Cli {
    /// Read JSON from the clipboard instead of a file
    #[arg(long)]
    pub(crate) clipboard: bool,

    /// JSON file to open. If the file name is `check` or `update`, pass it explicitly as `./check` or `./update`.
    #[arg(value_name = "FILE")]
    pub(crate) file: Option<String>,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Debug, Clone, Subcommand, PartialEq, Eq)]
pub(crate) enum Commands {
    /// Self-update the application from GitHub
    Update,
    /// Compare the build-time commit hash with the remote main branch
    Check,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StartupMode {
    Editor {
        use_clipboard: bool,
        file_arg: Option<String>,
    },
    Update,
    Check,
}

impl From<Cli> for StartupMode {
    fn from(cli: Cli) -> Self {
        match cli.command {
            Some(Commands::Update) => Self::Update,
            Some(Commands::Check) => Self::Check,
            None => Self::Editor {
                use_clipboard: cli.clipboard,
                file_arg: cli.file,
            },
        }
    }
}

pub(crate) fn parse() -> StartupMode {
    Cli::parse().into()
}

pub(crate) fn run_update() -> Result<(), Box<dyn std::error::Error>> {
    self_update(REPO_OWNER, REPO_NAME, &[])?;
    Ok(())
}

pub(crate) fn run_check() -> Result<(), Box<dyn std::error::Error>> {
    let result = check_remote_commit(REPO_OWNER, REPO_NAME, MAIN_BRANCH, BUILD_COMMIT_HASH)?;
    println!("{result}");
    Ok(())
}
