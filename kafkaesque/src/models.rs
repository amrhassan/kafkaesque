use crate::formats::api::{
    MetadataRespV0Broker, MetadataRespV0Partition, MetadataRespV0Topic, MetadataResponseV0,
};
use derive_more::{Display, From, Into};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, From, Into, Display, PartialEq, Eq, Hash)]
pub struct NodeId(pub i32);

#[derive(Debug, Clone, Copy, From, Into, Display, PartialEq, Eq, Hash)]
pub struct PartitionId(pub i32);

#[derive(Debug, Clone, Copy, From, Into, Display)]
pub struct PartitionCount(pub i32);

#[derive(Debug, Clone, Copy, From, Into, Display)]
pub struct ReplicationFactor(pub i16);

#[derive(Debug, Clone, From, Into, Display, PartialEq, Eq, Hash)]
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
    #[namewise_from(collect)]
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Clone, namewise::From)]
#[namewise_from(from_type = "MetadataRespV0Partition")]
pub struct Partition {
    #[namewise_from(from_name = "partition_index")]
    pub id: PartitionId,

    #[namewise_from(from_name = "leader_id")]
    pub leader: NodeId,

    #[namewise_from(from_name = "replica_nodes", collect)]
    pub replicas: Vec<NodeId>,

    #[namewise_from(from_name = "in_sync_replica_nodes", collect)]
    pub in_sync_replicas: Vec<NodeId>,
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
