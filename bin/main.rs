use anyhow::Result;
use arguments::Arguments;
use literate::{CodeMatcher, LanguageMatcher};
use std::io::{stdin, stdout};
use tracing::{subscriber::set_global_default, Level};

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

            literate::extract(stdin(), stdout(), &*matcher)?;

            Ok(())
        }
        _ => Ok(()),
    }
}
