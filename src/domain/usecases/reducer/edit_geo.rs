use blumer_lib_errors::AppError;

use crate::domain::entities::EditGeoEntity;
use crate::persistence::repositories::ReducerRepositoryInterface;


pub struct EditGeoUseCase;

impl EditGeoUseCase{

    pub async fn execute(reducer_repository: &impl ReducerRepositoryInterface, entity: &EditGeoEntity) -> Result<(), AppError> {
        reducer_repository.edit_geo(&entity).await
    }
}