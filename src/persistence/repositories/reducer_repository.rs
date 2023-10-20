use async_trait::async_trait;
use std::sync::Arc;
use blumer_lib_errors::AppError;

use scylla::batch::{Batch, BatchType};
use scylla::prepared_statement::PreparedStatement;
use scylla::Session;
use uuid::Uuid;

use crate::domain::entities::{CreateProfileEntity, DeleteProfileEntity, EditGeoEntity, ProfilePhotoEntity};
use crate::persistence::repositories::ReducerRepositoryInterface;

#[derive(Clone)]
pub struct ReducerRepository {
    session: Arc<Session>,
}


impl ReducerRepository {
    pub fn new(session: Arc<Session>) -> Self {
        ReducerRepository { session }
    }
}


#[async_trait]
impl ReducerRepositoryInterface for &ReducerRepository {
    async fn create_profile(&self, entity: CreateProfileEntity) -> Result<(), AppError> {
        let mut batch: Batch = Batch::new(BatchType::Unlogged);
        batch.append_statement(r#"
            INSERT INTO
                    profile.profiles (
                        user_id, username, names, photo, photo_hash, portrait, portrait_hash,
                        description, privacy, verified, is_active
                    )
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, true)
        "#);


        batch.append_statement(r#"
            INSERT INTO
                profile.by_username (username, user_id)
            VALUES
                (?, ?)
        "#);


        batch.append_statement(r#"
            INSERT INTO
                profile.profile_details (user_id, lat, lng, birthday, created_at)
            VALUES
                (?, 0.0, 0.0, ?, toTimestamp(now()))
        "#);

        let prepared_batch: Batch = self.session.prepare_batch(&batch).await?;

        let batch_values = (
            (
                &entity.user_id, &entity.username,
                &entity.names, &entity.photo, &entity.photo_hash,
                &entity.portrait, &entity.portrait_hash,
                &entity.description, &entity.privacy,
                &entity.verified,
            ),
            (&entity.username, &entity.user_id, ),
            (&entity.user_id, &entity.birthday, )
        );

        let _ = self.session.batch(&prepared_batch, batch_values).await?;
        Ok(())
    }

    async fn edit_geo(&self, entity: &EditGeoEntity) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profile_details SET lat = ?, lng = ? WHERE user_id = ?")
            .await?;

        let _ = self.session.execute(&prepared, (&entity.lat, &entity.lng, &entity.user_id, )).await?;
        Ok(())
    }

    async fn edit_photo(&self, entity: &ProfilePhotoEntity) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profiles SET photo = ?, photo_hash = ? WHERE user_id = ? IF EXISTS")
            .await?;

        let _ = self.session.execute(&prepared, (&entity.photo, &entity.photo_hash, &entity.user_id, )).await?;
        Ok(())
    }

    async fn edit_portrait(&self, entity: &ProfilePhotoEntity) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profiles SET portrait = ?, portrait_hash = ? WHERE user_id = ? IF EXISTS")
            .await?;

        let _ = self.session.execute(&prepared, (&entity.photo, &entity.photo_hash, &entity.user_id, )).await?;
        Ok(())
    }

    async fn verify(&self, user_id: &Uuid, verified: bool) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profiles SET verified = ? WHERE user_id = ? IF EXISTS")
            .await?;

        let _ = self.session.execute(&prepared, (&verified, &user_id,)).await?;
        Ok(())
    }

    async fn delete_profile(&self, entity: &DeleteProfileEntity) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profiles SET is_active = false WHERE user_id = ? IF EXISTS")
            .await?;

        let _ = self.session.execute(&prepared, (&entity.user_id, )).await?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use futures::StreamExt;
    use uuid::Uuid;
    use crate::domain::entities::ProfileEntity;
    use crate::domain::events::mappers::ProfileEventMapper;
    use crate::domain::events::reducer::REDUCER_CREATE_PROFILE_TOPIC;
    use crate::infrastructure::kafka::KafkaProducerConfig;
    use crate::utils::test::init_app;
    use super::*;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).try_init();
    }

    #[actix_web::main]
    #[test]
    async fn test_create_profile() {
        init();

        let (session, ) = init_app().await.expect("Error initializing app");

        let repository = &ReducerRepository {
            session: Arc::new(session)
        };

        let entity = CreateProfileEntity {
            user_id: Uuid::parse_str("d08dc16f-f0df-43f1-9f18-1b569302d875").expect("Error parsing uuid"),
            username: "demodemo".to_string(),
            names: "Demo Test".to_string(),
            photo: None,
            photo_hash: None,
            portrait: None,
            portrait_hash: None,
            birthday: NaiveDate::from_ymd_opt(1995, 10, 04).expect("Error creating date"),
            description: "".to_string(),
            verified: true,
            privacy: true,
            lat: 0.0,
            lng: 0.0,
            is_active: true,
        };
        let create = repository.create_profile(entity).await;
        info!("Creating profile {:?}", create)
    }


    #[actix_web::main]
    #[test]
    async fn test_edit_geo() {
        init();

        let (session, ) = init_app().await.expect("Error initializing app");

        let repository = &ReducerRepository {
            session: Arc::new(session)
        };

        let entity = EditGeoEntity {
            user_id: Uuid::parse_str("d08dc16f-f0df-43f1-9f18-1b569302d875").expect("Error parsing uuid"),
            lat: 2.0,
            lng: 3.0,
        };
        let create = repository.edit_geo(&entity).await;
        info!("Edit geo {:?}", create)
    }

    #[actix_web::main]
    #[test]
    async fn test_verify() {
        init();

        let (session, ) = init_app().await.expect("Error initializing app");

        let repository = &ReducerRepository {
            session: Arc::new(session)
        };

        let user_id = Uuid::parse_str("d08dc16f-f0df-43f1-9f18-1b569302d875").expect("Error parsing uuid");
        let verified = true;
        let verify = repository.verify(&user_id, true).await;
        info!("Verify {:?}", verify);
        assert!(verify.is_ok())
    }

    // #[actix_web::main]
    // #[test]
    // async fn test_bulk_create_profile() {
    //     init();
    //
    //     let (session, ) = init_app().await.expect("Error initializing app");
    //
    //     let mut user_vec: Vec<ProfileEntity> = Vec::new();
    //
    //     let prepared: PreparedStatement = session
    //         .prepare(r#"
    //             SELECT
    //                 user_id, username, names, photo, photo_hash, portrait, portrait_hash,
    //                 description, verified, privacy, is_active
    //             FROM
    //                 profile.profiles
    //         "#)
    //         .await.expect("prepare error");
    //
    //     let mut rows_stream = session.execute_iter(prepared, &[]).await.expect("error excecuting").into_typed::<ProfileEntity>();
    //
    //     let producer = KafkaProducerConfig::create_producer();
    //
    //     while let Some(next_row_res) = rows_stream.next().await {
    //         if let Ok(entity) = next_row_res {
    //             let proto = ProfileEventMapper::proto(&CreateProfileEntity {
    //                 user_id: entity.user_id,
    //                 username: entity.username,
    //                 names: entity.names,
    //                 photo: entity.photo,
    //                 photo_hash: entity.photo_hash,
    //                 portrait: entity.portrait,
    //                 portrait_hash: entity.portrait_hash,
    //                 birthday: NaiveDate::from_ymd_opt(2000, 10, 10).unwrap(),
    //                 description: entity.description,
    //                 verified: entity.verified,
    //                 privacy: entity.privacy,
    //                 lat: 0.0,
    //                 lng: 0.0,
    //                 is_active: true,
    //             }).await.expect("Error proto");
    //
    //             KafkaProducerConfig::send_message(&producer, &proto, &REDUCER_CREATE_PROFILE_TOPIC).await
    //         }
    //
    //     }
    // }
}