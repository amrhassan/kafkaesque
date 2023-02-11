use super::{
    codec::{Read, Write},
    request::{CorrelationId, RequestHeader, RequestMessage},
    Result, DEFAULT_BUF_SIZE,
};
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;
use tokio::{
    io::BufStream,
    net::{TcpStream, ToSocketAddrs},
};
use tracing::debug;

/// A connection to a single broker
#[derive(Debug)]
pub struct BrokerConnection {
    next_cid: i32,
    stream: BufStream<TcpStream>,
    client_id: String,
}

impl BrokerConnection {
    pub async fn connect(client_id: impl Into<String>, addr: impl ToSocketAddrs) -> Result<Self> {
        Self::connect_with_buffer_size(client_id.into(), addr, DEFAULT_BUF_SIZE, DEFAULT_BUF_SIZE)
            .await
    }

    pub async fn connect_with_buffer_size(
        client_id: impl Into<String>,
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
            client_id: client_id.into(),
        };
        Ok(c)
    }

    pub async fn send<Req: RequestMessage + Write + Debug, Resp: Read + Debug>(
        &mut self,
        message: Req,
    ) -> Result<Resp> {
        self.write_request(message).await?;
        self.stream.flush().await?;
        self.read_response().await
    }

    pub async fn send_many<ReqM: RequestMessage + Write + Debug, Resp: Read + Debug>(
        &mut self,
        messages: impl IntoIterator<Item = ReqM>,
    ) -> Result<Vec<Resp>> {
        let mut len = 0;
        for message in messages {
            self.write_request(message).await?;
            len += 1;
        }
        self.stream.flush().await?;
        let mut responses = Vec::with_capacity(len);
        for _ in 0..len {
            responses.push(self.read_response().await?);
        }
        Ok(responses)
    }

    pub async fn shutdown(mut self) -> Result<()> {
        self.stream.shutdown().await?;
        Ok(())
    }

    async fn write_request<ReqM: RequestMessage + Write + Debug>(
        &mut self,
        message: ReqM,
    ) -> Result<()> {
        let header = self.generate_header::<ReqM>();
        let req_len = header.calculate_size() + message.calculate_size();
        debug!("Sending request [len={req_len},header={header:?},message={message:?}]");
        req_len.write_to(&mut self.stream).await?;
        header.write_to(&mut self.stream).await?;
        message.write_to(&mut self.stream).await?;
        Ok(())
    }

    async fn read_response<Resp: Read + Debug>(&mut self) -> Result<Resp> {
        let resp_len = i32::read_from(&mut self.stream).await?;
        let resp_cid = CorrelationId::read_from(&mut self.stream).await?;

        debug!("Received response [len={resp_len},cid={resp_cid:?}]",);

        let resp = Resp::read_from(&mut self.stream).await?;

        Ok(resp)
    }

    fn generate_header<M: RequestMessage>(&mut self) -> RequestHeader {
        RequestHeader {
            api_key: M::API_KEY,
            api_version: M::API_VERSION,
            cid: self.get_next_cid(),
            client_id: self.client_id.clone(),
        }
    }

    fn get_next_cid(&mut self) -> CorrelationId {
        let cid = CorrelationId::from(self.next_cid);
        self.next_cid += 1;
        cid
    }
}
