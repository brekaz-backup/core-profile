use blumer_lib_errors::AppError;

use crate::domain::entities::DeleteProfileEntity;
use crate::persistence::repositories::ReducerRepositoryInterface;


pub struct DeleteProfileUseCase;

impl DeleteProfileUseCase {

    pub async fn execute(repository: &impl ReducerRepositoryInterface, entity: &DeleteProfileEntity) -> Result<(), AppError> {
        repository.delete_profile(&entity).await
    }
}