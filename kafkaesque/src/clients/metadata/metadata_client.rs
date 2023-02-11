use std::time::Duration;

use itertools::Itertools;

use super::{Metadata, TopicName, TopicSpec};
use crate::{
    clients::{lazy_connection::LazyBrokerConnection, ClientConfig, ClientError, Result},
    formats::{
        messages::{
            CreateTopicsReqV0, CreateTopicsReqV0CreateTopic, CreateTopicsRespV0,
            MetadataReqV0Topic, MetadataRequestV0, MetadataResponseV0,
        },
        ErrorCode,
    },
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
        topic_names: impl IntoIterator<Item = impl Into<TopicName>>,
    ) -> Result<Metadata> {
        let req = MetadataRequestV0 {
            topics: topic_names
                .into_iter()
                .map(|name| MetadataReqV0Topic {
                    name: name.into().into(),
                })
                .collect(),
        };
        let resp: MetadataResponseV0 = self.conn.get_connection().await?.send(req).await?;
        Ok(resp.into())
    }

    /// Create topics
    pub async fn create_topics(
        &self,
        topic_specs: impl IntoIterator<Item = TopicSpec>,
        timeout: Duration,
    ) -> Result<()> {
        let req = CreateTopicsReqV0 {
            topics: topic_specs
                .into_iter()
                .map(|def| CreateTopicsReqV0CreateTopic {
                    name: def.name.into(),
                    num_partitions: def.partition_count.into(),
                    replication_factor: def.replication_factor.into(),
                    assignments: vec![],
                    configs: vec![],
                })
                .collect(),
            timeout_ms: timeout.as_millis() as i32,
        };

        let resp: CreateTopicsRespV0 = self.conn.get_connection().await?.send(req).await?;

        let errors = resp
            .topics
            .into_iter()
            .filter(|t| t.err_code != ErrorCode::NONE)
            .map(|t| (TopicName::from(t.name), t.err_code))
            .collect_vec();

        if !errors.is_empty() {
            Err(ClientError::TopicCreation { errors })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::clients::*;
    use tracing::info;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    #[tokio::test]
    async fn test_topic_creation_and_reading_topic_metadata() {
        setup_tracing();
        let broker_list = BrokerList(vec!["localhost:9092".into()]);
        let client_id = "test-client".into();
        let client_config = ClientConfig {
            broker_list,
            client_id,
        };
        let client = MetadataClient::new(client_config);

        let topic_name = TopicName::from(format!("KAFKAESQUE_TEST_{}", fastrand::u32(0..9999)));

        client
            .create_topics(
                [TopicSpec {
                    name: topic_name.clone(),
                    replication_factor: 1.into(),
                    partition_count: 1.into(),
                }],
                Duration::from_secs(5),
            )
            .await
            .unwrap();

        let metadata = client.get_metadata([topic_name]).await.unwrap();

        println!("{metadata:#?}")
    }

    pub fn setup_tracing() {
        let tracing_fmt_layer = fmt::layer().with_target(false).with_ansi(true);
        let tracing_filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("debug"))
            .expect("Failed to set global logging filter");
        tracing_subscriber::registry()
            .with(tracing_filter_layer)
            .with(tracing_fmt_layer)
            .init();
        info!("Initiated tracing");
    }
}
