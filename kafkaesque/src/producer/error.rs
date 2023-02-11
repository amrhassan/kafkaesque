use crate::{
    clients::ClientError,
    connection::ConnectionError,
    formats::ErrorCode,
    models::{NodeId, PartitionId, TopicName},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ProducerError>;

#[derive(Debug, Error)]
pub enum ProducerError {
    // #[error("IO error: {0}")]
    // Io(#[from] std::io::Error),
    // #[error("Protocol error: {0}")]
    // Format(#[from] FormatError),
    #[error("Connection error: ${0}")]
    Connection(#[from] ConnectionError),
    #[error("Producing failed with errors: {errors:?}")]
    Producing {
        errors: Vec<(TopicName, PartitionId, ErrorCode)>,
    },
    #[error("Metadata for node (id={node_id}) not found")]
    NodeMetadataNotFound { node_id: NodeId },
    #[error(
        "Leader not found for the topic partition (name={topic_name}, partition_id={partition_id})"
    )]
    TopicPartitionLeaderNotFound {
        topic_name: TopicName,
        partition_id: PartitionId,
    },
    #[error("Client error: {0}")]
    Client(#[from] ClientError),
}
