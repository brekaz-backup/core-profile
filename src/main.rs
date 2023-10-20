#[cfg_attr(test, macro_use)]
extern crate log;
extern crate env_logger;

mod adapters;
mod domain;
mod infrastructure;
mod persistence;
mod utils;

use std::env;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use async_graphql::dataloader::DataLoader;
use blumer_lib_authorization_rs::clients::profile::ProfileAuthorization;
use dotenv::dotenv;

use crate::{
    adapters::{
        graphql::{configure_service, create_schema_with_context},
        shared::state::AppState,
    },
};
use crate::adapters::profile::AppDataLoader;
use crate::domain::events::reducer::{ProfileEventConsumer, REDUCER_CREATE_PROFILE_TOPIC,
                                     REDUCER_PROFILE_PHOTO_TOPIC,
                                     REDUCER_REACTION_TOPIC, REDUCER_VERIFY_TOPIC};
use crate::infrastructure::kafka::{KafkaConsumerConfig, KafkaProducerConfig};
use crate::infrastructure::redis::config::RedisConfig;
use crate::infrastructure::scylladb::connection::ScyllaConfig;
use crate::infrastructure::scylladb::initialization::ScyllaInit;
use crate::persistence::repositories::{ProfileRepository, RedisRepository, ReducerRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let session = ScyllaConfig::create_scylla_session().await;

    ScyllaInit::create_keyspaces(&session).await;
    ScyllaInit::create_tables(&session).await;
    ScyllaInit::create_materialized_views(&session).await;

    let redis_client = RedisConfig::connection().await;
    let scylla_session = Arc::new(session);
    let kafka_group = std::env::var("KAFKA_GROUP").expect("Can't get DB URL");
    let app_data_loader =
        DataLoader::new(
            AppDataLoader {
                redis: RedisRepository::new(redis_client.clone()),
                repository: ProfileRepository::new(scylla_session.clone()),
            }, tokio::spawn,
        );


    let redis_client = RedisConfig::connection().await;
    let state = AppState {
        profile_repository: ProfileRepository::new(scylla_session.clone()),
        producer: KafkaProducerConfig::create_producer(),
        redis_repository: RedisRepository::new(redis_client.clone()),
        auth_client: ProfileAuthorization::new(env::var("AUTH_URL").expect("Can't get DB URL")).await.expect("Can connect RPC client")
    };

    let redis_consumer = RedisConfig::connection().await;
    let session = Arc::new(ScyllaConfig::create_scylla_session().await);

    let profile_event = ProfileEventConsumer::new(
        ReducerRepository::new(session.clone()),
        ProfileRepository::new(session.clone()),
        RedisRepository::new(redis_consumer),
        KafkaConsumerConfig::create_consumer(
            kafka_group,
            vec![
                REDUCER_CREATE_PROFILE_TOPIC,
                REDUCER_REACTION_TOPIC,
                REDUCER_PROFILE_PHOTO_TOPIC,
                REDUCER_VERIFY_TOPIC,
            ],
        ),
        KafkaProducerConfig::create_producer(),
    );

    tokio::spawn(
        async move {
            profile_event.consume().await
        }
    );

    let schema = web::Data::new(create_schema_with_context(state, app_data_loader));
    let host = std::env::var("GRAPHQL_HOST").unwrap();
    println!("GraphiQL IDE: http://{}", host);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(configure_service)
            .app_data(schema.clone())
    })
        .bind(host)?
        .run()
        .await
}
