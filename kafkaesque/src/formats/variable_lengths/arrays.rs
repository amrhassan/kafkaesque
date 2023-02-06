use crate::formats::{
    codec::{FixedLength, Read, Write},
    Result,
};
use tokio::io::AsyncRead;
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

impl<A: Write> Write for Vec<A> {
    fn calculate_size(&self) -> i32 {
        i32::SIZE + self.iter().map(|a| a.calculate_size()).sum::<i32>()
    }
    async fn write_to(
        &self,
        writer: &mut (dyn tokio::io::AsyncWrite + Send + Unpin),
    ) -> Result<()> {
        let len: i32 = self.len() as i32;
        len.write_to(writer).await?;
        for a in self {
            a.write_to(writer).await?;
        }
        Ok(())
    }
}
