use async_trait::async_trait;
use rdkafka::producer::FutureProducer;
use anyhow::Result;
use crate::domain::entities::ProfileEntity;
use crate::domain::events::mappers::ProfileEditEventMapper;
use crate::infrastructure::kafka::{KafkaProducerConfig, KafkaProducerInterface};


pub struct ProfileEditEventProducer {
    producer: FutureProducer,
}

impl ProfileEditEventProducer {
    pub fn new(producer: FutureProducer) -> Self {
        ProfileEditEventProducer { producer }
    }
}

#[async_trait]
impl KafkaProducerInterface<ProfileEntity> for ProfileEditEventProducer {
    async fn send_message(
        &self,
        kafka_topic: &str,
        message: &ProfileEntity,
    ) -> Result<()> {
        let message = ProfileEditEventMapper::proto(&message).await?;
        KafkaProducerConfig::send_message(&self.producer, &message, kafka_topic).await;
        Ok(())
    }
}