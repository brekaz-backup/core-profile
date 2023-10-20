use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;
use blumer_lib_errors::AppError;
use chrono::{Duration, TimeZone, Utc};
use futures::{StreamExt};
use scylla::batch::{Batch, BatchType};
use scylla::prepared_statement::PreparedStatement;
use scylla::Session;
use uuid::Uuid;
use crate::domain::entities::{ProfileDetailsEntity, ProfileEntity};
use crate::persistence::repositories::profile_repository_interface::ProfileRepositoryInterface;
use crate::utils::errors::ProfileError;

#[derive(Clone)]
pub struct ProfileRepository {
    session: Arc<Session>,
}


impl ProfileRepository {
    pub fn new(session: Arc<Session>) -> Self {
        ProfileRepository { session }
    }
}


#[async_trait]
impl ProfileRepositoryInterface for &ProfileRepository {
    async fn find_user_by_id(&self, user_id: &Uuid) -> Result<ProfileEntity, AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare(r#"
                SELECT
                    user_id, username, names, photo, photo_hash, portrait, portrait_hash,
                    description, verified, privacy, is_active
                FROM
                    profile.profiles
                WHERE
                    user_id = ?
            "#)
            .await?;
        let result = self.session.execute(&prepared, (user_id, ))
            .await?.first_row_typed::<ProfileEntity>().map_err(|_| AppError::NotFound)?;
        if !result.is_active {
            return Err(
                AppError::NotFound
            );
        }

        Ok(result)
    }


    async fn find_user_by_ids(&self, user_ids: &Vec<Uuid>) -> Result<Vec<ProfileEntity>, AppError> {
        let mut user_vec: Vec<ProfileEntity> = Vec::new();

        let prepared: PreparedStatement = self
            .session
            .prepare(r#"
                SELECT
                    user_id, username, names, photo, photo_hash, portrait, portrait_hash,
                    description, verified, privacy, is_active
                FROM
                    profile.profiles
                WHERE
                    user_id IN ?
            "#)
            .await?;

        let mut rows_stream = self.session.execute_iter(prepared, (user_ids, )).await?.into_typed::<ProfileEntity>();

        while let Some(next_row_res) = rows_stream.next().await {
            let entity = next_row_res.map_err(|e| AppError::DatasourceError(e.to_string()))?;

            if entity.is_active {
                user_vec.push(entity);
            }
        }

        Ok(user_vec)
    }

    async fn find_user_id_by_username(&self, username: &String) -> Result<Uuid, AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("SELECT user_id FROM profile.by_username WHERE username = ?")
            .await?;
        let result = self.session.execute(&prepared, (username, ))
            .await?.first_row_typed::<(Uuid, )>().map_err(|_| AppError::NotFound)?;
        Ok(result.0)
    }

    async fn find_user_details(&self, user_id: &Uuid) -> Result<ProfileDetailsEntity, AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare(r#"
                SELECT
                    user_id, birthday, lat, lng
                FROM
                    profile.profile_details
                WHERE
                    user_id = ?
            "#)
            .await?;
        let result = self.session.execute(&prepared, (user_id, ))
            .await?.first_row_typed::<ProfileDetailsEntity>().map_err(|_| AppError::NotFound)?;

        Ok(result)
    }


    async fn edit_privacy(&self, user_id: &Uuid, privacy: &bool) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profiles SET privacy = ? WHERE user_id = ?")
            .await?;

        let _ = self.session.execute(&prepared, (&privacy, &user_id, )).await?;
        Ok(())
    }

    async fn edit_username(&self, user_id: &Uuid, username: &String) -> Result<(), AppError> {
        let mut batch: Batch = Batch::new(BatchType::Unlogged);
        batch.append_statement(r#"
            UPDATE
                profile.profiles
            SET
                username = ?
            WHERE
                user_id = ?
        "#);
        batch.append_statement(r#"
            INSERT INTO
                profile.by_username (username, user_id)
            VALUES
                (?, ?)
        "#);

        let prepared_batch: Batch = self.session.prepare_batch(&batch).await?;

        let batch_values = (
            (&username, &user_id),
            (&username, &user_id)
        );

        let _ = self.session.batch(&prepared_batch, batch_values).await?;

        Ok(())
    }

    async fn edit_names(&self, user_id: &Uuid, names: &String) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profiles SET names = ? WHERE user_id = ?")
            .await?;

        let _ = self.session.execute(&prepared, (&names, &user_id, )).await?;
        Ok(())
    }

    async fn edit_description(&self, user_id: &Uuid, description: &String) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.profiles SET description = ? WHERE user_id = ?")
            .await?;

        let _ = self.session.execute(&prepared, (&description, &user_id, )).await?;
        Ok(())
    }

    async fn can_update_username(&self, user_id: &Uuid) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("SELECT user_id, updated_at FROM profile.username_updates WHERE user_id = ?")
            .await?;
        let result = self.session.execute(&prepared, (user_id, ))
            .await?.maybe_first_row_typed::<(Uuid, Duration, )>().map_err(|_| AppError::NotFound)?;

        if let Some(data) = result {
            let last_date = Utc::now() - Duration::days(30);
            let dt = Utc.timestamp_nanos(data.1.num_nanoseconds().unwrap_or_default());

            if dt > last_date {
                return Err(
                    ProfileError::CantUpdateUsernameYet.into()
                );
            }
        }
        Ok(())
    }

    async fn can_update_names(&self, user_id: &Uuid) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("SELECT user_id, updated_at FROM profile.name_updates WHERE user_id = ?")
            .await?;
        let result = self.session.execute(&prepared, (user_id, ))
            .await?.maybe_first_row_typed::<(Uuid, Duration, )>().map_err(|_| AppError::NotFound)?;

        if let Some(data) = result {
            let last_date = Utc::now() - Duration::days(30);
            let dt = Utc.timestamp_nanos(data.1.num_nanoseconds().unwrap_or_default());
            if dt > last_date {
                return Err(
                    ProfileError::CantUpdateNamesYet.into()
                );
            }
        }
        Ok(())
    }

    async fn new_name_update(&self, user_id: &Uuid) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.name_updates SET updated_at =  toTimestamp(now()) WHERE user_id = ?")
            .await?;

        let _ = self.session.execute(&prepared, (&user_id, )).await?;
        Ok(())
    }

    async fn new_username_update(&self, user_id: &Uuid) -> Result<(), AppError> {
        let prepared: PreparedStatement = self
            .session
            .prepare("UPDATE profile.username_updates SET updated_at =  toTimestamp(now()) WHERE user_id = ?")
            .await?;

        let _ = self.session.execute(&prepared, (&user_id, )).await?;
        Ok(())
    }
}
