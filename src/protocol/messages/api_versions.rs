use crate::protocol::api_keys::ApiKey;
use crate::protocol::codec::Write;
use crate::protocol::request::{ApiVersion, RequestMessage};
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
