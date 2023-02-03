use crate::{
    io::{Read, Write},
    types::{ApiVersionsRequest, CorrelationId, ErrorCode, Request},
    Result,
};
use std::sync::atomic::{AtomicI32, Ordering};
use tokio::io::AsyncWriteExt;
use tokio::{
    io::{AsyncReadExt, BufStream, BufWriter},
    net::{TcpStream, ToSocketAddrs},
};
use tracing::debug;

pub struct Client {
    next_cid: AtomicI32,
    stream: BufStream<TcpStream>,
}

impl Client {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<Self> {
        let c = Client {
            next_cid: AtomicI32::new(0),
            stream: BufStream::new(TcpStream::connect(addr).await?),
        };
        Ok(c)
    }

    pub async fn api_versions(&mut self) -> Result<()> {
        let req = Request::new(ApiVersionsRequest::new(self.get_next_cid()));
        req.write_to(&mut self.stream).await?;
        self.stream.flush().await?;
        let err_code = ErrorCode::read_from(&mut self.stream).await?;
        debug!("Got err_code: {:?}", err_code);
        Ok(())
    }

    fn get_next_cid(&self) -> CorrelationId {
        CorrelationId::from(self.next_cid.fetch_add(1, Ordering::Relaxed))
    }
}
