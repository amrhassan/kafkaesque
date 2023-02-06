use crate::formats::api_keys::ApiKey;
use crate::formats::codec::{Read, Write};
use crate::formats::request::{ApiVersion, RequestMessage};
use crate::formats::response::ErrorCode;

#[derive(Debug, Write, RequestMessage)]
#[request_message(version = 0, key = "Metadata")]
pub struct MetadataRequestV0 {
    pub topics: Vec<ReqV0Topic>,
}

#[derive(Debug, Write, Read)]
pub struct MetadataResponseV0 {
    pub brokers: Vec<RespV0Broker>,
    pub topics: Vec<RespV0Topic>,
}

#[derive(Debug, Write, Read)]
pub struct ReqV0Topic {
    pub name: String,
}

#[derive(Debug, Write, Read)]
pub struct RespV0Broker {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Write, Read)]
pub struct RespV0Topic {
    pub error_code: ErrorCode,
    pub name: String,
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Write, Read)]
pub struct Partition {
    pub error_code: ErrorCode,
    pub partition_index: i32,
    pub leader_id: i32,
    pub replica_nodes: Vec<i32>,
    pub in_sync_replica_nodes: Vec<i32>,
}
