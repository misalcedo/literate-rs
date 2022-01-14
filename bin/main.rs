use crate::arguments::Commands;
use anyhow::Result;
use arguments::Arguments;
use literate::{CodeMatcher, LanguageMatcher};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use tracing::{info, subscriber::set_global_default, Level};

mod arguments;

fn main() -> Result<()> {
    let arguments = Arguments::from_args();

    set_verbosity(arguments.verbose)?;
    run_subcommand(arguments)
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

fn run_subcommand(arguments: Arguments) -> Result<()> {
    match arguments.command {
        None => {
            let matcher: Box<dyn CodeMatcher> = match arguments.language {
                Some(language) => Box::new(LanguageMatcher::new(language, arguments.required)),
                _ => Box::new(!arguments.required),
            };

            let input: Box<dyn Read> = match arguments.input {
                None => Box::new(stdin()),
                Some(path) => Box::new(File::open(path)?),
            };

            let output: Box<dyn Write> = match arguments.output {
                None => Box::new(stdout()),
                Some(path) => Box::new(
                    File::options()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .create_new(!arguments.overwrite)
                        .open(path)?,
                ),
            };

            literate::extract(input, output, matcher)?;

            Ok(())
        }
        Some(Commands::Walk(command)) => {
            let matcher: Box<dyn CodeMatcher> = match command.language {
                Some(language) => Box::new(LanguageMatcher::new(language, command.required)),
                _ => Box::new(!command.required),
            };

            let files = literate::walk_extract(
                command.input.canonicalize()?,
                command.extension.as_str(),
                command.output,
                matcher,
                command.overwrite,
            )?;

            info!("Extracted {files} into the output directory.");

            Ok(())
        }
    }
}
