use std::sync::Arc;
use neo4rs::Graph;

pub struct Database {
    uri: String,
    username: String,
    password: String,
}

impl Database {
    pub fn new() -> Database {
        let uri = std::env::var("NEO_URI").expect("Can't read env var");
        let username = std::env::var("NEO_USERNAME").expect("Can't read env var");
        let password = std::env::var("NEO_PASSWORD").expect("Can't read env var");

        Database {
            uri: uri.parse().unwrap(),
            username: username.parse().unwrap(),
            password: password.parse().unwrap(),
        }
    }

    pub async fn graph(&self) -> Arc<Graph> {
        Arc::new(Graph::new(&self.uri, &self.username, &self.password).await.unwrap())
    }
}