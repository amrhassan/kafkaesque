use crate::Result;
use derive_more::{Display, From, Into};
use integer_encoding::VarIntAsyncReader;
use std::convert::identity;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Boolean(pub bool);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int8(pub i8);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int16(pub i16);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int32(pub i32);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Int64(pub i64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Float64(pub f64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct UInt32(pub u32);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct VarInt(i32);

#[derive(Debug, Clone, Display, From, Into)]
pub struct UnsignedVarInt(pub u64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct VarLong(i64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct Uuid(uuid::Uuid);

macro_rules! primitive_io_impl {
    ($SelfT:ty, $write_map:expr, $write:expr, $read:expr, $read_map:expr,) => {
        impl $crate::io::Write for $SelfT {
            async fn write_to(
                &self,
                sink: &mut (dyn tokio::io::AsyncWrite + Send + Unpin),
            ) -> Result<()> {
                let to_be_written = ($write_map)(self.0);
                $write(sink, to_be_written).await?;
                Ok(())
            }
        }

        #[allow(clippy::redundant_closure_call)]
        impl $crate::io::Read for $SelfT {
            async fn read_from(
                source: &mut (dyn tokio::io::AsyncRead + Send + Unpin),
            ) -> Result<Self> {
                let read = ($read)(source).await?;
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
    identity,
    write_varint_bytes,
    read_varint_bytes::<i32>,
    identity,
);

primitive_io_impl!(
    VarLong,
    identity,
    write_varint_bytes,
    read_varint_bytes::<i64>,
    identity,
);

primitive_io_impl!(
    UnsignedVarInt,
    identity,
    write_varint_bytes,
    read_varint_bytes::<u64>,
    identity,
);

primitive_io_impl!(
    Uuid,
    uuid::Uuid::into_bytes,
    write_uuid_bytes,
    read_uuid_bytes,
    uuid::Uuid::from_bytes,
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

async fn read_uuid_bytes(mut reader: impl AsyncRead + Send + Unpin) -> Result<[u8; 16]> {
    let mut buf = [0; 16];
    reader.read_exact(&mut buf).await?;
    Ok(buf)
}

async fn write_uuid_bytes(mut writer: impl AsyncWrite + Send + Unpin, bs: [u8; 16]) -> Result<()> {
    writer.write_all(&bs).await?;
    Ok(())
}

async fn write_varint_bytes(
    mut writer: impl AsyncWrite + Send + Unpin,
    n: impl integer_encoding::VarInt,
) -> Result<usize> {
    let mut buf = [0_u8; 10];
    let b = n.encode_var(&mut buf);
    writer.write_all(&buf[0..b]).await?;
    Ok(b)
}

async fn read_varint_bytes<VI: integer_encoding::VarInt>(
    mut reader: impl AsyncRead + Send + Unpin,
) -> Result<VI> {
    let n = reader.read_varint_async().await?;
    Ok(n)
}
