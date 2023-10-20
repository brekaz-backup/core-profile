use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;

pub struct KafkaConsumerConfig;

impl KafkaConsumerConfig {
    pub fn create_consumer(group_id: String, kafka_topics: Vec<&str>) -> StreamConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &group_id)
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
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "45000")
            .set("enable.auto.commit", "true")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(kafka_topics.as_slice())
            .expect("Can't subscribe to specified topics");

        consumer
    }
}
