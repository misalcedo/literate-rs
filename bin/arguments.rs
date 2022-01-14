use clap::{AppSettings, Parser, Subcommand};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Deref;

lazy_static! {
    static ref NON_EMPTY: Regex = Regex::new(r"^[^\s].*$").expect("Invalid regular expression.");
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::InferSubcommands))]
#[clap(global_setting(AppSettings::ArgsNegateSubcommands))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Arguments {
    #[clap(short, long, global(true), parse(from_occurrences))]
    /// Make the subcommand more talkative.
    pub verbose: usize,
    /// The sub-command to execute.
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(
    short,
    long,
    validator_regex(NON_EMPTY.deref(), "must not be empty or start with blank space")
    )]
    /// The language that the fenced code blocks must match to be included in the output.
    pub language: Option<String>,
    #[clap(short, long, requires("language"))]
    /// Require fenced code blocks have a language to be included in the output.
    pub required: bool,
}

impl Arguments {
    /// Creates a new [`Arguments`] instance from the command-line arguments.
    pub fn from_args() -> Self {
        Arguments::parse()
    }
}

#[derive(Debug, Parser)]
/// Walks a directory tree, extracting each matching file found during the walk and outputting the contents to the output directory with the `.md` extension removed.
pub struct WalkCommand {}

#[derive(Debug, Subcommand)]
#[clap()]
pub enum Commands {
    Walk(WalkCommand),
}
