#![cfg_attr(docsrs, feature(doc_cfg))]

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use std::io::{BufReader, Read, Write};
use tracing::trace;

mod code;
mod error;
mod heading;

#[cfg(feature = "walk")]
#[cfg_attr(docsrs, doc(cfg(feature = "walk")))]
pub mod walk;

pub use code::{CodeMatcher, LanguageMatcher};
pub use heading::{HeadingMatcher, PatternMatcher};
pub use error::LiterateError;

const MINIMUM_CAPACITY: usize = 1024;

/// Extracts fenced code blocks from the input.
/// If the matcher returns [`true`] for the language, the contents of the code block are written to the output.
/// Otherwise, the contents are ignored.
/// If the language of the fenced code block is blank (empty or blank space), the matcher will get [`None`] as the language.
///
/// Returns the number of extracted bytes.
pub fn extract<Input, Output, Matcher>(
    input: Input,
    mut output: Output,
    matcher: Matcher,
) -> Result<usize, LiterateError>
where
    Input: Read,
    Output: Write,
    Matcher: CodeMatcher,
{
    let mut buffer = BufReader::new(input);
    let mut contents = String::with_capacity(MINIMUM_CAPACITY);

    buffer.read_to_string(&mut contents)?;

    let parser = Parser::new_ext(contents.as_str(), Options::all());

    let mut printing = false;
    let mut bytes = 0;

    for event in parser {
        trace!("Received event: {:?}", event);

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
            _ => {},
        }
    }

    output.flush()?;

    Ok(bytes)
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum QuoteState<'a> {
    Candidate(HeadingLevel, &'a str),
    Printing(HeadingLevel, &'a str),
    Searching
}

/// Extracts fenced code blocks from the input.
/// If the matcher returns [`true`] for the language, the contents of the code block are written to the output.
/// Otherwise, the contents are ignored.
/// If the language of the fenced code block is blank (empty or blank space), the matcher will get [`None`] as the language.
///
/// Returns the number of extracted bytes.
pub fn quote<Input, Output, Matcher>(
    input: Input,
    mut output: Output,
    matcher: Matcher,
) -> Result<usize, LiterateError>
    where
        Input: Read,
        Output: Write,
        Matcher: HeadingMatcher,
{
    let mut buffer = BufReader::new(input);
    let mut contents = String::with_capacity(MINIMUM_CAPACITY);

    buffer.read_to_string(&mut contents)?;

    let parser = Parser::new_ext(contents.as_str(), Options::all());

    let mut state = QuoteState::Searching;
    let mut bytes = 0;

    for (event, range) in parser.into_offset_iter() {
        trace!("Received event: {:?} for excerpt: {:?}", event, &contents[range.clone()]);

        match (state, event) {
            (QuoteState::Searching, Event::Start(Tag::Heading(level, ..))) => {
                state = QuoteState::Candidate(level, &contents[range]);
            }
            (QuoteState::Printing(level, _), Event::Start(Tag::Heading(l, ..))) if level <= l => {
                state = QuoteState::Candidate(level, &contents[range]);
            }
            (QuoteState::Candidate(level, excerpt), Event::Text(body)) => {
                if matcher.matches(level, Some(&*body).filter(|s| !s.trim().is_empty())) {
                    state = QuoteState::Printing(level, excerpt);
                } else {
                    state = QuoteState::Searching;
                }
            }
            (QuoteState::Candidate(..), _) => {
                state = QuoteState::Searching;
            }
            (QuoteState::Printing(..), _) => {
                if bytes <= range.start {
                    let excerpt = &contents[range].as_bytes();
                    output.write_all(excerpt)?;
                    output.write_all(b"\n")?;
                    bytes += excerpt.len();
                }
            }
            (_, _) => {}
        }
    }

    output.flush()?;

    Ok(bytes)
}
