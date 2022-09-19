// use crate::{
//     io::{Read, Write},
//     Result,
// };
// use derive_more::{Display, From, Into};
// use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

// #[derive(Debug, Clone, Display, From, Into)]
// pub struct CompactString(pub std::string::String);

// #[async_trait::async_trait]
// impl Read for CompactString {
//     async fn read_from(mut reader: impl AsyncRead + Send + Sync + Unpin) -> Result<Self> {
//         todo!()
//         // let len = Int16::read_from(&mut source).await?;
//         // let mut buf = vec![0; len.0 as usize];
//         // source.read_exact(&mut buf).await?;
//         // let s = String(std::string::String::from_utf8(buf)?);
//         // Ok(s)
//     }
// }

// #[async_trait::async_trait]
// impl Write for CompactString {
//     async fn write_to(&self, mut writer: impl AsyncWrite + Send + Sync + Unpin) -> Result<()> {
//         // let len = self.0.len() as u64;
//         todo!()
//         // unsigned_varint::aio
//         // sink.write_all(self.0.as_bytes()).await?;
//         // Ok(())
//     }
// }
