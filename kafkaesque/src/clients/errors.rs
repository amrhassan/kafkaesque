use crate::{
    connection::ConnectionError,
    formats::ErrorCode,
    models::{NodeId, PartitionId, TopicName},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Connection error: ${0}")]
    Connection(#[from] ConnectionError),
    #[error("TopicCreation error: {errors:?}")]
    TopicCreation { errors: Vec<(TopicName, ErrorCode)> },
    #[error("TopicDeletion error: {errors:?}")]
    TopicDeletion { errors: Vec<(TopicName, ErrorCode)> },
    #[error("Metadata for node (id={node_id}) not found")]
    NodeMetadataNotFound { node_id: NodeId },
    #[error("Producing error: {errors:?}")]
    Producing {
        errors: Vec<(TopicName, PartitionId, ErrorCode)>,
    },
}
