use std::path::{Path, PathBuf};

/// Determines the output path of the contents extracted from an input file.
#[cfg_attr(docsrs, doc(cfg(feature = "walk")))]
pub trait PathMapper {
    /// Maps the extracted file's path relative to the input path to an output path.
    fn map_path(&self, file: impl AsRef<Path>) -> PathBuf;
}

impl<Mapper: PathMapper + ?Sized> PathMapper for Box<Mapper> {
    fn map_path(&self, file: impl AsRef<Path>) -> PathBuf {
        (**self).map_path(file)
    }
}

impl<Mapper: PathMapper + ?Sized> PathMapper for &Mapper {
    fn map_path(&self, file: impl AsRef<Path>) -> PathBuf {
        (*self).map_path(file)
    }
}

impl PathMapper for PathBuf {
    fn map_path(&self, file: impl AsRef<Path>) -> PathBuf {
        self.join(file.as_ref().with_extension(""))
    }
}

impl PathMapper for str {
    fn map_path(&self, file: impl AsRef<Path>) -> PathBuf {
        Path::new(self).join(file.as_ref().with_extension(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_file() {
        assert_eq!(
            "target/".map_path("hello/world/foo.rs.md"),
            Path::new("target/hello/world/foo.rs")
        );
        assert_eq!(
            "target/".map_path("hello/world/foo.rs"),
            Path::new("target/hello/world/foo")
        );
        assert_eq!("target/".map_path("/hello/foo.rs"), Path::new("/hello/foo"));
    }
}
