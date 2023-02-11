use super::RecordHeader;
use crate::formats::api_keys::ApiKey;
use crate::formats::codec::{Read, Write};
use crate::formats::crc32::Crc32;
use crate::formats::request::{ApiVersion, RequestMessage};
use crate::formats::variable_lengths::{NullableBytes, SizedValue};
use crate::formats::{ErrorCode, NullableString, VarInt, VarIntArray};
use crate::formats::{FixedLength, Result};
use futures::executor::block_on;
use non_empty_vec::NonEmpty;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Write, RequestMessage)]
#[request_message(version = 3, key = "Produce")]
pub struct ProduceReqV3 {
    pub transactional_id: NullableString,
    pub acks: i16,
    pub timeout_ms: i32,
    pub topic_data: Vec<ProduceReqV3TopicData>,
}

#[derive(Debug, Write)]
pub struct ProduceReqV3TopicData {
    pub topic_name: String,
    pub partition_data: Vec<ProduceReqV3PartitionData>,
}

#[derive(Debug, Write)]
pub struct ProduceReqV3PartitionData {
    pub partition_id: i32,
    pub records: NullableBytes<RecordBatch>,
}

#[derive(Debug, Write, Read)]
pub struct ProduceRespV3 {
    pub responses: Vec<ProduceRespV3Response>,
}

#[derive(Debug, Write, Read)]
pub struct ProduceRespV3Response {
    pub name: String,
    pub partition_responses: Vec<ProduceV3RespPartitionResponse>,
    pub throttle_time_ms: i32,
}

#[derive(Debug, Write, Read)]
pub struct ProduceV3RespPartitionResponse {
    pub partition_id: i32,
    pub err_code: ErrorCode,
    pub base_offset: i64,
    pub log_append_time_ms: i64,
}

#[derive(Debug, Clone)]
pub struct RecordBatchInput {
    pub base_offset: i64,
    pub partition_leader_epoch: i32,
    pub attributes: RecordBatchAttributes,
    pub last_offset_delta: i32,
    pub base_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: NonEmpty<Record>,
}

impl RecordBatchInput {
    pub fn into_record_batch(self) -> RecordBatch {
        let crc = {
            let mut crc = Crc32::default();
            let crc_input = RecordBatchCrcInput {
                attributes: self.attributes,
                last_offset_delta: self.last_offset_delta,
                base_timestamp: self.base_timestamp,
                max_timestamp: self.max_timestamp,
                producer_id: self.producer_id,
                producer_epoch: self.producer_epoch,
                base_sequence: self.base_sequence,
                records: &self.records,
            };
            block_on(crc_input.write_to(&mut crc)).expect("Failed to write to Crc32");
            crc.finalize() as i32
        };
        let mut rb = RecordBatch {
            base_offset: self.base_offset,
            batch_length: 0, // To be set afterwards
            partition_leader_epoch: -1,
            magic: 2,
            crc,
            attributes: self.attributes,
            last_offset_delta: self.last_offset_delta,
            base_timestamp: self.base_timestamp,
            max_timestamp: self.max_timestamp,
            producer_id: self.producer_id,
            producer_epoch: self.producer_epoch,
            base_sequence: self.base_sequence,
            records: self.records,
        };
        rb.batch_length = rb.calculate_size();
        rb
    }
}

#[derive(Write, Debug)]
struct RecordBatchCrcInput<'a> {
    attributes: RecordBatchAttributes,
    last_offset_delta: i32,
    base_timestamp: i64,
    max_timestamp: i64,
    producer_id: i64,
    producer_epoch: i16,
    base_sequence: i32,
    records: &'a [Record],
}

#[derive(Debug, Clone, Write, Read)]
pub struct RecordBatch {
    pub base_offset: i64,
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    pub magic: i8,
    pub crc: i32,
    pub attributes: RecordBatchAttributes,
    pub last_offset_delta: i32,
    pub base_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: NonEmpty<Record>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecordBatchCompression {
    #[default]
    None,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RecordBatchAttributes {
    pub compression: RecordBatchCompression,
    pub is_transactional: bool,
    pub is_control_batch: bool,
    pub has_delete_horizons: bool,
}

impl RecordBatchAttributes {
    const COMPRESSION_GZIP: i16 = 0b0000000000000001;
    const COMPRESSION_SNAPPY: i16 = 0b0000000000000010;
    const COMPRESSION_LZ4: i16 = 0b0000000000000011;
    const COMPRESSION_ZSTD: i16 = 0b0000000000000100;
    const IS_TRANSACTIONAL: i16 = 0b0000000000010000;
    const IS_CONTROL_BATCH: i16 = 0b0000000000100000;
    const HAS_DELETE_HORIZONS: i16 = 0b0000000001000000;
}

impl Write for RecordBatchAttributes {
    fn calculate_size(&self) -> i32 {
        i16::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        let compression = match self.compression {
            RecordBatchCompression::None => 0,
            RecordBatchCompression::Gzip => Self::COMPRESSION_GZIP,
            RecordBatchCompression::Snappy => Self::COMPRESSION_SNAPPY,
            RecordBatchCompression::Lz4 => Self::COMPRESSION_LZ4,
            RecordBatchCompression::Zstd => Self::COMPRESSION_ZSTD,
        };
        let is_transactional = if self.is_transactional {
            Self::IS_TRANSACTIONAL
        } else {
            0
        };
        let is_control = if self.is_control_batch {
            Self::IS_CONTROL_BATCH
        } else {
            0
        };
        let has_delete_horizons = if self.has_delete_horizons {
            Self::HAS_DELETE_HORIZONS
        } else {
            0
        };
        let n: i16 = compression & is_transactional & is_control & has_delete_horizons;
        n.write_to(writer).await
    }
}

impl Read for RecordBatchAttributes {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        let n = i16::read_from(reader).await?;
        let compression = if n & Self::COMPRESSION_GZIP != 0 {
            RecordBatchCompression::Gzip
        } else if n & Self::COMPRESSION_SNAPPY != 0 {
            RecordBatchCompression::Snappy
        } else if n & Self::COMPRESSION_LZ4 != 0 {
            RecordBatchCompression::Lz4
        } else if n & Self::COMPRESSION_ZSTD != 0 {
            RecordBatchCompression::Zstd
        } else {
            RecordBatchCompression::None
        };
        let is_transactional = n & Self::IS_TRANSACTIONAL != 0;
        let is_control_batch = n & Self::IS_CONTROL_BATCH != 0;
        let has_delete_horizons = n & Self::HAS_DELETE_HORIZONS != 0;
        Ok(RecordBatchAttributes {
            compression,
            is_transactional,
            is_control_batch,
            has_delete_horizons,
        })
    }
}

#[derive(Debug, Clone, Write, Read)]
pub struct Record {
    pub content: SizedValue<RecordContent>,
}

#[derive(Debug, Clone, Read, Write)]
pub struct RecordContent {
    pub attributes: i8,
    pub timestamp_delta: VarInt,
    pub offset_delta: VarInt,
    pub key: VarIntArray<u8>,
    pub value: VarIntArray<u8>,
    pub headers: VarIntArray<RecordHeader>,
}

impl RecordContent {
    pub fn into_record(self) -> Record {
        Record {
            content: SizedValue(self),
        }
    }
}

#[derive(Debug, Clone, Write, Read)]
pub struct ControlRecord {
    pub version: i16,
    pub ty: i16,
}
