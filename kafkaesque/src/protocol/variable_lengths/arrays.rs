use crate::protocol::{codec::Read, Result};
use tokio::io::AsyncRead;
use tracing::trace;

// Reader for ARRAY type
impl<A: Read> Read for Vec<A> {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        let length = i32::read_from(reader).await?;
        trace!("reading array len = {length}");
        let mut output = Vec::with_capacity(length as usize);
        for _ in 0..length {
            output.push(A::read_from(reader).await?);
        }
        Ok(output)
    }
}
