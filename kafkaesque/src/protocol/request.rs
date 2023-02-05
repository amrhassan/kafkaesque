use super::api_keys::ApiKey;
use super::codec::{FixedLength, Read, Write};
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

impl Read for CorrelationId {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        Ok(i32::read_from(reader).await?.into())
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

#[derive(Debug, Write)]
pub struct RequestHeader {
    pub api_key: ApiKey,
    pub api_version: ApiVersion,
    pub cid: CorrelationId,
    pub client_id: &'static str,
}

pub trait RequestMessage {
    const API_KEY: ApiKey;
    const API_VERSION: ApiVersion;
}
