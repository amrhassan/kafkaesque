use crate::formats::api_keys::ApiKey;
use crate::formats::codec::{Read, Write};
use crate::formats::request::{ApiVersion, RequestMessage};
use crate::formats::ErrorCode;

#[derive(Debug, Write, RequestMessage)]
#[request_message(version = 0, key = "ApiVersions")]
pub struct ApiVersionsReq;

#[derive(Debug, Read)]
pub struct ApiVersionsResp {
    pub error_code: ErrorCode,
    pub api_keys: Vec<ApiKeyVersionsReqV0Version>,
}

#[derive(Debug, Read)]
pub struct ApiKeyVersionsReqV0Version {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::{response::ErrorCode, BrokerConnection};

    #[tokio::test]
    async fn test_api_versions() {
        let mut conn = BrokerConnection::connect("test-client", "localhost:9092")
            .await
            .unwrap();
        let resp: ApiVersionsResp = conn.send(ApiVersionsReq).await.unwrap();
        assert_eq!(resp.error_code, ErrorCode::from(0));
        assert!(!resp.api_keys.is_empty())
    }
}
