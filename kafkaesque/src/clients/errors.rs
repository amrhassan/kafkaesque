use super::{ClientConfigBuilderError, TopicName};
use crate::formats::{ErrorCode, FormatError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Protocol error: {0}")]
    Format(#[from] FormatError),
    #[error("ClientConfig builder error: {0}")]
    ConfigBuilderError(#[from] ClientConfigBuilderError),
    #[error("TopicCreation error: {errors:?}")]
    TopicCreation { errors: Vec<(TopicName, ErrorCode)> },
    #[error("TopicDeletion error: {errors:?}")]
    TopicDeletion { errors: Vec<(TopicName, ErrorCode)> },
}
