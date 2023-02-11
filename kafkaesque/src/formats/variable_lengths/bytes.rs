use super::VarInt;
use crate::formats::{FixedLength, Read, Result, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub struct NullableBytes<M>(pub M);

impl<M: Write> Write for NullableBytes<M> {
    fn calculate_size(&self) -> i32 {
        i32::SIZE + self.0.calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        let content_len = self.0.calculate_size();
        if content_len == 0 {
            (-1_i32).write_to(writer).await?;
        } else {
            content_len.write_to(writer).await?;
            self.0.write_to(writer).await?;
        }
        Ok(())
    }
}

impl<M: Read + Default> Read for NullableBytes<M> {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        let content_len = i32::read_from(reader).await?;
        if content_len == -1 {
            Ok(NullableBytes(M::default()))
        } else {
            Ok(NullableBytes(M::read_from(reader).await?))
        }
    }
}

/// Value that is prefixed by a VarInt size
#[derive(Debug, Clone)]
pub struct SizedValue<M>(pub M);

impl<M: Write> Write for SizedValue<M> {
    fn calculate_size(&self) -> i32 {
        let value_size = self.0.calculate_size();
        VarInt::from(value_size).calculate_size() + value_size
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        let value_size = VarInt::from(self.0.calculate_size());
        value_size.write_to(writer).await?;
        self.0.write_to(writer).await?;
        Ok(())
    }
}

impl<M: Read> Read for SizedValue<M> {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        let _value_len = VarInt::read_from(reader).await?;
        let m = M::read_from(reader).await?;
        Ok(SizedValue(m))
    }
}
