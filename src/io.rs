use crate::Result;
use tokio::io::{AsyncRead, AsyncWrite};

pub trait Write {
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()>;
}

pub trait Read: Sized {
    async fn read_from(reader: &mut (dyn AsyncRead + Send + Unpin)) -> Result<Self>;
}

pub trait FixedLength {
    const SIZE: i32;
}

pub trait VariableLength {
    /// Calculate the required size in bytes
    fn calculate_size(&self) -> i32;
}
