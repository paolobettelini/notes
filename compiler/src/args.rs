use clap::{error::ErrorKind, CommandFactory, Parser};

/// Stellar notes compiler CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {
    /// Use a regex
    #[arg(short, long)]
    pub regex: bool,

    /// Ignore case
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Check only snippets folder
    #[arg(long)]
    pub snippets: bool,

    /// Compile current git status files last pull
    #[arg(long = "pull", short = 'p')]
    pub git: bool,

    /// Check only latex folder
    #[arg(long)]
    pub latex: bool,

    /// Check only pages folder
    #[arg(long)]
    pub pages: bool,

    /// Check only courses folder
    #[arg(long)]
    pub courses: bool,

    /// Check only universes folder
    #[arg(long)]
    pub universes: bool,

    /// Matches only the files that contain this value
    /// NOTE: is it not affected by --regex and --ignore-case
    #[arg(long)]
    pub containing: Option<String>,

    /// Compile query
    #[clap()]
    pub inputs: Option<Vec<String>>,
}

impl App {
    pub fn validate(&self) {
        // regex, ignore_case, git are all mutually exclusive
        if self.regex && self.ignore_case {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'ignore_case' flag if 'regex' flag is set",
            )
            .exit();
        } else if self.ignore_case && self.git {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'ignore_case' flag if 'git' flag is set",
            )
            .exit();
        } else if self.git && self.regex {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'git' flag if 'regex' flag is set",
            )
            .exit();
        }
    }
}
