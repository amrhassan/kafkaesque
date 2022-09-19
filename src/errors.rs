use std::string::FromUtf8Error;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, KafkaesqueError>;

#[derive(Debug, Error)]
pub enum KafkaesqueError {
    #[error("UTF8 parsing error: {0}")]
    Utf8Parsing(#[from] FromUtf8Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
