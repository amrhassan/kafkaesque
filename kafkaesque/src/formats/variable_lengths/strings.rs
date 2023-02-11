use crate::formats::{
    codec::{FixedLength, Read, Write},
    Result,
};
use derive_more::{From, Into};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tracing::trace;

use super::VarInt;

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

impl Write for String {
    fn calculate_size(&self) -> i32 {
        self.as_str().calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        self.as_str().write_to(writer).await
    }
}

impl Read for String {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        trace!("reading string len");
        let len = i16::read_from(reader).await?;
        let mut buf = vec![0u8; len as usize];
        trace!("reading a string of len {len}");
        reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8(buf)?)
    }
}

#[derive(Debug, Clone, From, Into, Default)]
pub struct NullableString(String);

impl Write for NullableString {
    fn calculate_size(&self) -> i32 {
        i16::SIZE + self.0.len() as i32
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        if self.0.is_empty() {
            (-1i16).write_to(writer).await
        } else {
            (self.0.len() as i16).write_to(writer).await?;
            writer.write_all(self.0.as_bytes()).await?;
            Ok(())
        }
    }
}

impl Read for NullableString {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        trace!("reading nullable_string len");
        let len = i16::read_from(reader).await?.max(0);
        let mut buf = vec![0u8; len as usize];
        trace!("reading a nullable_string of len {len}");
        reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8(buf)?.into())
    }
}

/// A String whos length is encoded as a VarInt
#[derive(Debug, Clone, From, Into)]
pub struct VarIntString(String);

impl Write for VarIntString {
    fn calculate_size(&self) -> i32 {
        VarInt::from(self.0.len() as i64).calculate_size() + self.0.len() as i32
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        VarInt::from(self.0.len() as i64).write_to(writer).await?;
        writer.write_all(self.0.as_bytes()).await?;
        Ok(())
    }
}

impl Read for VarIntString {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        let len = VarInt::read_from(reader).await?;
        let mut buf = vec![0u8; i64::from(len) as usize];
        reader.read_exact(&mut buf).await?;
        let s = String::from_utf8(buf)?.into();
        Ok(s)
    }
}
