use crate::formats::codec::{Read, Write};
use crate::formats::variable_lengths::SizedValue;
use crate::formats::VarIntArray;
use bytes::Bytes;

use super::RecordBatchAttributes;

#[derive(Debug, Clone, Write, Read)]
pub struct RecordHeader {
    pub key: SizedValue<String>,
    pub value: VarIntArray<u8>,
}

impl RecordHeader {
    pub fn create(key: impl Into<String>, value: Bytes) -> Self {
        RecordHeader {
            key: SizedValue(key.into()),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordInput {
    pub headers: Vec<RecordHeader>,
    pub key: Bytes,
    pub value: Bytes,
}

#[derive(Debug, Clone)]
pub struct RecordBatchInput2 {
    pub attributes: RecordBatchAttributes,
    pub records: Vec<RecordInput>,
}
