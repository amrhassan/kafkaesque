use crate::formats::{Read, Result, Write};
use derive_more::{Display, From, Into};
use integer_encoding::{VarIntAsyncReader, VarIntAsyncWriter};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct VarInt(pub i64);

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value.into())
    }
}

impl Read for VarInt {
    async fn read_from(mut reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        let n = reader.read_varint_async().await?;
        Ok(VarInt(n))
    }
}

impl Write for VarInt {
    fn calculate_size(&self) -> i32 {
        integer_encoding::VarInt::required_space(self.0) as i32
    }
    async fn write_to(&self, mut writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        writer.write_varint_async(self.0).await?;
        Ok(())
    }
}
