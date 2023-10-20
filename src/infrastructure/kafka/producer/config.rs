use std::time::Duration;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use log::{info, warn};

pub struct KafkaProducerConfig;

impl KafkaProducerConfig {
    pub fn create_producer() -> FutureProducer {
        ClientConfig::new()
            .set("group.id", "kafka")
            .set(
                "bootstrap.servers",
                std::env::var("KAFKA_BROKER").unwrap(),
            )
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", std::env::var("KAFKA_MECHANISMS").unwrap())
            .set("sasl.username", std::env::var("KAFKA_USER").unwrap())
            .set(
                "sasl.password", std::env::var("KAFKA_PASSWORD").unwrap(),
            )
            .set("message.timeout.ms", "45000")
            .create()
            .expect("Producer creation failed")
    }

    // TODO: send without caller blocking
    pub async fn send_message(producer: &FutureProducer, message: &Vec<u8>, kafka_topic: &str) {
        let record: FutureRecord<String, Vec<u8>> = FutureRecord::to(kafka_topic).payload(&message);
        let delivery_status = producer
            .send(record, Timeout::After(Duration::from_secs(0)))
            .await;

        match delivery_status {
            Ok(_) => info!("Message was sent, topic: {}", kafka_topic),
            Err(res) => warn!("Message wasn't sent: {}", res.0),
        }
    }
}
