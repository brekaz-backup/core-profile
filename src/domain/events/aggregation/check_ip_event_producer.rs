use async_trait::async_trait;
use rdkafka::producer::FutureProducer;
use anyhow::Result;
use uuid::Uuid;
use crate::domain::events::mappers::CheckIpEventMapper;
use crate::infrastructure::kafka::{KafkaProducerConfig, KafkaProducerInterface};


pub struct CheckIpEventProducer {
    producer: FutureProducer,
}

impl CheckIpEventProducer {
    pub fn new(producer: FutureProducer) -> Self {
        CheckIpEventProducer { producer }
    }
}

#[async_trait]
impl KafkaProducerInterface<(Uuid, String, )> for CheckIpEventProducer {
    async fn send_message(
        &self,
        kafka_topic: &str,
        message: &(Uuid, String, ),
    ) -> Result<()> {
        let message = CheckIpEventMapper::proto(&message.0, &message.1).await?;
        KafkaProducerConfig::send_message(&self.producer, &message, kafka_topic).await;
        Ok(())
    }
}