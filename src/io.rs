use tokio::io::{self, AsyncRead, AsyncWrite};

#[async_trait::async_trait]
pub trait Write {
    async fn write(&self, mut sink: impl AsyncWrite + Send + Sync + Unpin) -> io::Result<()>;
}

#[async_trait::async_trait]
pub trait Read: Sized {
    async fn read(mut source: impl AsyncRead + Send + Sync + Unpin) -> io::Result<Self>;
}
