use thiserror::Error;

pub type Result<T> = std::result::Result<T, RemoverError>;

#[derive(Error, Debug)]
pub enum RemoverError {
    #[error("Invalid regex pattern: {0}")]
    InvalidPattern(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 encoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}
