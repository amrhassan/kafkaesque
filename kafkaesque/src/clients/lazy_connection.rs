use super::{ClientConfig, Result};
use crate::formats::BrokerConnection;
use futures::future::select_ok;
use std::sync::Arc;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};
use tracing::trace;

/// A lazily-initialized connection to one of several available brokers.
#[derive(Debug, Clone)]
pub struct LazyBrokerConnection {
    config: ClientConfig,
    conn: Arc<Mutex<Option<BrokerConnection>>>,
}

impl LazyBrokerConnection {
    #[allow(unused)]
    pub fn new(config: ClientConfig) -> Self {
        LazyBrokerConnection {
            config,
            conn: Default::default(),
        }
    }

    /// Get a working mutable reference to a broker connection, initializing one beforehand if
    /// necessary.
    #[allow(unused)]
    pub async fn get_connection(&self) -> Result<MappedMutexGuard<BrokerConnection>> {
        let mut lock = self.conn.lock().await;
        if lock.is_none() {
            trace!("creating connection to");
            lock.insert(self.connect_to_single_broker().await?);
        }
        Ok(MutexGuard::map(lock, |l| {
            l.as_mut().expect("BrokerConnection is missing")
        }))
    }

    #[allow(unused)]
    pub async fn reset(&self) -> Result<()> {
        trace!("resetting connection");
        let mut lock = self.conn.lock().await;
        let new_connection = self.connect_to_single_broker().await?;
        if let Some(old_conn) = lock.replace(new_connection) {
            old_conn.shutdown().await?;
        }
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
        clients::BrokerList,
        formats::{
            messages::{ApiVersionsReq, ApiVersionsResp},
            ErrorCode,
        },
    };

    #[tokio::test]
    async fn test_lazy_connection() {
        let broker_list = BrokerList(vec!["localhost:9092".into()]);
        let client_id = "test-client".into();
        let client_config = ClientConfig {
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
        assert_eq!(resp.error_code, ErrorCode::None);
        assert!(!resp.api_keys.is_empty())
    }
}
