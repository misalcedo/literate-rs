use crate::arguments::{Commands, ExtractCommand, LanguageArguments, Verbosity, WalkCommand};
use anyhow::Result;
use arguments::Arguments;
use literate::{CodeMatcher, LanguageMatcher, LiterateError};
use std::fs::File;
use std::io::ErrorKind::BrokenPipe;
use std::io::{stdin, stdout, Read, Write};
use tracing::{info, subscriber::set_global_default, Level};

mod arguments;

fn main() -> Result<()> {
    let arguments = Arguments::parse_from_args();

    set_verbosity(arguments.verbosity)?;
    run_subcommand(arguments)
}

fn set_verbosity(verbosity: Verbosity) -> Result<()> {
    let mut level = match verbosity.verbose {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE,
    };

    if verbosity.trace {
        level = Level::TRACE;
    } else if verbosity.debug {
        level = Level::DEBUG;
    }

    let collector = tracing_subscriber::fmt().with_max_level(level).finish();

    Ok(set_global_default(collector)?)
}

fn run_subcommand(arguments: Arguments) -> Result<()> {
    match arguments.command {
        None => run_extraction(arguments.extract),
        Some(Commands::Walk(command)) => run_walk(command),
    }
}

fn run_extraction(arguments: ExtractCommand) -> Result<()> {
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
                .create_new(!arguments.force)
                .open(path)?,
        ),
    };

    let matcher: Box<dyn CodeMatcher> = arguments.matcher.into();
    match literate::extract(input, output, matcher) {
        Ok(bytes) => Ok(info!("Extracted {bytes} bytes into the output directory.")),
        Err(LiterateError::IO(error)) if error.kind() == BrokenPipe => Ok(()),
        Err(error) => Ok(eprintln!("{error}")),
    }
}

fn run_walk(command: WalkCommand) -> Result<()> {
    let matcher: Box<dyn CodeMatcher> = command.matcher.into();

    let files = literate::walk::extract(
        command.input.canonicalize()?,
        command.extension.as_str(),
        command.output,
        matcher,
        command.force,
    )?;

    info!("Extracted {files} files into the output directory.");

    Ok(())
}

impl From<LanguageArguments> for Box<dyn CodeMatcher> {
    fn from(arguments: LanguageArguments) -> Self {
        match arguments.language {
            Some(language) => Box::new(LanguageMatcher::new(language, arguments.required)),
            _ => Box::new(!arguments.required),
        }
    }
}
