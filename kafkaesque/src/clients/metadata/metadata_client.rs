use super::Metadata;
use crate::{
    clients::{lazy_connection::LazyBrokerConnection, ClientConfig, Result},
    formats::messages::{MetadataRequestV0, MetadataResponseV0, ReqV0Topic},
};

/// Metadata client
#[derive(Debug, Clone)]
pub struct MetadataClient {
    conn: LazyBrokerConnection,
}

impl MetadataClient {
    pub fn new(config: ClientConfig) -> Self {
        MetadataClient {
            conn: LazyBrokerConnection::new(config),
        }
    }

    /// Get topic and broker metadata for topic names
    pub async fn get_metadata(
        &self,
        topic_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Metadata> {
        let req = MetadataRequestV0 {
            topics: topic_names
                .into_iter()
                .map(|name| ReqV0Topic { name: name.into() })
                .collect(),
        };
        let resp: MetadataResponseV0 = self.conn.get_connection().await?.send(req).await?;
        Ok(resp.into())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::clients::*;
//     use tracing::info;
//     use tracing_subscriber::{fmt, prelude::*, EnvFilter};

//     #[tokio::test]
//     async fn test_reading_topic_metadata() {
//         setup_tracing();
//         let broker_list = BrokerList(vec!["localhost:9092".into()]);
//         let client_id = "test-client".into();
//         let client_config = ClientConfig {
//             broker_list,
//             client_id,
//         };
//         let client = MetadataClient::new(client_config);
//         let metadata = client
//             .get_metadata(&["AmrTests".to_string()])
//             .await
//             .unwrap();
//         println!("{:#?}", metadata);
//         panic!("FAILED")
//     }

//     pub fn setup_tracing() {
//         let tracing_fmt_layer = fmt::layer().with_target(false).with_ansi(true);
//         let tracing_filter_layer = EnvFilter::try_from_default_env()
//             .or_else(|_| EnvFilter::try_new("debug"))
//             .expect("Failed to set global logging filter");
//         tracing_subscriber::registry()
//             .with(tracing_filter_layer)
//             .with(tracing_fmt_layer)
//             .init();
//         info!("Initiated tracing");
//     }
// }
