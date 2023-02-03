use crate::{
    io::{VariableLength, Write},
    types::Int32,
    Result,
};
use tokio::io::AsyncWrite;

pub struct Response<M> {
    message: M,
}

impl<M: Write + VariableLength> Write for Response<M> {
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int32::from(self.message.calculate_size())
            .write_to(writer)
            .await?;
        self.message.write_to(writer).await?;
        Ok(())
    }
}
