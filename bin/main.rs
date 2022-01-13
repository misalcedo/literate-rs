use anyhow::Result;
use clap::{AppSettings, Parser};
use tracing::{subscriber::set_global_default, Level};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::InferSubcommands))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Arguments {
    #[clap(short, long, parse(from_occurrences))]
    /// Make the subcommand more talkative.
    verbose: usize,
}

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    set_verbosity(arguments.verbose)?;

    Ok(println!("Hello, world!"))
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
