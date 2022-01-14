use std::ffi::OsStr;
use std::path::Path;

/// Determines whether a file will be included in the directory tree walking extraction or not.
pub trait FileFilter {
    /// Tests whether this file should be included in the list of files to extract.
    fn filter_file(&self, file: impl AsRef<Path>) -> bool;
}

impl<Filter: FileFilter + ?Sized> FileFilter for Box<Filter> {
    fn filter_file(&self, file: impl AsRef<Path>) -> bool {
        (**self).filter_file(file)
    }
}

fn is_hidden(file: impl AsRef<Path>) -> bool {
    file.as_ref()
        .file_name()
        .and_then(OsStr::to_str)
        .filter(|s| s.starts_with("."))
        .is_some()
}

fn has_extension(extension: &str, file: impl AsRef<Path>) -> bool {
    let file = file.as_ref();

    if let Some(actual) = file.extension().and_then(OsStr::to_str) {
        actual == extension
    } else {
        false
    }
}

impl FileFilter for str {
    fn filter_file(&self, file: impl AsRef<Path>) -> bool {
        let file = file.as_ref();

        !is_hidden(file)
            && has_extension("md", file)
            && has_extension(self, file.with_extension(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_md() {
        assert!("rs".filter_file("foo.rs.md"));
        assert!(!"rs".filter_file("foo.ta.md"));
        assert!(!"rs".filter_file("foo.rs"));
    }
}
