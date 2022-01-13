use std::fmt::{Display, Formatter, Write};

/// Determines whether a fenced code block should be included in the output.
pub trait CodeMatcher {
    /// Tests whether this fenced code block should be included in the output.
    fn matches(&self, language: Option<&str>) -> bool;
}

impl CodeMatcher for bool {
    fn matches(&self, _: Option<&str>) -> bool {
        *self
    }
}

impl CodeMatcher for str {
    fn matches(&self, language: Option<&str>) -> bool {
        match language {
            Some(language) => self == language,
            None => false,
        }
    }
}

impl CodeMatcher for Option<&str> {
    fn matches(&self, language: Option<&str>) -> bool {
        match (language, *self) {
            (Some(actual), Some(expected)) => actual == expected,
            _ => true,
        }
    }
}

/// Matches the fenced code block against an [`Option`]al language.
/// Exposes control over whether to include fenced code blocks without a language in the output.  
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct LanguageMatcher<'a> {
    pub language: &'a str,
    pub required: bool,
}

impl Display for LanguageMatcher<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.language)?;

        if !self.required {
            f.write_char('?')?;
        }

        Ok(())
    }
}

impl<'a> LanguageMatcher<'a> {
    /// Creates a new [`LanguageMatcher`].
    pub fn new(language: &'a str, required: bool) -> Self {
        LanguageMatcher { language, required }
    }
}

impl CodeMatcher for LanguageMatcher<'_> {
    fn matches(&self, language: Option<&str>) -> bool {
        match language {
            Some(actual) => actual == self.language,
            None => !self.required,
        }
    }
}
