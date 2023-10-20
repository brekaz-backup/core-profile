use blumer_lib_errors::AppError;
use uuid::Uuid;
use crate::domain::entities::ProfileEntity;
use crate::domain::mappers::ProfileEntityMapper;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface};

#[derive(Default)]
pub struct EditNamesUseCase;

impl EditNamesUseCase {
    pub async fn execute(repository: &impl ProfileRepositoryInterface, redis: &impl RedisRepositoryInterface, user_id: &Uuid, names: &String) -> Result<ProfileEntity, AppError> {
        repository.can_update_names(user_id).await?;
        repository.edit_names(&user_id, &names).await?;
        repository.new_name_update(&user_id).await?;
        let profile = repository.find_user_by_id(&user_id).await?;
        let data = ProfileEntityMapper::proto(&profile).await?;
        redis.store_profile(&user_id, &data).await?;
        Ok(profile)
    }
}