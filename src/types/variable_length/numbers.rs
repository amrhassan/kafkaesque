use crate::Result;
use derive_more::{Display, From, Into};
use integer_encoding::VarIntAsyncReader;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct VarInt(i32);

#[derive(Debug, Clone, Display, From, Into)]
pub struct UnsignedVarInt(pub u64);

#[derive(Debug, Copy, Clone, Display, From, Into)]
pub struct VarLong(i64);

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
