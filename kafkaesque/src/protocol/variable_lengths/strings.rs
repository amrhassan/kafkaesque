use crate::protocol::{
    codec::{FixedLength, Read, Write},
    Result,
};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};

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

impl Read for String {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        let len = i16::read_from(reader).await?;
        let mut buf = vec![0u8; len as usize];
        reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8(buf)?)
    }
}
