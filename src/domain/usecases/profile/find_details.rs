use anyhow::Result;
use blumer_lib_errors::AppError;
use uuid::Uuid;
use crate::domain::entities::ProfileDetailsEntity;
use crate::domain::events::mappers::ProfileDetailMapper;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface};


pub struct FindDetailsUseCase;

impl FindDetailsUseCase {
    pub async fn execute(repository: &impl ProfileRepositoryInterface, redis: &impl RedisRepositoryInterface, profile_id: &Uuid) -> Result<ProfileDetailsEntity, AppError> {
        return if let Some(details) = redis.get_profile_details(&profile_id).await? {
            Ok(
                ProfileDetailMapper::entity(&details).await.map_err(|e| AppError::ServerError(format!("From Proto {:?}", e)))?
            )
        } else {
            let details = repository.find_user_details(&profile_id).await?;
            let data = ProfileDetailMapper::proto(&details).await.map_err(|e| AppError::ServerError(format!("To Proto {:?}", e)))?;
            redis.store_profile_details(&profile_id, &data).await?;
            Ok(details)
        };
    }
}