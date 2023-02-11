use crate::formats::api_keys::ApiKey;
use crate::formats::codec::{Read, Write};
use crate::formats::request::{ApiVersion, RequestMessage};
use crate::formats::ErrorCode;

#[derive(Debug, Write, RequestMessage)]
#[request_message(version = 0, key = "DeleteTopics")]
pub struct DeleteTopicsReqV0 {
    pub topic_names: Vec<String>,
    pub timeout_ms: i32,
}

#[derive(Debug, Write, Read)]
pub struct DeleteTopicsRespV0 {
    pub topics: Vec<DeleteTopicsRespV0Topic>,
}

#[derive(Debug, Write, Read)]
pub struct DeleteTopicsRespV0Topic {
    pub name: String,
    pub err_code: ErrorCode,
}
