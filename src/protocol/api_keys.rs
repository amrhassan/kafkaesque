use super::codec::{FixedLength, Write};
use crate::Result;
use tokio::io::AsyncWrite;

#[derive(Debug, Clone, Copy)]
pub enum ApiKey {
    ApiVersions = 18,
}

impl Write for ApiKey {
    fn calculate_size(&self) -> i32 {
        i16::SIZE
    }
    async fn write_to(&self, writer: &mut (dyn AsyncWrite + Send + Unpin)) -> Result<()> {
        (*self as i16).write_to(writer).await
    }
}
