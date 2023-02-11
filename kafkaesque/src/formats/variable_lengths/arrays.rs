use super::VarInt;
use crate::formats::{
    codec::{FixedLength, Read, Write},
    FormatError, Result,
};
use bytes::Bytes;
use derive_more::{From, Into};
use non_empty_vec::NonEmpty;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::trace;

// Reader for ARRAY type
impl<A: Read> Read for Vec<A> {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        trace!("reading array len");
        let length = i32::read_from(reader).await?;
        trace!("reading array of len {length}");
        let mut output = Vec::with_capacity(length as usize);
        for _ in 0..length {
            output.push(A::read_from(reader).await?);
        }
        Ok(output)
    }
}

impl<A: Read> Read for NonEmpty<A> {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        let mut vs = Vec::read_from(reader).await?;
        // TODO: optimize. reading into a vec is unnecessary.
        if vs.is_empty() {
            Err(FormatError::UnexpectedEmptySeq)
        } else {
            let head = vs.remove(0);
            Ok(NonEmpty::from((head, vs)))
        }
    }
}

impl<A: Write> Write for &[A] {
    fn calculate_size(&self) -> i32 {
        i32::SIZE + self.iter().map(|a| a.calculate_size()).sum::<i32>()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        let len: i32 = self.len() as i32;
        len.write_to(writer).await?;
        for a in self.iter() {
            a.write_to(writer).await?;
        }
        Ok(())
    }
}

impl<A: Write> Write for Vec<A> {
    fn calculate_size(&self) -> i32 {
        self.as_slice().calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        self.as_slice().write_to(writer).await
    }
}

impl<A: Write> Write for NonEmpty<A> {
    fn calculate_size(&self) -> i32 {
        self.as_slice().calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        self.as_slice().write_to(writer).await
    }
}

impl Write for Bytes {
    fn calculate_size(&self) -> i32 {
        (&self[..]).calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        (&self[..]).write_to(writer).await
    }
}

impl Read for Bytes {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        Ok(Vec::read_from(reader).await?.into())
    }
}

/// An array whose length is encoded as VarInt
#[derive(Debug, Clone, From, Into, Default)]
pub struct VarIntArray<A>(Vec<A>);

impl<A: Write> Write for VarIntArray<A> {
    fn calculate_size(&self) -> i32 {
        VarInt::from(self.0.len() as i64).calculate_size()
            + self.0.iter().map(|a| a.calculate_size()).sum::<i32>()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        VarInt::from(self.0.len() as i64).write_to(writer).await?;
        for a in self.0.iter() {
            a.write_to(writer).await?;
        }
        Ok(())
    }
}

impl<A: Read> Read for VarIntArray<A> {
    async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
        let len = VarInt::read_from(reader).await?.0 as usize;
        let mut output = Vec::with_capacity(len);
        for _ in 0..len {
            output.push(A::read_from(reader).await?);
        }
        Ok(output.into())
    }
}

impl From<Bytes> for VarIntArray<u8> {
    fn from(value: Bytes) -> Self {
        VarIntArray(value.to_vec())
    }
}
