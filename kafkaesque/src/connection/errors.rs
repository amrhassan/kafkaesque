use crate::formats::FormatError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectionError>;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Format error: {0}")]
    Format(#[from] FormatError),
}
