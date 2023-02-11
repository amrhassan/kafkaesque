use super::{topic_leaders::TopicPartitionLeaders, Result};
use crate::{
    clients::ProduceClient,
    config::ConnectionConfig,
    models::{PartitionId, TopicName},
};
use bytes::Bytes;
use std::time::Duration;

/// Message producer
#[derive(Debug, Clone)]
pub struct Producer {
    leaders: TopicPartitionLeaders,
}

impl Producer {
    pub fn create(config: ConnectionConfig) -> Self {
        let leaders = TopicPartitionLeaders::create(config);
        Producer { leaders }
    }

    /// Reset the cache, including the knowledge of topic partition leaders.
    pub async fn reset_cache(&self, topic_names: &[&TopicName]) -> Result<()> {
        self.leaders.reset_cache(topic_names).await
    }

    pub async fn shutdown(self) -> Result<()> {
        self.leaders.shutdown().await
    }

    pub async fn produce(
        &self,
        topic_name: &TopicName,
        partition_id: PartitionId,
        message: Bytes,
        acks: i16,
        timeout: Duration,
    ) -> Result<()> {
        let conn = self
            .leaders
            .get_connection_to_leader(topic_name, partition_id)
            .await?;
        ProduceClient::new(conn)
            .produce_v3(topic_name, partition_id, message, acks, timeout)
            .await?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         clients::MetadataClient, config::BrokerList, models::TopicSpec, tests::setup_tracing,
//     };

//     #[tokio::test]
//     async fn test_producing() {
//         setup_tracing();
//         let broker_list = BrokerList(vec!["localhost:9092".into()]);
//         let client_id = "test-client".into();
//         let config = ConnectionConfig {
//             broker_list,
//             client_id,
//         };
//         let metadata_client = MetadataClient::create(config.clone());
//         let producer = Producer::create(config);

//         let topic_name = TopicName::from(format!("KAFKAESQUE_TEST_{}", fastrand::u32(0..9999)));

//         metadata_client
//             .create_topics(
//                 [TopicSpec {
//                     name: topic_name.clone(),
//                     replication_factor: 1.into(),
//                     partition_count: 1.into(),
//                 }],
//                 Duration::from_secs(5),
//             )
//             .await
//             .unwrap();

//         let partition_id = PartitionId(0);
//         let message = Bytes::from("Hello".as_bytes());
//         let acks = -1;
//         let timeout = Duration::from_secs(1);
//         producer
//             .produce(&topic_name, partition_id, message, acks, timeout)
//             .await
//             .unwrap();

//         // TODO: test producing

//         metadata_client.shutdown().await.unwrap();
//         producer.shutdown().await.unwrap();
//     }
// }
