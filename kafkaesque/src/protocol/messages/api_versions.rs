use crate::protocol::api_keys::ApiKey;
use crate::protocol::codec::{Read, Write};
use crate::protocol::request::{ApiVersion, RequestMessage};
use crate::protocol::response::ErrorCode;
use crate::Result;
use derive_more::{Constructor, From, Into};
use tokio::io::AsyncWrite;

#[derive(Debug, Write)]
pub struct ApiVersionsRequest;

impl RequestMessage for ApiVersionsRequest {
    const API_KEY: ApiKey = ApiKey::ApiVersions;
    const API_VERSION: ApiVersion = ApiVersion(0);
}

#[derive(Debug, Read)]
pub struct ApiVersionsResponse {
    pub api_keys: Vec<ApiKeyVersioned>,
}

#[derive(Debug, Read)]
pub struct ApiKeyVersioned {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::os::unix::process::CommandExt;

    #[tokio::test]
    async fn test_api_versions() {
        let mut client = Client::connect("test-client", "localhost:9092")
            .await
            .unwrap();
        let resp: Response<ApiVersionsResponse> = client.send(ApiVersionsRequest).await.unwrap();
        assert_eq!(resp.err_code, ErrorCode::from(0));
        assert!(!resp.message.api_keys.is_empty())
    }
}
