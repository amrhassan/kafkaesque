use crate::{
    io::{Read, Write},
    types::Int16,
    Result,
};
use derive_more::{Display, From, Into};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Clone, Display, From, Into)]
pub struct String(pub std::string::String);

#[async_trait::async_trait]
impl Read for String {
    async fn read_from(mut source: impl AsyncRead + Send + Unpin) -> Result<Self> {
        let len = Int16::read_from(&mut source).await?;
        let mut buf = vec![0; len.0 as usize];
        source.read_exact(&mut buf).await?;
        let s = String(std::string::String::from_utf8(buf)?);
        Ok(s)
    }
}

#[async_trait::async_trait]
impl Write for String {
    async fn write_to(&self, mut sink: impl AsyncWrite + Send + Unpin) -> Result<()> {
        let len = Int16::from(self.0.len() as i16);
        len.write_to(&mut sink).await?;
        sink.write_all(self.0.as_bytes()).await?;
        Ok(())
    }
}
