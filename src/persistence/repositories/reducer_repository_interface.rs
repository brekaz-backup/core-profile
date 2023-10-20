use async_trait::async_trait;
use blumer_lib_errors::AppError;
use uuid::Uuid;

use crate::domain::entities::{CreateProfileEntity, DeleteProfileEntity, EditGeoEntity, ProfilePhotoEntity};

#[async_trait]
pub trait ReducerRepositoryInterface {
    async fn create_profile(&self, entity: CreateProfileEntity) -> Result<(), AppError>;
    async fn edit_geo(&self, entity: &EditGeoEntity) -> Result<(), AppError>;
    async fn edit_photo(&self, entity: &ProfilePhotoEntity) -> Result<(), AppError>;
    async fn edit_portrait(&self, entity: &ProfilePhotoEntity) -> Result<(), AppError>;
    async fn verify(&self, user_id: &Uuid, verified: bool) -> Result<(), AppError>;
    async fn delete_profile(&self, entity: &DeleteProfileEntity) -> Result<(), AppError>;
}