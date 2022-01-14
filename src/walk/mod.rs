mod filter;
mod mapper;

use crate::{extract, CodeMatcher, LiterateError};
pub use filter::FileFilter;
pub use mapper::PathMapper;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

/// Walks a the directory tree and [`extract`]s matching files.
/// The resulting content is written to the mapped filename.
///
/// Returns the number of extracted files.
pub fn walk_extract<Input, Filter, Matcher, Mapper>(
    input: Input,
    filter: Filter,
    matcher: Matcher,
    mapper: Mapper,
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
        .filter_entry(|e| e.file_type().is_file() && filter.filter_file(e.path()))
    {
        let dir_entry = entry?;
        let file = dir_entry.path();
        let input = File::open(file)?;
        let output = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .create_new(!overwrite)
            .open(mapper.map_path(file.strip_prefix(base_path)?))?;

        extract(input, output, &matcher)?;

        files += 1;
    }

    Ok(files)
}
