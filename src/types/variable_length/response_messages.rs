use super::ApiKey;
use crate::{
    io::{Read, Write},
    types::{Int16, Int32},
    Result,
};
use derive_more::{From, Into};
use tokio::io::{AsyncRead, AsyncWrite};

pub trait ResponseMessage {
    const API_KEY: ApiKey;
    const API_VERSION: u16;
}

#[derive(From, Into, Debug)]
pub struct ErrorCode(Int16);

impl Read for ErrorCode {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        Ok(Int16::read_from(reader).await?.into())
    }
}
