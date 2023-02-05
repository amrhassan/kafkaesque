use crate::{
    protocol::codec::{FixedLength, Write},
    Result,
};
use derive_more::{Display, From, Into};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

impl<'a> Write for &'a str {
    fn calculate_size(&self) -> i32 {
        i16::SIZE + self.len() as i32
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        let len = self.len() as i16;
        len.write_to(writer).await?;
        writer.write_all(self.as_bytes()).await?;
        Ok(())
    }
}
