use crate::formats::messages::{MetadataRespV0Broker, MetadataRespV0Topic, MetadataResponseV0};
use derive_more::{Display, From, Into};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, From, Into, Display)]
pub struct NodeId(pub i32);

#[derive(Debug, Clone, Copy, From, Into, Display)]
pub struct PartitionId(pub i32);

#[derive(Debug, Clone, Copy, From, Into, Display)]
pub struct PartitionCount(pub i32);

#[derive(Debug, Clone, Copy, From, Into, Display)]
pub struct ReplicationFactor(pub i16);

#[derive(Debug, Clone, From, Into, Display)]
pub struct TopicName(pub String);

#[derive(Debug, Clone, namewise::From)]
#[namewise_from(from_type = "MetadataRespV0Broker")]
pub struct Broker {
    #[namewise_from(from_name = "node_id")]
    pub id: NodeId,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone, namewise::From)]
#[namewise_from(from_type = "MetadataRespV0Topic")]
pub struct Topic {
    pub name: TopicName,
}

#[derive(Debug, Clone)]
pub struct TopicSpec {
    pub name: TopicName,
    pub partition_count: PartitionCount,
    pub replication_factor: ReplicationFactor,
}

#[derive(Debug, Clone, namewise::From)]
#[namewise_from(from_type = "MetadataResponseV0")]
pub struct Metadata {
    #[namewise_from(collect)]
    pub topics: Vec<Topic>,
    #[namewise_from(collect)]
    pub brokers: Vec<Broker>,
}

#[derive(Debug, Clone)]
pub struct TopicConfig(pub HashMap<String, String>);

pub struct TopicAssignments {
    /// Mapping from PartitionIDs to Broker IDs
    pub partition_to_broker_ids: HashMap<PartitionId, Vec<NodeId>>,
}
