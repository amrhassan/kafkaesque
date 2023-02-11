use crate::formats::api_keys::ApiKey;
use crate::formats::codec::{Read, Write};
use crate::formats::request::{ApiVersion, RequestMessage};
use crate::formats::{ErrorCode, NullableString};

#[derive(Debug, Write, RequestMessage)]
#[request_message(version = 0, key = "CreateTopics")]
pub struct CreateTopicsReqV0 {
    pub topics: Vec<CreateTopicsReqV0CreateTopic>,
    pub timeout_ms: i32,
}

#[derive(Debug, Write, Read)]
pub struct CreateTopicsReqV0CreateTopic {
    pub name: String,
    pub num_partitions: i32,
    pub replication_factor: i16,
    pub assignments: Vec<CreateTopicV0Assignments>,
    pub configs: Vec<CreateTopicsV0Config>,
}

#[derive(Debug, Write, Read)]
pub struct CreateTopicV0Assignments {
    pub partition_index: i32,
    pub broker_ids: Vec<i32>,
}

#[derive(Debug, Write, Read)]
pub struct CreateTopicsV0Config {
    pub name: String,
    pub value: NullableString,
}

#[derive(Debug, Write, Read)]
pub struct CreateTopicsRespV0 {
    pub topics: Vec<CreateTopicsRespV0Topic>,
}

#[derive(Debug, Write, Read)]
pub struct CreateTopicsRespV0Topic {
    pub name: String,
    pub err_code: ErrorCode,
}
