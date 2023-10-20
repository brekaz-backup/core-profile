use async_trait::async_trait;
use blumer_lib_errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait RedisRepositoryInterface {
    async fn get_profile(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError>;
    async fn store_profile(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError>;
    async fn get_profile_id(&self, username: &String) -> Result<Option<String>, AppError>;
    async fn store_profile_id(&self, username: &String, profile_id: &Uuid) -> Result<(), AppError>;
    async fn get_profile_details(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError>;
    async fn store_profile_details(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError>;
    async fn get_count(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError>;
    async fn store_count(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError>;
    async fn get_social(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError>;
    async fn store_social(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError>;
}