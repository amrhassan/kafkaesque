use crate::{
    clients::{ClientError, Result},
    config::ConnectionConfig,
    connection::LazyBrokerConnection,
    formats::{
        api::{
            CreateTopicsReqV0, CreateTopicsReqV0CreateTopic, CreateTopicsRespV0, DeleteTopicsReqV0,
            DeleteTopicsRespV0, MetadataReqV0Topic, MetadataRequestV0, MetadataResponseV0,
        },
        ErrorCode,
    },
    models::{Metadata, TopicName, TopicSpec},
};
use derive_more::Constructor;
use itertools::Itertools;
use std::time::Duration;

/// Metadata client
#[derive(Debug, Clone, Constructor)]
pub struct MetadataClient {
    conn: LazyBrokerConnection,
}

impl MetadataClient {
    pub fn create(config: ConnectionConfig) -> Self {
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
            .filter(|t| t.err_code != ErrorCode::None)
            .map(|t| (TopicName::from(t.name), t.err_code))
            .collect_vec();

        if !errors.is_empty() {
            Err(ClientError::TopicCreation { errors })
        } else {
            Ok(())
        }
    }

    /// Delete topics
    pub async fn delete_topics(
        &self,
        topic_names: impl IntoIterator<Item = TopicName>,
        timeout: Duration,
    ) -> Result<()> {
        let req = DeleteTopicsReqV0 {
            topic_names: topic_names.into_iter().map_into().collect(),
            timeout_ms: timeout.as_millis() as i32,
        };

        let resp: DeleteTopicsRespV0 = self.conn.get_connection().await?.send(req).await?;

        let errors = resp
            .topics
            .into_iter()
            .filter(|t| t.err_code != ErrorCode::None)
            .map(|t| (TopicName::from(t.name), t.err_code))
            .collect_vec();

        if !errors.is_empty() {
            Err(ClientError::TopicDeletion { errors })
        } else {
            Ok(())
        }
    }

    pub async fn shutdown(self) -> Result<()> {
        self.conn.shutdown().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        clients::*,
        config::{BrokerList, ConnectionConfig},
        models::{TopicName, TopicSpec},
        tests::setup_tracing,
    };
    use std::time::Duration;

    #[tokio::test]
    async fn test_topic_creation_and_reading_topic_metadata() {
        setup_tracing();
        let broker_list = BrokerList(vec!["localhost:9092".into()]);
        let client_id = "test-client".into();
        let client_config = ConnectionConfig {
            broker_list,
            client_id,
        };
        let client = MetadataClient::create(client_config);

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

        let metadata = client.get_metadata([topic_name.clone()]).await.unwrap();
        println!("{metadata:#?}");

        client
            .delete_topics([topic_name], Duration::from_secs(5))
            .await
            .unwrap();

        client.shutdown().await.unwrap()
    }
}
