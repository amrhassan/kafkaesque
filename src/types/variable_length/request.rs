use crate::{
    io::{FixedLength, VariableLength, Write},
    types::{Int16, Int32},
    Result,
};
use derive_more::{Constructor, From, Into};
use tokio::io::AsyncWrite;

#[derive(Debug, From, Into, Clone, Copy)]
pub struct CorrelationId(i32);

impl Write for CorrelationId {
    fn calculate_size(&self) -> i32 {
        Int32::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        Int32::from(i32::from(*self)).write_to(writer).await
    }
}

#[derive(Debug, Constructor)]
pub struct Request<M> {
    message: M,
}

impl<M: Write> Write for Request<M> {
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
