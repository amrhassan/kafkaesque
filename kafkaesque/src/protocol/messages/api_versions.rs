use crate::protocol::api_keys::ApiKey;
use crate::protocol::codec::{Read, Write};
use crate::protocol::request::{ApiVersion, RequestMessage};

#[derive(Debug, Write, RequestMessage)]
#[request_message(version = 0, key = "ApiVersions")]
pub struct ApiVersionsRequest;

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
    use crate::protocol::{response::ErrorCode, BrokerConnection, Response};

    #[tokio::test]
    async fn test_api_versions() {
        let mut conn = BrokerConnection::connect("test-client", "localhost:9092")
            .await
            .unwrap();
        let resp: Response<ApiVersionsResponse> = conn.send(ApiVersionsRequest).await.unwrap();
        assert_eq!(resp.err_code, ErrorCode::from(0));
        assert!(!resp.message.api_keys.is_empty())
    }
}
