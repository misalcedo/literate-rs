use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiterateError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[cfg(feature = "walk")]
    #[error(transparent)]
    Walk(#[from] walkdir::Error),
    #[cfg(feature = "walk")]
    #[error(transparent)]
    Prefix(#[from] std::path::StripPrefixError),
    #[error("Unknown error")]
    Unknown,
}
