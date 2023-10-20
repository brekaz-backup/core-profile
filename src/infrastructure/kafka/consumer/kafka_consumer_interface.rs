use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait KafkaConsumerInterface {
    async fn consume(
        &self,
        kafka_topic: &str,
    ) -> Result<()>;
}