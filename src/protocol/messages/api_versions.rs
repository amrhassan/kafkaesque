use crate::protocol::api_keys::ApiKey;
use crate::protocol::codec::{Read, Write};
use crate::protocol::request::{ApiVersion, RequestMessage};
use crate::protocol::response::ErrorCode;
use crate::Result;
use derive_more::{Constructor, From, Into};
use tokio::io::AsyncWrite;

#[derive(Debug)]
pub struct ApiVersionsRequest;

impl RequestMessage for ApiVersionsRequest {
    const API_KEY: ApiKey = ApiKey::ApiVersions;
    const API_VERSION: ApiVersion = ApiVersion(0);
}

impl Write for ApiVersionsRequest {
    fn calculate_size(&self) -> i32 {
        0
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ApiVersionsResponse {
    pub api_keys: Vec<ApiKeyVersioned>,
}

#[derive(Debug)]
pub struct ApiKeyVersioned {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,
}

impl Read for ApiKeyVersioned {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        let v = ApiKeyVersioned {
            api_key: ApiKey::read_from(reader).await?,
            min_version: i16::read_from(reader).await?,
            max_version: i16::read_from(reader).await?,
        };
        Ok(v)
    }
}

impl Read for ApiVersionsResponse {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        let v = ApiVersionsResponse {
            api_keys: Vec::<ApiKeyVersioned>::read_from(reader).await?,
        };
        Ok(v)
    }
}
