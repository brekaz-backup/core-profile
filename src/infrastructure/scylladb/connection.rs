use std::env;

use scylla::{Session, SessionBuilder};

pub struct ScyllaConfig;

impl ScyllaConfig {
    pub async fn create_scylla_session() -> Session {
        let user = env::var("SCYLLADB_USER").expect("Can't get SCYLLADB_USER");
        let password = env::var("SCYLLADB_PASSWORD").expect("Can't get SCYLLADB_PASSWORD");
        let nodes = env::var("SCYLLADB_NODES").expect("Can't get SCYLLADB_NODES");
        let session = SessionBuilder::new()
            .known_nodes(&nodes.split(",").collect::<Vec<&str>>())
            .user(user, password)
            .build()
            .await
            .expect("Can connect To SycllaDB");

        session
    }
}
