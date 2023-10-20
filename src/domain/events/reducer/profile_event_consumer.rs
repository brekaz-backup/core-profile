use anyhow::{Result};

use futures::TryStreamExt;
use log::{info, warn};
use rdkafka::consumer::StreamConsumer;
use rdkafka::Message;
use rdkafka::message::{BorrowedMessage, Headers, OwnedMessage};
use rdkafka::producer::{FutureProducer};

use crate::domain::entities::PhotoTypeEntity;
use crate::domain::events::aggregation::{AGGREGATE_CHECK_IP_TOPIC,
                                         AGGREGATE_PROFILE_EDIT_TOPIC, CheckIpEventProducer, ProfileEditEventProducer};
use crate::domain::events::mappers::{ProfileEventMapper, ProfilePhotoEventMapper, VerifyEventMapper};
use crate::domain::events::reducer::{REDUCER_CREATE_PROFILE_TOPIC, REDUCER_PROFILE_PHOTO_TOPIC, REDUCER_VERIFY_TOPIC};
use crate::domain::usecases::reducer::{CreateProfileUseCase, EditPhotoUseCase, EditPortraitUseCase, VerifyUseCase};
use crate::infrastructure::kafka::{KafkaProducerInterface};
use crate::persistence::repositories::{ProfileRepository, RedisRepository, ReducerRepository};

pub struct ProfileEventConsumer {
    consumer: StreamConsumer,
    producer: FutureProducer,
    reducer_repository: ReducerRepository,
    profile_repository: ProfileRepository,
    redis_repository: RedisRepository,
}

impl ProfileEventConsumer {
    pub fn new(reducer_repository: ReducerRepository,
               profile_repository: ProfileRepository,
               redis_repository: RedisRepository,
               consumer: StreamConsumer,
               producer: FutureProducer) -> Self {
        ProfileEventConsumer { reducer_repository, profile_repository, redis_repository, consumer, producer }
    }

    async fn record_borrowed_message_receipt(&self, msg: &BorrowedMessage<'_>) {
        info!("Message received Borrowed: {:?}", msg);
    }

    async fn record_owned_message_receipt(&self, _msg: &OwnedMessage) -> Result<()> {
        info!("Message received Owned: {:?}", _msg);
        let reducer_repository = &self.reducer_repository;
        let profile_repository = &self.profile_repository;
        let redis_repository = &self.redis_repository;


        match _msg.topic() {
            REDUCER_CREATE_PROFILE_TOPIC => {
                info!("Creating profile");
                let entity = ProfileEventMapper::entity(&_msg.payload().unwrap()).await.map_err(|e| {
                    info!("Error  ProfileEventMapper {:?}", e);
                    e
                })?;
                CreateProfileUseCase::execute(&reducer_repository, entity.clone()).await.map_err(|e| {
                    info!("Error CreateProfileUseCase {:?}", e);
                    e
                })?;
                info!("Profile was created successfully!");
                if let Some(headers) = _msg.headers() {
                    let header = headers.get_as::<str>(0)?;
                    info!(" header key {:?}", header);
                    if let Some(ip) = header.value {
                        let email_ip_producer = CheckIpEventProducer::new(self.producer.clone());
                        email_ip_producer.send_message(AGGREGATE_CHECK_IP_TOPIC, &(entity.user_id, ip.to_string(), )).await?;
                    }
                }
            }
            REDUCER_VERIFY_TOPIC => {
                info!("Verify users");
                let (user_id, status) = VerifyEventMapper::entity(&_msg.payload().unwrap()).await.map_err(|e| {
                    info!("Error VerifyEventMapper {:?}", e);
                    e
                })?;
                let profile = VerifyUseCase::execute(&reducer_repository, &profile_repository, &redis_repository, &user_id, status).await.map_err(|e| {
                    info!("VerifyUseCase event {:?}", e);
                    e
                })?;

                let profile_edit_event = ProfileEditEventProducer::new(self.producer.clone());
                profile_edit_event.send_message(AGGREGATE_PROFILE_EDIT_TOPIC, &profile).await.map_err(|e| {
                    info!("profile_edit_event error: {}", e.to_string());
                    e
                })?;
            }
            REDUCER_PROFILE_PHOTO_TOPIC => {
                let entity = ProfilePhotoEventMapper::entity(&_msg.payload().unwrap()).await.map_err(|e| {
                    info!("Error on Photo event {:?}", e);
                    e
                })?;
                let profile = match entity.photo_type {
                    PhotoTypeEntity::Profile => EditPhotoUseCase::execute(&reducer_repository, &profile_repository, &redis_repository, &entity).await.map_err(|e| {
                        info!("Error on Photo event {:?}", e);
                        e
                    })?,
                    PhotoTypeEntity::Portrait => EditPortraitUseCase::execute(&reducer_repository, &profile_repository, &redis_repository, &entity).await.map_err(|e| {
                        info!("Error on Photo event {:?}", e);
                        e
                    })?
                };

                let profile_edit_event = ProfileEditEventProducer::new(self.producer.clone());
                profile_edit_event.send_message(AGGREGATE_PROFILE_EDIT_TOPIC, &profile).await.map_err(|e| {
                    info!("profile_edit_event error: {}", e.to_string());
                    e
                })?;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn consume(&self) {
        info!("Starting event loop");
        loop {
            match self.consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(borrowed_message) => {
                    self.record_borrowed_message_receipt(&borrowed_message).await;
                    let owned_message = borrowed_message.detach();
                    let _ = self.record_owned_message_receipt(&owned_message).await;
                }
            }
        }
    }
}