use super::{ClientError, Result};
use crate::{
    connection::LazyBrokerConnection,
    formats::{
        api::{
            ProduceReqV3, ProduceReqV3PartitionData, ProduceReqV3TopicData, ProduceRespV3,
            RecordBatchAttributes, RecordBatchCompression, RecordBatchInput, RecordContent,
        },
        NullableBytes, NullableString, VarIntArray,
    },
    models::{PartitionId, TopicName},
};
use bytes::Bytes;
use derive_more::Constructor;
use non_empty_vec::ne_vec;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Message producer
#[derive(Debug, Clone, Constructor)]
pub struct ProduceClient {
    conn: LazyBrokerConnection,
}

impl ProduceClient {
    pub async fn shutdown(self) -> Result<()> {
        self.conn.shutdown().await?;
        Ok(())
    }

    pub async fn produce_v3(
        &self,
        topic_name: &TopicName,
        partition_id: PartitionId,
        message: Bytes,
        acks: i16,
        timeout: Duration,
    ) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Invalid timestamp")
            .as_secs() as i64;
        let rb = RecordBatchInput {
            base_offset: 0,
            partition_leader_epoch: 0,
            attributes: RecordBatchAttributes {
                compression: RecordBatchCompression::None,
                is_control_batch: false,
                is_transactional: false,
                has_delete_horizons: false,
            },
            last_offset_delta: 0,
            base_timestamp: timestamp,
            max_timestamp: timestamp,
            producer_id: -1,
            producer_epoch: -1,
            base_sequence: -1,
            records: ne_vec![RecordContent {
                attributes: 0,
                timestamp_delta: 0.into(),
                offset_delta: 1.into(),
                key: Default::default(),
                value: VarIntArray::from(message.to_vec()),
                headers: vec![].into(),
            }
            .into_record()],
        }
        .into_record_batch();
        let req = ProduceReqV3 {
            transactional_id: NullableString::default(),
            timeout_ms: timeout.as_millis() as i32,
            acks,
            topic_data: vec![ProduceReqV3TopicData {
                topic_name: topic_name.clone().into(),
                partition_data: vec![ProduceReqV3PartitionData {
                    partition_id: partition_id.into(),
                    records: NullableBytes(rb),
                }],
            }],
        };
        let resp: ProduceRespV3 = self.conn.send(req).await?;

        if resp
            .responses
            .iter()
            .flat_map(|r| r.partition_responses.iter())
            .any(|r| r.err_code.is_not_okay())
        {
            Err(ClientError::Producing {
                errors: resp
                    .responses
                    .into_iter()
                    .flat_map(|r| r.partition_responses)
                    .filter(|r| r.err_code.is_not_okay())
                    .map(|r| (topic_name.clone(), partition_id, r.err_code))
                    .collect(),
            })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{BrokerList, ConnectionConfig},
        tests::setup_tracing,
    };

    #[tokio::test]
    async fn test_producing() {
        setup_tracing();
        let broker_list = BrokerList(vec!["localhost:9092".into()]);
        let client_id = "test-client".into();
        let config = ConnectionConfig {
            broker_list,
            client_id,
        };
        let conn = LazyBrokerConnection::new(config);
        // let metadata_client = MetadataClient::new(conn.clone());
        let produce_client = ProduceClient::new(conn);

        let topic_name = TopicName::from(format!("KAFKAESQUE_TEST_PRODUCE"));

        // metadata_client
        //     .create_topics(
        //         [TopicSpec {
        //             name: topic_name.clone(),
        //             replication_factor: 1.into(),
        //             partition_count: 1.into(),
        //         }],
        //         Duration::from_secs(5),
        //     )
        //     .await
        //     .unwrap();

        let partition_id = PartitionId(0);
        let message = Bytes::from("Hello".as_bytes());
        let acks = -1;
        let timeout = Duration::from_secs(1);
        produce_client
            .produce_v3(&topic_name, partition_id, message, acks, timeout)
            .await
            .unwrap();

        // metadata_client.shutdown().await.unwrap();
        produce_client.shutdown().await.unwrap();
    }
}
