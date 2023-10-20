use dotenv::dotenv;
use scylla::Session;
use anyhow::Result;
use crate::infrastructure::scylladb::connection::ScyllaConfig;
use crate::infrastructure::scylladb::initialization::ScyllaInit;

pub async fn init_app() -> Result<(Session, )> {
    dotenv().ok();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let session = ScyllaConfig::create_scylla_session().await;

    ScyllaInit::create_keyspaces(&session).await;
    ScyllaInit::create_tables(&session).await;
    ScyllaInit::create_materialized_views(&session).await;

    Ok((session, ))
}