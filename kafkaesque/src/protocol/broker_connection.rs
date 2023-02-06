use super::{
    codec::{Read, Write},
    request::{CorrelationId, RequestHeader, RequestMessage},
    response::Response,
    DEFAULT_BUF_SIZE,
};
use crate::{protocol::response::ErrorCode, Result};
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;
use tokio::{
    io::BufStream,
    net::{TcpStream, ToSocketAddrs},
};
use tracing::debug;

pub struct BrokerConnection {
    next_cid: i32,
    stream: BufStream<TcpStream>,
    client_id: &'static str,
}

impl BrokerConnection {
    pub async fn connect(client_id: &'static str, addr: impl ToSocketAddrs) -> Result<Self> {
        Self::connect_with_buffer_size(client_id, addr, DEFAULT_BUF_SIZE, DEFAULT_BUF_SIZE).await
    }

    pub async fn connect_with_buffer_size(
        client_id: &'static str,
        addr: impl ToSocketAddrs,
        read_buf_size: usize,
        write_buf_size: usize,
    ) -> Result<Self> {
        let c = BrokerConnection {
            next_cid: 0,
            stream: BufStream::with_capacity(
                read_buf_size,
                write_buf_size,
                TcpStream::connect(addr).await?,
            ),
            client_id,
        };
        Ok(c)
    }

    pub async fn send<ReqM: RequestMessage + Write + Debug, Resp: Read + Debug>(
        &mut self,
        message: ReqM,
    ) -> Result<Response<Resp>> {
        let header = self.generate_header::<ReqM>();
        let req_len = header.calculate_size() + message.calculate_size();

        debug!("Sending request [len={req_len},header={header:?},message={message:?}]");

        req_len.write_to(&mut self.stream).await?;
        header.write_to(&mut self.stream).await?;
        message.write_to(&mut self.stream).await?;
        self.stream.flush().await?;

        let resp_len = i32::read_from(&mut self.stream).await?;
        let resp_cid = CorrelationId::read_from(&mut self.stream).await?;
        let err_code = ErrorCode::read_from(&mut self.stream).await?;

        debug!(
            "Received response [len={resp_len},cid={resp_cid:?},error_code={:?}]",
            err_code
        );

        let resp_message = Resp::read_from(&mut self.stream).await?;

        Ok(Response {
            err_code,
            message: resp_message,
        })
    }

    fn generate_header<M: RequestMessage>(&mut self) -> RequestHeader {
        RequestHeader {
            api_key: M::API_KEY,
            api_version: M::API_VERSION,
            cid: self.get_next_cid(),
            client_id: self.client_id,
        }
    }

    fn get_next_cid(&mut self) -> CorrelationId {
        let cid = CorrelationId::from(self.next_cid);
        self.next_cid += 1;
        cid
    }
}
