use blumer_lib_errors::AppError;

use crate::domain::entities::{ProfileEntity, ProfilePhotoEntity};
use crate::domain::mappers::ProfileEntityMapper;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface, ReducerRepositoryInterface};


pub struct EditPhotoUseCase;

impl EditPhotoUseCase{

    pub async fn execute(reducer_repository: &impl ReducerRepositoryInterface,
                         profile_repository: &impl ProfileRepositoryInterface,
                         redis: &impl RedisRepositoryInterface,
                         entity: &ProfilePhotoEntity) -> Result<ProfileEntity, AppError> {
        reducer_repository.edit_photo(&entity).await?;
        let profile = profile_repository.find_user_by_id(&entity.user_id).await?;
        let data = ProfileEntityMapper::proto(&profile).await?;
        redis.store_profile(&entity.user_id, &data).await?;
        Ok(profile)
    }
}