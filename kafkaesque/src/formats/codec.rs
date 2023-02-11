use super::Result;
use tokio::io::{AsyncRead, AsyncWrite};

pub use kafkaesque_macros::{Read, Write};

pub trait Write {
    /// Calculate the total size in bytes
    fn calculate_size(&self) -> i32;

    /// Encode bytes
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()>;
}

pub trait Read: Sized {
    /// Decode from bytes
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self>;
}

pub trait FixedLength {
    const SIZE: i32;
}
