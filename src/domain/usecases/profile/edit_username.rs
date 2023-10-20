use blumer_lib_errors::AppError;
use lazy_static::lazy_static;
use uuid::Uuid;
use regex::Regex;
use crate::domain::entities::ProfileEntity;
use crate::domain::mappers::ProfileEntityMapper;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface};
use crate::utils::errors::ProfileError;


lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_.]").unwrap();
}

pub struct EditUsernameUseCase;

impl EditUsernameUseCase {
    pub async fn execute(repository: &impl ProfileRepositoryInterface, redis: &impl RedisRepositoryInterface, user_id: &Uuid, new_username: &String) -> Result<ProfileEntity, AppError> {
        let username = EditUsernameUseCase::clean_username(new_username)?;

        if repository.find_user_id_by_username(&username).await.is_ok() {
            return Err(
                ProfileError::UsernameNotAvailable.into()
            );
        }

        repository.can_update_username(&user_id).await?;
        repository.edit_username(&user_id, &username).await?;
        repository.new_username_update(&user_id).await?;
        let profile = repository.find_user_by_id(&user_id).await?;
        let data = ProfileEntityMapper::proto(&profile).await?;
        redis.store_profile(&user_id, &data).await?;
        Ok(profile)
    }

    fn clean_username(value: &String) -> Result<String, AppError> {
        let username = value.to_lowercase();
        if !USERNAME_REGEX.is_match(&username) {
            return Err(ProfileError::NotValidUsername.into());
        }

        Ok(username)
    }
}