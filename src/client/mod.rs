use crate::{
    io::{Read, Write},
    types::{ApiVersionsRequest, CorrelationId, ErrorCode, Int32, RequestHeader, RequestMessage},
    Result,
};
use std::{
    fmt::Debug,
    sync::atomic::{AtomicI32, Ordering},
};
use tokio::io::AsyncWriteExt;
use tokio::{
    io::{AsyncReadExt, BufStream, BufWriter},
    net::{TcpStream, ToSocketAddrs},
};
use tracing::debug;

pub struct Client {
    next_cid: i32,
    stream: BufStream<TcpStream>,
}

impl Client {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<Self> {
        let c = Client {
            next_cid: 0,
            stream: BufStream::new(TcpStream::connect(addr).await?),
        };
        Ok(c)
    }

    pub async fn send<M: RequestMessage + Write + Debug>(&mut self, message: M) -> Result<()> {
        let header = self.generate_header::<M>();
        let size = header.calculate_size() + message.calculate_size();

        debug!("Sending request [size={size},header={header:?},message={message:?}]");

        Int32::from(size).write_to(&mut self.stream).await?;
        header.write_to(&mut self.stream).await?;
        message.write_to(&mut self.stream).await?;
        self.stream.flush().await?;

        // let err_code = ErrorCode::read_from(&mut self.stream).await?;
        // debug!("Got err_code: {:?}", err_code);
        Ok(())
    }

    fn generate_header<M: RequestMessage>(&mut self) -> RequestHeader {
        RequestHeader {
            api_key: M::API_KEY,
            api_version: M::API_VERSION,
            cid: self.get_next_cid(),
        }
    }

    fn get_next_cid(&mut self) -> CorrelationId {
        let cid = CorrelationId::from(self.next_cid);
        self.next_cid += 1;
        cid
    }
}
