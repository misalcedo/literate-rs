use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::io::{BufReader, Read, Write};
use tracing::trace;

mod error;
mod matcher;

pub use error::LiterateError;
pub use matcher::{CodeMatcher, LanguageMatcher};

const MINIMUM_CAPACITY: usize = 1024;

/// Extracts fenced code blocks from the input.
/// If the matcher returns [`true`] for the language, the contents of the code block are written to the output.
/// Otherwise, the contents are ignored.
/// If the language of the fenced code block is blank (empty or blank space), the matcher will get [`None`] as the language.
pub fn extract<Input: Read, Output: Write, Matcher: CodeMatcher>(
    input: Input,
    mut output: Output,
    matcher: Matcher,
) -> Result<usize, LiterateError> {
    let mut buffer = BufReader::new(input);
    let mut contents = String::with_capacity(MINIMUM_CAPACITY);

    buffer.read_to_string(&mut contents)?;

    let parser = Parser::new_ext(contents.as_str(), Options::all());

    let mut printing = false;
    let mut bytes = 0;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(language))) => {
                printing = matcher.matches(Some(&*language).filter(|s| !s.trim().is_empty()));
            }
            Event::Text(body) if printing => {
                output.write_all(body.as_bytes())?;
                bytes += body.len();
            }
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                printing = false;
            }
            event => trace!("Received event: {:?}", event),
        }
    }

    output.flush()?;

    Ok(bytes)
}
