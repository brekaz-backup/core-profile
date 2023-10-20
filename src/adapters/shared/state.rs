use rdkafka::producer::FutureProducer;
use blumer_lib_authorization_rs::clients::profile::ProfileAuthorization;
use crate::persistence::repositories::{ProfileRepository, RedisRepository};

#[derive(Clone)]
pub struct AppState {
    pub profile_repository: ProfileRepository,
    pub producer: FutureProducer,
    pub redis_repository: RedisRepository,
    pub auth_client: ProfileAuthorization,
}
