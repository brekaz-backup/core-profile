use blumer_lib_errors::AppError;
use uuid::Uuid;

use crate::domain::entities::{ProfileEntity, ProfilePhotoEntity};
use crate::domain::mappers::ProfileEntityMapper;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface, ReducerRepositoryInterface};


pub struct VerifyUseCase;

impl VerifyUseCase {
    pub async fn execute(reducer_repository: &impl ReducerRepositoryInterface,
                         profile_repository: &impl ProfileRepositoryInterface,
                         redis: &impl RedisRepositoryInterface,
                         user_id: &Uuid, verified: bool) -> Result<ProfileEntity, AppError> {
        reducer_repository.verify(&user_id, verified).await?;
        let profile = profile_repository.find_user_by_id(&user_id).await?;
        let data = ProfileEntityMapper::proto(&profile).await?;
        redis.store_profile(&user_id, &data).await?;
        Ok(profile)
    }
}