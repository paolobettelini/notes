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

    /// Execute git pull and compile changes
    #[arg(long = "pull", short = 'p')]
    pub pull: bool,

    /// Compile local git changes
    #[arg(long = "diff", short = 'd')]
    pub diff: bool,

    /// Check only latex folder
    #[arg(long)]
    pub latex: bool,

    /// Check only typst folder
    #[arg(long)]
    pub typst: bool,

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
        // regex, ignore_case, pull and diff are all mutually exclusive
        if self.regex && self.ignore_case {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'ignore_case' flag if 'regex' flag is set",
            )
            .exit();
        } else if self.ignore_case && self.pull {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'ignore_case' flag if 'pull' flag is set",
            )
            .exit();
        } else if self.pull && self.regex {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'pull' flag if 'regex' flag is set",
            )
            .exit();
        } else if self.ignore_case && self.diff {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'ignore_case' flag if 'diff' flag is set",
            )
            .exit();
        } else if self.diff && self.regex {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'diff' flag if 'regex' flag is set",
            )
            .exit();
        } else if self.pull && self.diff {
            let mut cmd = App::command();

            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot use 'pull' flag if 'diff' flag is set",
            )
            .exit();
        }
    }
}
