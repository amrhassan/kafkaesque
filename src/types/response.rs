use super::Int32;
use crate::{io::Write, Result};
use tokio::io::AsyncWrite;

pub struct Response<M> {
    message: M,
}

impl<M: ResponseMessage> Write for Response<M> {
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        todo!()
        // Int32::from(self.message.calculate_size())
        //     .write_to(writer)
        //     .await?;
        // self.message.write_to(writer).await?;
        // Ok(())
    }
}

pub trait ResponseMessage: Write {
    fn calculate_size(&self) -> i32;
}
