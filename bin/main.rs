use anyhow::Result;
use clap::{AppSettings, Parser};
use literate::LanguageMatcher;
use std::io::{stdin, stdout};
use tracing::{subscriber::set_global_default, Level};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::InferSubcommands))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Arguments {
    #[clap(short, long, parse(from_occurrences))]
    /// Make the subcommand more talkative.
    verbose: usize,
    #[clap(short, long)]
    /// The language that the fenced code blocks must match to be included in the output.
    language: Option<String>,
    #[clap(short, long, requires("language"))]
    /// Require fenced code blocks have a language to be included in the output.
    required: bool,
}

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    set_verbosity(arguments.verbose)?;

    match arguments.language {
        Some(language) => literate::extract(
            stdin(),
            stdout(),
            LanguageMatcher::new(language.as_str(), arguments.required),
        )?,
        None => literate::extract(stdin(), stdout(), !arguments.required)?,
    };

    Ok(())
}

fn set_verbosity(occurrences: usize) -> Result<()> {
    let level = match occurrences {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let collector = tracing_subscriber::fmt().with_max_level(level).finish();

    Ok(set_global_default(collector)?)
}
