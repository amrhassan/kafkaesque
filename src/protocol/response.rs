use super::{
    api_keys::ApiKey,
    codec::{FixedLength, Read, Write},
};
use crate::Result;
use derive_more::{From, Into};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug)]
pub struct Response<M> {
    message: M,
}

impl<M: Write + ResponseMessage> Write for Response<M> {
    fn calculate_size(&self) -> i32 {
        i32::SIZE + self.message.calculate_size()
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        (self.message.calculate_size() as i32)
            .write_to(writer)
            .await?;
        self.message.write_to(writer).await?;
        Ok(())
    }
}

pub trait ResponseMessage {
    const API_KEY: ApiKey;
    const API_VERSION: u16;
}

#[derive(From, Into, Debug)]
pub struct ErrorCode(i16);

impl Read for ErrorCode {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        Ok(i16::read_from(reader).await?.into())
    }
}
