use super::CorrelationId;
use crate::{
    io::{FixedLength, VariableLength, Write},
    types::{Int16, Int32},
    Result,
};
use derive_more::{From, Into};
use tokio::io::AsyncWrite;

#[derive(Debug, Clone, Copy)]
pub enum ApiKey {
    ApiVersions = 18,
}

impl Write for ApiKey {
    fn calculate_size(&self) -> i32 {
        Int16::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int16::from(*self as i16).write_to(writer).await
    }
}

#[derive(From, Into, Copy, Clone)]
pub struct ApiVersion(i16);

impl Write for ApiVersion {
    fn calculate_size(&self) -> i32 {
        Int16::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int16::from(i16::from(*self)).write_to(writer).await
    }
}

pub struct ApiVersionsRequest {
    pub api_key: ApiKey,
    pub api_version: ApiVersion,
    pub cid: CorrelationId,
}

impl ApiVersionsRequest {
    pub fn new(cid: CorrelationId) -> Self {
        ApiVersionsRequest {
            api_key: ApiKey::ApiVersions,
            api_version: 0.into(),
            cid,
        }
    }
}

impl Write for ApiVersionsRequest {
    fn calculate_size(&self) -> i32 {
        self.api_key.calculate_size()
            + self.api_version.calculate_size()
            + self.cid.calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        self.api_key.write_to(writer).await?;
        self.api_version.write_to(writer).await?;
        self.cid.write_to(writer).await?;
        Ok(())
    }
}
