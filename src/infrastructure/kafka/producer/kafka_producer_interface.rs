use async_trait::async_trait;

use anyhow::Result;

#[async_trait]
pub trait KafkaProducerInterface<V: ?Sized> {
    async fn send_message(
        &self,
        kafka_topic: &str,
        message: &V,
    ) -> Result<()>;
}


//pub trait KafkaProducerInterface<V: ?Sized> {