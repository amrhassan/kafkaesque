use super::codec::{Read, Write};
use derive_more::{From, Into};

#[derive(From, Into, Debug, PartialEq, Eq, Read, Write)]
pub struct ErrorCode(i16);
