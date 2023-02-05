use crate::{
    protocol::codec::{FixedLength, Read, Write},
    Result,
};
use derive_more::{Display, From, Into};
use integer_encoding::VarIntAsyncReader;
use std::convert::identity;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;

impl FixedLength for bool {
    const SIZE: i32 = 1;
}

impl Read for bool {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self> {
        Ok(i8::read_from(reader).await? == 0)
    }
}

impl Write for bool {
    fn calculate_size(&self) -> i32 {
        bool::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        let n: i8 = if *self { 1 } else { 0 };
        n.write_to(writer).await
    }
}

// use crate::Result;
// use derive_more::{Display, From, Into};
// use integer_encoding::VarIntAsyncReader;
// use std::convert::identity;
// use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

// #[derive(Debug, Copy, Clone, Display, From, Into)]
// pub struct VarInt(i32);

// #[derive(Debug, Copy, Clone, Display, From, Into)]
// pub struct VarLong(i64);

// // #[derive(Debug, Clone, Display, From, Into)]
// // pub struct UnsignedVarInt(pub u64);

// macro_rules! var_number_io_impl {
//     ($SelfT:ty, $size:expr, $write_map:expr, $write:expr, $read:expr, $read_map:expr,) => {
//         impl $crate::io::Write for $SelfT {
//             fn calculate_size(&self) -> i32 {
//                 $size(self)
//             }
//             async fn write_to(
//                 &self,
//                 sink: &mut (dyn tokio::io::AsyncWrite + Send + Unpin),
//             ) -> Result<()> {
//                 let to_be_written = ($write_map)(self.0);
//                 $write(sink, to_be_written).await?;
//                 Ok(())
//             }
//         }
//         #[allow(clippy::redundant_closure_call)]
//         impl $crate::io::Read for $SelfT {
//             async fn read_from(
//                 source: &mut (dyn tokio::io::AsyncRead + Send + Unpin),
//             ) -> Result<Self> {
//                 let read = ($read)(source).await?;
//                 let s = $read_map(read);
//                 Ok(s.into())
//             }
//         }
//     };
// }

// var_number_io_impl!(
//     VarInt,
//     |n: &VarInt| integer_encoding::VarInt::required_space(i32::from(*n)) as i32,
//     identity,
//     write_varint_bytes,
//     read_varint_bytes::<i32>,
//     identity,
// );

// var_number_io_impl!(
//     VarLong,
//     |n: &VarLong| integer_encoding::VarInt::required_space(i64::from(*n)) as i32,
//     identity,
//     write_varint_bytes,
//     read_varint_bytes::<i64>,
//     identity,
// );

// async fn write_varint_bytes(
//     mut writer: impl AsyncWrite + Send + Unpin,
//     n: impl integer_encoding::VarInt,
// ) -> Result<usize> {
//     let mut buf = [0_u8; 10];
//     let b = n.encode_var(&mut buf);
//     writer.write_all(&buf[0..b]).await?;
//     Ok(b)
// }

// async fn read_varint_bytes<VI: integer_encoding::VarInt>(
//     mut reader: impl AsyncRead + Send + Unpin,
// ) -> Result<VI> {
//     let n = reader.read_varint_async().await?;
//     Ok(n)
// }
