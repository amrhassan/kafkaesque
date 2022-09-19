use crate::io::{Read, Write};
use derive_more::{Display, From, Into};
use std::{convert::identity, io};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use zigzag::ZigZag;

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Boolean(bool);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int8(i8);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int16(i16);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int32(i32);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int64(i64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Float64(f64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct UInt32(u32);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct VarInt(i32);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct VarLong(i64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Uuid(uuid::Uuid);

macro_rules! primitive_io_impl {
    ($SelfT:ty, $write_map:expr, $write:expr, $read:expr, $read_map:expr,) => {
        #[async_trait::async_trait]
        impl $crate::io::Write for $SelfT {
            async fn write(
                &self,
                mut sink: impl tokio::io::AsyncWrite + Send + Sync + Unpin,
            ) -> tokio::io::Result<()> {
                let written = ($write_map)(self.0);
                $write(&mut sink, written).await?;
                Ok(())
            }
        }

        #[async_trait::async_trait]
        #[allow(clippy::redundant_closure_call)]
        impl $crate::io::Read for $SelfT {
            async fn read(
                mut source: impl tokio::io::AsyncRead + Send + Sync + Unpin,
            ) -> tokio::io::Result<Self> {
                let read = ($read)(&mut source).await?;
                let s = $read_map(read);
                Ok(s.into())
            }
        }
    };
}

primitive_io_impl!(
    Boolean,
    (|b| if b { 1 } else { 0 }),
    AsyncWriteExt::write_u8,
    AsyncReadExt::read_u8,
    (|n| n != 0),
);

primitive_io_impl!(
    VarInt,
    <i32 as ZigZag>::encode,
    AsyncWriteExt::write_u32,
    AsyncReadExt::read_u32,
    <i32 as ZigZag>::decode,
);

primitive_io_impl!(
    VarLong,
    <i64 as ZigZag>::encode,
    AsyncWriteExt::write_u64,
    AsyncReadExt::read_u64,
    <i64 as ZigZag>::decode,
);

primitive_io_impl!(
    UInt32,
    identity,
    AsyncWriteExt::write_u32,
    AsyncReadExt::read_u32,
    identity,
);

primitive_io_impl!(
    Float64,
    identity,
    AsyncWriteExt::write_f64,
    AsyncReadExt::read_f64,
    identity,
);

primitive_io_impl!(
    Int64,
    identity,
    AsyncWriteExt::write_i64,
    AsyncReadExt::read_i64,
    identity,
);

primitive_io_impl!(
    Int32,
    identity,
    AsyncWriteExt::write_i32,
    AsyncReadExt::read_i32,
    identity,
);

primitive_io_impl!(
    Int16,
    identity,
    AsyncWriteExt::write_i16,
    AsyncReadExt::read_i16,
    identity,
);

primitive_io_impl!(
    Int8,
    identity,
    AsyncWriteExt::write_i8,
    AsyncReadExt::read_i8,
    identity,
);

#[async_trait::async_trait]
impl Read for Uuid {
    async fn read(mut source: impl AsyncRead + Send + Sync + Unpin) -> io::Result<Self> {
        let mut buf = [0; 16];
        source.read_exact(&mut buf).await?;
        Ok(uuid::Uuid::from_bytes(buf).into())
    }
}

#[async_trait::async_trait]
impl Write for Uuid {
    async fn write(&self, mut sink: impl AsyncWrite + Send + Sync + Unpin) -> io::Result<()> {
        sink.write_all(self.0.as_bytes()).await?;
        Ok(())
    }
}
