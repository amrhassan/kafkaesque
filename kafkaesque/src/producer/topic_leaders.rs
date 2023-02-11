use super::{ProducerError, Result};
use crate::{
    clients::MetadataClient,
    config::{BrokerList, ConnectionConfig},
    connection::LazyBrokerConnection,
    models::{NodeId, PartitionId, TopicName},
};
use futures::{future::try_join_all, TryFutureExt};
use itertools::Itertools;
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::RwLock, try_join};

/// Owner of topic partition leadership information
///
/// Caches the data internally for subsequent use.
#[derive(Debug, Clone)]
pub struct TopicPartitionLeaders {
    /// Leaders of topic partitions
    cache: Arc<RwLock<HashMap<(TopicName, PartitionId), LazyBrokerConnection>>>,
    metadata_client: MetadataClient,
    config: Arc<ConnectionConfig>,
}

impl TopicPartitionLeaders {
    pub fn create(config: ConnectionConfig) -> Self {
        let metadata_client = MetadataClient::create(config.clone());
        let cache = Default::default();
        TopicPartitionLeaders {
            cache,
            metadata_client,
            config: Arc::new(config),
        }
    }

    /// Find out and create a connection to the leader of the specified topic partition
    pub async fn get_connection_to_leader(
        &self,
        topic_name: &TopicName,
        partition_id: PartitionId,
    ) -> Result<LazyBrokerConnection> {
        let key = (topic_name.clone(), partition_id);
        let read_lock = self.cache.read().await;
        if let Some(conn) = read_lock.get(&key).cloned() {
            Ok(conn)
        } else {
            drop(read_lock);
            let topic_leaders = self.find_topic_leaders(topic_name).await?;
            let mut write_lock = self.cache.write().await;
            for (node_id, conn) in topic_leaders {
                write_lock.insert((topic_name.clone(), node_id), conn);
            }
            write_lock.get(&key).cloned().ok_or_else(|| {
                ProducerError::TopicPartitionLeaderNotFound {
                    topic_name: topic_name.clone(),
                    partition_id,
                }
            })
        }
    }

    /// Reset the cache thus re-learning about topic partion leaders on subsequent attempts
    /// to use the connections
    pub async fn reset_cache(&self, topic_names: &[&TopicName]) -> Result<()> {
        let mut lock = self.cache.write().await;
        let keys_to_remove = lock
            .keys()
            .cloned()
            .filter(|(name, _)| topic_names.contains(&name))
            .collect_vec();
        for key in keys_to_remove {
            if let Some(conn) = lock.remove(&key) {
                conn.shutdown().await?;
            }
        }
        Ok(())
    }

    pub async fn shutdown(self) -> Result<()> {
        try_join!(
            self.metadata_client.shutdown().map_err(ProducerError::from),
            try_join_all(
                self.cache
                    .write()
                    .await
                    .drain()
                    .map(|(_key, value)| value.shutdown().map_err(ProducerError::from)),
            )
        )?;
        Ok(())
    }

    async fn find_topic_leaders(
        &self,
        topic_name: &TopicName,
    ) -> Result<Vec<(PartitionId, LazyBrokerConnection)>> {
        let metadata = self
            .metadata_client
            .get_metadata([topic_name.clone()])
            .await?;
        let brokers: HashMap<NodeId, LazyBrokerConnection> = metadata
            .brokers
            .into_iter()
            .map(|broker| {
                let config = ConnectionConfig {
                    broker_list: BrokerList::from_hostnames_and_ports([(
                        broker.host,
                        broker.port as u16,
                    )]),
                    client_id: self.config.client_id.clone(),
                };
                let conn = LazyBrokerConnection::new(config);
                (broker.id, conn)
            })
            .collect();
        let res = metadata
            .topics
            .into_iter()
            .filter(|topic| &topic.name == topic_name)
            .flat_map(|topic| topic.partitions)
            .map(|partition| {
                let conn = brokers.get(&partition.leader).cloned().ok_or(
                    ProducerError::NodeMetadataNotFound {
                        node_id: partition.leader,
                    },
                )?;
                Result::Ok((partition.id, conn))
            })
            .try_collect()?;
        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TopicPartition {
    pub topic_name: TopicName,
    pub partition_id: PartitionId,
}
