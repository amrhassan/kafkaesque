use super::{BrokerConnection, Result};
use crate::{
    config::ConnectionConfig,
    formats::{Read, RequestMessage, Write},
};
use futures::future::select_ok;
use std::{fmt::Debug, sync::Arc};
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};
use tracing::trace;

/// A lazily-initialized connection to one of several available brokers.
#[derive(Debug, Clone)]
pub struct LazyBrokerConnection {
    config: Arc<ConnectionConfig>,
    conn: Arc<Mutex<Option<BrokerConnection>>>,
}

impl LazyBrokerConnection {
    pub fn new(config: ConnectionConfig) -> Self {
        LazyBrokerConnection {
            config: Arc::new(config),
            conn: Default::default(),
        }
    }

    pub async fn shutdown(self) -> Result<()> {
        if let Some(conn) = self.conn.lock().await.take() {
            conn.shutdown().await?;
        }
        Ok(())
    }

    /// Send a single request message and read its response.
    pub async fn send<Req: RequestMessage + Write + Debug, Resp: Read + Debug>(
        &self,
        message: Req,
    ) -> Result<Resp> {
        self.get_connection().await?.send(message).await
    }

    /// Batch-send multiple request messages before reading their responses.
    pub async fn send_many<ReqM: RequestMessage + Write + Debug, Resp: Read + Debug>(
        &self,
        messages: impl IntoIterator<Item = ReqM>,
    ) -> Result<Vec<Resp>> {
        self.get_connection().await?.send_many(messages).await
    }

    /// Get a working mutable reference to a broker connection, initializing one beforehand if
    /// necessary.
    pub async fn get_connection(&self) -> Result<MappedMutexGuard<BrokerConnection>> {
        let mut lock = self.conn.lock().await;
        if lock.is_none() {
            trace!("creating connection to");
            let _ = lock.insert(self.connect_to_single_broker().await?);
        }
        Ok(MutexGuard::map(lock, |l| {
            l.as_mut().expect("BrokerConnection is missing")
        }))
    }

    pub async fn reset(&self) -> Result<()> {
        trace!("resetting connection");
        let mut lock = self.conn.lock().await;
        let new_connection = self.connect_to_single_broker().await?;
        lock.replace(new_connection);
        Ok(())
    }

    /// Attempt to connect to any the listed brokers, return the first connection
    /// that gets established.
    async fn connect_to_single_broker(&self) -> Result<BrokerConnection> {
        let (c, _) = select_ok(self.config.broker_list.iter().map(|address| {
            Box::pin(BrokerConnection::connect(
                self.config.client_id.clone(),
                address.as_to_socket_address(),
            ))
        }))
        .await?;
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::BrokerList,
        formats::{
            api::{ApiVersionsReq, ApiVersionsResp},
            ErrorCode,
        },
    };

    #[tokio::test]
    async fn test_lazy_connection() {
        let broker_list = BrokerList(vec!["localhost:9092".into()]);
        let client_id = "test-client".into();
        let client_config = ConnectionConfig {
            broker_list,
            client_id,
        };
        let conn = LazyBrokerConnection::new(client_config);
        let resp: ApiVersionsResp = conn
            .get_connection()
            .await
            .unwrap()
            .send(ApiVersionsReq)
            .await
            .unwrap();
        conn.shutdown().await.unwrap();
        assert_eq!(resp.error_code, ErrorCode::None);
        assert!(!resp.api_keys.is_empty())
    }
}
