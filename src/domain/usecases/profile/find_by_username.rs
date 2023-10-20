use anyhow::Result;
use blumer_lib_errors::AppError;
use uuid::Uuid;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface};


pub struct FindByUsernameUsecase;

impl FindByUsernameUsecase {

    pub async fn execute(repository: &impl ProfileRepositoryInterface, redis: &impl RedisRepositoryInterface, username: &String) -> Result<Uuid, AppError> {
        return if let Some(profile_id) = redis.get_profile_id(&username).await? {
            Ok(Uuid::parse_str(&profile_id).map_err(|e| AppError::ServerError(e.to_string()))?)
        } else {
            let profile_id = repository.find_user_id_by_username(username).await?;
            redis.store_profile_id(&username, &profile_id).await?;
            Ok(profile_id)
        };
    }
}