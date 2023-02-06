use crate::formats::messages::{MetadataResponseV0, RespV0Broker, RespV0Topic};
use derive_more::{Display, From, Into};

#[derive(Debug, Clone, Copy, From, Into, Display)]
pub struct NodeId(i32);

#[derive(Debug, Clone, namewise::From)]
#[namewise_from(from_type = "RespV0Broker")]
pub struct Broker {
    pub node_id: NodeId,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone, namewise::From)]
#[namewise_from(from_type = "RespV0Topic")]
pub struct Topic {
    pub name: String,
}

#[derive(Debug, Clone, namewise::From)]
#[namewise_from(from_type = "MetadataResponseV0")]
pub struct Metadata {
    #[namewise_from(collect)]
    pub topics: Vec<Topic>,
    #[namewise_from(collect)]
    pub brokers: Vec<Broker>,
}
