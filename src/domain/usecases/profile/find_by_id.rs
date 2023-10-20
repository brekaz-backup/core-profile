use anyhow::Result;
use blumer_lib_errors::AppError;

use uuid::Uuid;
use crate::domain::entities::ProfileEntity;
use crate::domain::mappers::ProfileEntityMapper;
use crate::persistence::repositories::{ProfileRepositoryInterface, RedisRepositoryInterface};


pub struct FindByIdUsecase;

impl FindByIdUsecase {

    pub async fn execute(repository: &impl ProfileRepositoryInterface,
                         redis: &impl RedisRepositoryInterface, user_ids: &Vec<Uuid>) -> Result<Vec<ProfileEntity>, AppError> {
        repository.find_user_by_ids(&user_ids).await
    }
}

