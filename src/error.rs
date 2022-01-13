use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiterateError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown,
}
