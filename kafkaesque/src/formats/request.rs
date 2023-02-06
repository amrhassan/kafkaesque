use super::api_keys::ApiKey;
use super::codec::{Read, Write};
use derive_more::{From, Into};

pub use kafkaesque_macros::RequestMessage;

#[derive(Debug, From, Into, Clone, Copy, Read, Write)]
pub struct CorrelationId(i32);

#[derive(From, Into, Copy, Clone, Debug, Write)]
pub struct ApiVersion(pub i16);

#[derive(Debug, Write)]
pub struct RequestHeader {
    pub api_key: ApiKey,
    pub api_version: ApiVersion,
    pub cid: CorrelationId,
    pub client_id: String,
}

pub trait RequestMessage {
    const API_KEY: ApiKey;
    const API_VERSION: ApiVersion;
}
