use super::codec::{Read, Write};
use crate::Result;
use derive_more::{From, Into};

#[derive(Debug, Read, Write)]
pub struct Response<M> {
    pub err_code: ErrorCode,
    pub message: M,
}

#[derive(From, Into, Debug, PartialEq, Eq, Read, Write)]
pub struct ErrorCode(i16);
