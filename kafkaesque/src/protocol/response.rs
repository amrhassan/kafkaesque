use super::codec::Read;
use crate::Result;
use derive_more::{From, Into};
use tokio::io::AsyncRead;

#[derive(Debug)]
pub struct Response<M> {
    pub err_code: ErrorCode,
    pub message: M,
}

#[derive(From, Into, Debug, PartialEq, Eq, Read)]
pub struct ErrorCode(i16);

impl<R: Read> Read for Response<R> {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        let v = Response {
            err_code: ErrorCode::read_from(reader).await?,
            message: R::read_from(reader).await?,
        };
        Ok(v)
    }
}
