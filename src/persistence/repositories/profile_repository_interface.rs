use async_trait::async_trait;
use anyhow::Result;
use blumer_lib_errors::AppError;
use uuid::Uuid;
use crate::domain::entities::{ProfileDetailsEntity, ProfileEntity};

#[async_trait]
pub trait ProfileRepositoryInterface {
    async fn find_user_by_id(&self, user_id: &Uuid) -> Result<ProfileEntity, AppError>;
    async fn find_user_by_ids(&self, user_ids: &Vec<Uuid>) -> Result<Vec<ProfileEntity>, AppError>;
    async fn find_user_id_by_username(&self, username: &String) -> Result<Uuid, AppError>;
    async fn find_user_details(&self, user_id: &Uuid) -> Result<ProfileDetailsEntity, AppError>;
    async fn edit_privacy(&self, user_id: &Uuid, privacy: &bool) -> Result<(), AppError>;
    async fn edit_username(&self, user_id: &Uuid, username: &String) -> Result<(), AppError>;
    async fn edit_names(&self, user_id: &Uuid, names: &String) -> Result<(), AppError>;
    async fn edit_description(&self, user_id: &Uuid, description: &String) -> Result<(), AppError>;
    async fn can_update_username(&self, user_id: &Uuid) -> Result<(), AppError>;
    async fn can_update_names(&self, user_id: &Uuid) -> Result<(), AppError>;
    async fn new_name_update(&self, user_id: &Uuid) -> Result<(), AppError>;
    async fn new_username_update(&self, user_id: &Uuid) -> Result<(), AppError>;
}