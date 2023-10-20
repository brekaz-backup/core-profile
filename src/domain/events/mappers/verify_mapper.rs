use anyhow::Result;
use async_trait::async_trait;
use blumer_lib_errors::AppError;
use rdkafka::producer::FutureProducer;
use crate::domain::protos::schema::profile_edit_capnp::verify_user as VerifyUserProto;

use uuid::Uuid;
use crate::infrastructure::kafka::{KafkaProducerConfig, KafkaProducerInterface};

#[derive(Default)]
pub struct VerifyEventMapper;

impl VerifyEventMapper {
    #[allow(unused)]
    pub async fn proto(user_id: &Uuid, status: bool) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<VerifyUserProto::Builder>();
        proto.set_user_id(&user_id.to_string());
        proto.set_status(status);
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message)?;
        return Ok(buf_slice);
    }

    pub async fn entity(payload: &[u8]) -> Result<(Uuid, bool,)> {
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let proto = message_reader.get_root::<VerifyUserProto::Reader>()?;
        Ok(
                (
                    Uuid::parse_str(proto.get_user_id()?)?,
                    proto.get_status()
                )
        )

    }
}

pub struct VerifyEventProducer {
    producer: FutureProducer,
}

impl VerifyEventProducer {
    pub fn new(producer: FutureProducer) -> Self {
        VerifyEventProducer { producer }
    }
}

#[async_trait]
impl KafkaProducerInterface<(Uuid, bool,)> for VerifyEventProducer {
    async fn send_message(&self, kafka_topic: &str, message: &(Uuid, bool)) -> Result<()> {
        let message = VerifyEventMapper::proto(&message.0, message.1).await?;
        KafkaProducerConfig::send_message(&self.producer, &message, kafka_topic).await;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use log::info;
    use serde_json::json;
    use uuid::Uuid;
    use crate::domain::events::reducer::REDUCER_VERIFY_TOPIC;


    use super::*;

    async fn get_event_producer() -> VerifyEventProducer {
        return VerifyEventProducer {
            producer: KafkaProducerConfig::create_producer()
        };
    }

    #[tokio::test]
    async fn test_wallet_validation_event_producer() {
        init();

        let producer = get_event_producer().await;
        let event_result = producer.send_message(REDUCER_VERIFY_TOPIC, &(Uuid::parse_str("4d1f4740-2483-4c4a-9e8c-26e4a8bfbba3").unwrap(), true,)).await;
        info!("VerifyEventProducer Result {:?}", event_result);
        assert!(event_result.is_ok())
    }

    fn init() {
        dotenv().ok();
        let _ = env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).try_init();
    }

    #[tokio::test]
    async fn test_verify_event_mapper_to_proto() {
        init();
        let user_id = Uuid::new_v4();
        let status = true;


        let data = VerifyEventMapper::proto(&user_id, status).await.expect("Error getting proto");
        info!("VerifyEventMapper {:?}", data);
        let entity = VerifyEventMapper::entity(&data).await.expect("Error decoding verify");
        info!("VerifyEventMapper {:?}", entity);
        assert_eq!(false, data.is_empty());
    }
}

