use std::collections::HashMap;
use async_graphql::*;
use async_graphql::dataloader::Loader;
use uuid::Uuid;
use crate::adapters::profile::mappers::ProfileMapper;
use crate::adapters::profile::objects::Profile;
use crate::domain::mappers::graphql_mapper::GraphqlMapper;
use crate::domain::usecases::profile::FindByIdUsecase;
use crate::persistence::repositories::{ProfileRepository, RedisRepository};

#[derive(Clone)]
pub struct AppDataLoader {
    pub repository: ProfileRepository,
    pub redis: RedisRepository,
}


#[async_trait::async_trait]
impl Loader<Uuid> for AppDataLoader {
    type Value = Profile;
    type Error = Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let repository = &self.repository;
        let redis = &self.redis;
        let profiles = FindByIdUsecase::execute(&repository, &redis, &keys.into()).await.extend()?;

        Ok(profiles
            .into_iter()
            .map(|profile_entity| (profile_entity.user_id, ProfileMapper::to_object(profile_entity)))
            .collect::<HashMap<_, _>>())
    }
}