use crate::formats::api_keys::ApiKey;
use crate::formats::codec::{Read, Write};
use crate::formats::error_code::ErrorCode;
use crate::formats::request::{ApiVersion, RequestMessage};

#[derive(Debug, Write, RequestMessage)]
#[request_message(version = 0, key = "Metadata")]
pub struct MetadataRequestV0 {
    pub topics: Vec<MetadataReqV0Topic>,
}

#[derive(Debug, Write, Read)]
pub struct MetadataResponseV0 {
    pub brokers: Vec<MetadataRespV0Broker>,
    pub topics: Vec<MetadataRespV0Topic>,
}

#[derive(Debug, Write, Read)]
pub struct MetadataReqV0Topic {
    pub name: String,
}

#[derive(Debug, Write, Read)]
pub struct MetadataRespV0Broker {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Write, Read)]
pub struct MetadataRespV0Topic {
    pub error_code: ErrorCode,
    pub name: String,
    pub partitions: Vec<MetadataRespV0Partition>,
}

#[derive(Debug, Write, Read)]
pub struct MetadataRespV0Partition {
    pub error_code: ErrorCode,
    pub partition_index: i32,
    pub leader_id: i32,
    pub replica_nodes: Vec<i32>,
    pub in_sync_replica_nodes: Vec<i32>,
}
