use super::Int32;
use crate::{io::Write, Result};
use tokio::io::AsyncWrite;

pub struct Request<M> {
    message: M,
}

impl<M: RequestMessage> Write for Request<M> {
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int32::from(self.message.calculate_size())
            .write_to(writer)
            .await?;
        self.message.write_to(writer).await?;
        Ok(())
    }
}

pub trait RequestMessage: Write {
    fn calculate_size(&self) -> i32;
}
