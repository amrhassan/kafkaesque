use crate::formats::codec::{FixedLength, Read, Write};
use crate::formats::Result;
use std::convert::identity;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

macro_rules! fixed_length_copy_io_impl {
    ($SelfT:ty, $size:expr, $write_map:expr, $write:expr, $read:expr, $read_map:expr,) => {
        impl FixedLength for $SelfT {
            const SIZE: i32 = $size;
        }
        impl Write for $SelfT {
            fn calculate_size(&self) -> i32 {
                $size
            }
            async fn write_to(
                &self,
                sink: &mut (dyn tokio::io::AsyncWrite + Send + Unpin),
            ) -> Result<()> {
                let to_be_written = ($write_map)(self);
                $write(sink, *to_be_written).await?;
                Ok(())
            }
        }
        #[allow(clippy::redundant_closure_call)]
        impl Read for $SelfT {
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

fixed_length_copy_io_impl!(
    i8,
    1,
    identity,
    AsyncWriteExt::write_i8,
    AsyncReadExt::read_i8,
    identity,
);

fixed_length_copy_io_impl!(
    i16,
    2,
    identity,
    AsyncWriteExt::write_i16,
    AsyncReadExt::read_i16,
    identity,
);

fixed_length_copy_io_impl!(
    i32,
    4,
    identity,
    AsyncWriteExt::write_i32,
    AsyncReadExt::read_i32,
    identity,
);

fixed_length_copy_io_impl!(
    i64,
    8,
    identity,
    AsyncWriteExt::write_i64,
    AsyncReadExt::read_i64,
    identity,
);

fixed_length_copy_io_impl!(
    u32,
    4,
    identity,
    AsyncWriteExt::write_u32,
    AsyncReadExt::read_u32,
    identity,
);
