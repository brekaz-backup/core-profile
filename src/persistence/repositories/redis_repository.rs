use async_trait::async_trait;
use blumer_lib_errors::AppError;
use redis::cluster::ClusterClient;
use uuid::Uuid;

use crate::persistence::repositories::redis_repository_interface::RedisRepositoryInterface;


#[derive(Clone)]
pub struct RedisRepository {
    session: ClusterClient,
}


impl RedisRepository {
    pub fn new(session: ClusterClient) -> Self {
        RedisRepository { session }
    }
}


#[async_trait]
impl RedisRepositoryInterface for &RedisRepository {
    async fn get_profile(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::get(format!("profile:{}", profile_id.to_string()))
            .query_async::<_, Vec<u8>>(&mut conn)
            .await.map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        Ok(if !res.is_empty() { Some(res) } else { None })
    }

    async fn store_profile(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::set_ex(format!("profile:{}", profile_id.to_string()), data, 60)
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        if res == "OK" {
            Ok(())
        } else {
            Err(AppError::DatasourceError("Cant Store Profile in redis".to_string()))
        }
    }


    async fn get_profile_id(&self, username: &String) -> Result<Option<String>, AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::get(format!("username:{}", username))
            .query_async::<_, Option<String>>(&mut conn)
            .await.map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        Ok(res)
    }

    async fn store_profile_id(&self, username: &String, profile_id: &Uuid) -> Result<(), AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::set_ex(format!("username:{}", username), profile_id.to_string(), 60)
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        if res == "OK" {
            Ok(())
        } else {
            Err(AppError::DatasourceError("Cant Store Profile in redis".to_string()))
        }
    }

    async fn get_profile_details(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::get(format!("profile:details:{}", profile_id.to_string()))
            .query_async::<_, Vec<u8>>(&mut conn)
            .await.map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        Ok(if !res.is_empty() { Some(res) } else { None })
    }

    async fn store_profile_details(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::set_ex(format!("profile:details:{}", profile_id.to_string()), data, 60)
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        if res == "OK" {
            Ok(())
        } else {
            Err(AppError::DatasourceError("Cant Store Profile in redis".to_string()))
        }
    }


    async fn get_count(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::get(format!("count:{}", profile_id.to_string()))
            .query_async::<_, Vec<u8>>(&mut conn)
            .await.map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        Ok(if !res.is_empty() { Some(res) } else { None })
    }


    async fn store_count(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::set_ex(format!("count:{}", profile_id.to_string()), data, 90)
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        if res == "OK" {
            Ok(())
        } else {
            Err(AppError::DatasourceError("Cant Store Profile in redis".to_string()))
        }
    }

    async fn get_social(&self, profile_id: &Uuid) -> Result<Option<Vec<u8>>, AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::get(format!("social:{}", profile_id.to_string()))
            .query_async::<_, Vec<u8>>(&mut conn)
            .await.map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        Ok(if !res.is_empty() { Some(res) } else { None })
    }

    async fn store_social(&self, profile_id: &Uuid, data: &Vec<u8>) -> Result<(), AppError> {
        let mut conn = self.session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res = redis::Cmd::set_ex(format!("social:{}", profile_id.to_string()), data, 90)
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        if res == "OK" {
            Ok(())
        } else {
            Err(AppError::DatasourceError("Cant Store Social in redis".to_string()))
        }
    }
}