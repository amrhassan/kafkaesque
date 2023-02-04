use super::{ApiKey, ResponseMessage};
use crate::{
    io::{FixedLength, Write},
    types::{Int16, Int32},
    Result,
};
use tokio::io::AsyncWrite;

#[derive(Debug)]
pub struct Response<M> {
    message: M,
}

impl<M: Write + ResponseMessage> Write for Response<M> {
    fn calculate_size(&self) -> i32 {
        Int32::SIZE + self.message.calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int32::from(self.message.calculate_size())
            .write_to(writer)
            .await?;
        self.message.write_to(writer).await?;
        Ok(())
    }
}
