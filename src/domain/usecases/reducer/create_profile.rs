use blumer_lib_errors::AppError;

use crate::domain::entities::CreateProfileEntity;
use crate::persistence::repositories::ReducerRepositoryInterface;


pub struct CreateProfileUseCase;

impl CreateProfileUseCase {

    pub async fn execute(repository: &impl ReducerRepositoryInterface, entity: CreateProfileEntity) -> Result<(), AppError> {
        repository.create_profile(entity).await
    }
}