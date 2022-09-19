use crate::Result;
use tokio::io::{AsyncRead, AsyncWrite};

#[async_trait::async_trait]
pub trait Write {
    async fn write_to(&self, mut sink: impl AsyncWrite + Send + Sync + Unpin) -> Result<()>;
}

#[async_trait::async_trait]
pub trait Read: Sized {
    async fn read_from(mut source: impl AsyncRead + Send + Sync + Unpin) -> Result<Self>;
}
