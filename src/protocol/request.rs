use super::api_keys::ApiKey;
use super::io::{FixedLength, Write};
use crate::Result;
use derive_more::{Constructor, From, Into};
use tokio::io::AsyncWrite;

#[derive(Debug, From, Into, Clone, Copy)]
pub struct CorrelationId(i32);

impl Write for CorrelationId {
    fn calculate_size(&self) -> i32 {
        i32::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        self.0.write_to(writer).await
    }
}

#[derive(From, Into, Copy, Clone, Debug)]
pub struct ApiVersion(pub i16);

impl Write for ApiVersion {
    fn calculate_size(&self) -> i32 {
        i16::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        self.0.write_to(writer).await
    }
}

#[derive(Debug)]
pub struct RequestHeader {
    pub api_key: ApiKey,
    pub api_version: ApiVersion,
    pub cid: CorrelationId,
    pub client_id: &'static str,
}

impl Write for RequestHeader {
    fn calculate_size(&self) -> i32 {
        self.api_key.calculate_size()
            + self.api_version.calculate_size()
            + self.cid.calculate_size()
            + self.client_id.calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        self.api_key.write_to(writer).await?;
        self.api_version.write_to(writer).await?;
        self.cid.write_to(writer).await?;
        self.client_id.write_to(writer).await?;
        Ok(())
    }
}

pub trait RequestMessage {
    const API_KEY: ApiKey;
    const API_VERSION: ApiVersion;
}
