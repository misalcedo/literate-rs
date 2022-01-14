use clap::{AppSettings, Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Parser, PartialEq)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::InferSubcommands))]
#[clap(global_setting(AppSettings::ArgsNegateSubcommands))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub extract: ExtractCommand,
    #[clap(flatten)]
    pub verbosity: Verbosity,
}

impl Arguments {
    /// Parses a new [`Arguments`] instance from the command-line arguments.
    pub fn parse_from_args() -> Self {
        Arguments::parse()
    }
}

/// Set the logging verbosity or level.
#[derive(Args, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Verbosity {
    #[clap(
        short,
        long,
        global(true),
        help_heading("VERBOSITY"),
        conflicts_with_all(&["debug", "trace"]),
        parse(from_occurrences)
    )]
    /// Make the program more talkative.
    pub verbose: usize,
    #[clap(short, long, global(true), help_heading("VERBOSITY"), conflicts_with_all(&["verbose", "trace"]))]
    /// Print debug messages.
    pub debug: bool,
    #[clap(short, long, global(true), help_heading("VERBOSITY"), conflicts_with_all(&["verbose", "debug"]))]
    /// Print trace messages.
    pub trace: bool,
}

/// Defines which fenced code blocks will be included in the output.
#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct LanguageArguments {
    #[clap(short, long, help_heading("LANGUAGE"))]
    /// The language that the fenced code blocks must match to be included in the output.
    pub language: Option<String>,
    #[clap(short, long, help_heading("LANGUAGE"), requires("language"))]
    /// Require fenced code blocks have a language to be included in the output.
    pub required: bool,
}

/// The input and output stream arguments for extracting a single file.
#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct ExtractCommand {
    /// The input stream to read Markdown from. Defaults to STDIN.
    #[clap(short, long, help_heading("IO"))]
    pub input: Option<PathBuf>,
    /// The output stream to write matching fenced code block contents to. Defaults to STDOUT.
    /// The directory path to the file must already exist.
    #[clap(short, long, help_heading("IO"))]
    pub output: Option<PathBuf>,
    /// Overwrite the existing contents in the output stream.
    #[clap(short, long, help_heading("IO"), requires("output"))]
    pub force: bool,
    #[clap(flatten)]
    pub matcher: LanguageArguments,
}

#[derive(Clone, Debug, Eq, Parser, PartialEq)]
/// Walks a directory tree, extracting each matching file found during the walk and outputting the contents to the output directory with the `.md` extension removed.
pub struct WalkCommand {
    /// The input directory to read Markdown from. Defaults to the current directory.
    #[clap(short, long, default_value = ".", help_heading("WALK"))]
    pub input: PathBuf,
    /// The output directory to write matching fenced code block contents to.
    /// The directory path to the file must already exist.
    #[clap(short, long, help_heading("WALK"))]
    pub output: PathBuf,
    /// Overwrite any existing files in the output directory.
    #[clap(short, long, help_heading("WALK"))]
    pub force: bool,
    /// The file extension used to filter the files during the walk.
    /// Only files matching `.<EXTENSION>.md` will be extracted to the output directory.
    #[clap(short, long, help_heading("WALK"))]
    pub extension: String,
    #[clap(flatten)]
    pub matcher: LanguageArguments,
}

/// The sub-command to execute.
#[derive(Clone, Debug, Eq, PartialEq, Subcommand)]
pub enum Commands {
    Walk(WalkCommand),
}
