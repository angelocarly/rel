use std::sync::Arc;
use neo4rs::{Graph, Node, query};
use crate::command::Note;

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

    pub async fn get_note(&self, title: &str) -> Option<Note> {
        let mut result = self.graph().await.execute(
            query( "MATCH (n:Note {title: $title}) RETURN n")
                .param("title", title)
        ).await.unwrap();

        match result.next().await {
            Ok(r) => {
                match r {
                    Some(row) => {
                        let node: Node = row.get("n").unwrap();
                        return Some(Note {
                            title: node.get("title").unwrap(),
                            text: node.get("text").unwrap()
                        })
                    },
                    None => {
                        return None;
                    }
                }
            },
            Err(e) => {
                panic!("Failed to get row: {}", e);
            }
        }
    }

    pub async fn update_note(&self, note: &Note) {

        let mut result = self.graph().await.execute(
            query( "MATCH (n:Note {title: $title}) SET n.text = $text RETURN n")
                .param("title", note.title.as_str())
                .param("text", note.text.as_str())
        ).await.unwrap();

        result.next().await.unwrap().unwrap();
    }

    pub async fn save_note(&self, note: &Note) {

        let mut result = self.graph().await.execute(
            query( "CREATE (n:Note {title: $title, text: $text}) RETURN n")
                .param("title", note.title.as_str())
                .param("text", note.text.as_str())
        ).await.unwrap();

        result.next().await.unwrap().unwrap();
    }

    pub async fn delete_note(&self, note: &Note) {

        let mut result = self.graph().await.execute(
            query( "MATCH (n:Note {title: $title}) DELETE n")
                .param("title", note.title.as_str())
        ).await.unwrap();

        result.next().await.unwrap();
    }

}