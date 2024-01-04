use std::fmt::{Display, Formatter};
use pulldown_cmark::HeadingLevel;
use regex::Regex;

/// Determines whether a heading should be included in the output.
pub trait HeadingMatcher {
    /// Tests whether this heading should be included in the output.
    fn matches(&self, level: HeadingLevel, contents: Option<&str>) -> bool;
}

impl<Matcher: HeadingMatcher + ?Sized> HeadingMatcher for Box<Matcher> {
    fn matches(&self, level: HeadingLevel, contents: Option<&str>) -> bool {
        (**self).matches(level, contents)
    }
}

impl<Matcher: HeadingMatcher + ?Sized> HeadingMatcher for &Matcher {
    fn matches(&self, level: HeadingLevel, contents: Option<&str>) -> bool {
        (*self).matches(level, contents)
    }
}

impl HeadingMatcher for bool {
    fn matches(&self, _: HeadingLevel, _: Option<&str>) -> bool {
        *self
    }
}

impl HeadingMatcher for HeadingLevel {
    fn matches(&self, level: HeadingLevel, _: Option<&str>) -> bool {
        *self == level
    }
}

impl HeadingMatcher for Option<HeadingLevel> {
    fn matches(&self, level: HeadingLevel, _: Option<&str>) -> bool {
        self.map(|expected| expected == level).unwrap_or(true)
    }
}

impl HeadingMatcher for Option<Regex> {
    fn matches(&self, _: HeadingLevel, contents: Option<&str>) -> bool {
        match (self.as_ref(), contents) {
            (Some(regex), Some(contents)) => regex.is_match(contents),
            (Some(_), None) => false,
            (None, _) => true,
        }
    }
}

impl HeadingMatcher for str {
    fn matches(&self, _: HeadingLevel, contents: Option<&str>) -> bool {
        contents.map(|c| self == c).unwrap_or(false)
    }
}

impl HeadingMatcher for Option<&str> {
    fn matches(&self, _: HeadingLevel, contents: Option<&str>) -> bool {
        match self {
            None => true,
            _ => *self == contents
        }
    }
}

/// Matches the header against an regular expression for the contents and an optional level.
/// Exposes control over whether to include fenced code blocks without a language in the output.  
#[derive(Clone, Debug)]
pub struct PatternMatcher {
    level: Option<HeadingLevel>,
    pattern: Option<Regex>,
}

impl Display for PatternMatcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.level.as_ref() {
            None => f.write_str("h*"),
            Some(level) => level.fmt(f)
        }?;

        match self.pattern.as_ref() {
            None => f.write_str(" *"),
            Some(pattern) => write!(f, " {}", pattern)
        }
    }
}

impl PatternMatcher {
    /// Creates a new [`PatternMatcher`].
    pub fn new(level: Option<HeadingLevel>, pattern: Option<Regex>) -> Self {
        Self { level, pattern }
    }
}

impl HeadingMatcher for PatternMatcher {
    fn matches(&self, level: HeadingLevel, contents: Option<&str>) -> bool {
        HeadingMatcher::matches(&self.level, level, contents) && HeadingMatcher::matches(&self.pattern, level, contents)
    }
}
