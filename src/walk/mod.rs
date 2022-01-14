mod filter;
mod mapper;

use crate::{extract, CodeMatcher, LiterateError};
pub use filter::FileFilter;
pub use mapper::PathMapper;
use std::fs::{create_dir_all, File};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Walks a the directory tree and [`extract`]s matching files.
/// The resulting content is written to the mapped filename.
///
/// Returns the number of extracted files.
pub fn walk_extract<Input, Filter, Matcher, Mapper>(
    input: Input,
    filter: Filter,
    mapper: Mapper,
    matcher: Matcher,
    overwrite: bool,
) -> Result<usize, LiterateError>
where
    Input: AsRef<Path>,
    Filter: FileFilter,
    Matcher: CodeMatcher,
    Mapper: PathMapper,
{
    let mut files = 0;

    let base_path = input.as_ref();

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_entry(|e| filter_dir_entry(e, &filter))
    {
        let dir_entry = entry?;
        if dir_entry.file_type().is_dir() {
            continue;
        }

        let file = dir_entry.path();
        let input = File::open(file)?;
        let path = mapper.map_path(file.strip_prefix(base_path)?);

        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }

        let output = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .create_new(!overwrite)
            .open(path)?;

        extract(input, output, &matcher)?;

        files += 1;
    }

    Ok(files)
}

fn filter_dir_entry<Filter>(entry: &DirEntry, filter: Filter) -> bool
where
    Filter: FileFilter,
{
    let valid_directory = entry.file_type().is_dir()
        && entry
            .file_name()
            .to_str()
            .filter(|s| s.starts_with("."))
            .is_none();

    valid_directory || filter.filter_file(entry.path())
}
