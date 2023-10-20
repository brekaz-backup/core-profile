use blumer_lib_errors::AppError;
use uuid::Uuid;
use crate::domain::entities::ProfileEntity;
use crate::domain::mappers::ProfileEntityMapper;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface};


pub struct EditPrivacyUseCase;

impl EditPrivacyUseCase {

    pub async fn execute(repository: &impl ProfileRepositoryInterface, redis: &impl RedisRepositoryInterface, user_id: &Uuid, privacy: &bool) -> Result<ProfileEntity, AppError> {
        repository.edit_privacy(user_id, privacy).await?;
        let profile = repository.find_user_by_id(&user_id).await?;
        let data = ProfileEntityMapper::proto(&profile).await?;
        redis.store_profile(&user_id, &data).await?;
        Ok(profile)
    }
}