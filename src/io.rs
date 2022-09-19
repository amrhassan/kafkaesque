use crate::Result;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait::async_trait]
pub trait Write {
    async fn write_to(&self, mut writer: impl AsyncWrite + Send + Unpin) -> Result<()>;
}

#[async_trait::async_trait]
pub trait Read: Sized {
    async fn read_from(mut reader: impl AsyncRead + Send + Unpin) -> Result<Self>;
}
