use crate::{
    io::{FixedLength, Write},
    types::{Int16, Int32},
    Result,
};
use derive_more::{Constructor, From, Into};
use tokio::io::AsyncWrite;

#[derive(Debug, From, Into, Clone, Copy)]
pub struct CorrelationId(i32);

impl Write for CorrelationId {
    fn calculate_size(&self) -> i32 {
        Int32::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int32::from(i32::from(*self)).write_to(writer).await
    }
}

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

#[derive(From, Into, Copy, Clone, Debug)]
pub struct ApiVersion(pub i16);

impl Write for ApiVersion {
    fn calculate_size(&self) -> i32 {
        Int16::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int16::from(i16::from(*self)).write_to(writer).await
    }
}

#[derive(Debug)]
pub struct RequestHeader {
    pub api_key: ApiKey,
    pub api_version: ApiVersion,
    pub cid: CorrelationId,
}

impl Write for RequestHeader {
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

pub trait RequestMessage {
    const API_KEY: ApiKey;
    const API_VERSION: ApiVersion;
}

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
