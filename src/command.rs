use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{Write, Read, Seek, SeekFrom};
use std::ops::Not;
use std::sync::Arc;
use clap::builder::Str;
use neo4rs::{Graph, Node, query};
use tempfile::{tempdir, tempfile};
use crate::database;

#[derive(Clone)]
pub struct Note {
    pub title: String,
    pub(crate) text: String,
}

pub async fn edit_node(note: &Note) -> Note {
    // Create a directory inside of `std::env::temp_dir()`.
    let dir = tempdir().unwrap();

    let file_path = dir.path().join("note.md");
    let mut file = File::create(file_path.clone()).unwrap();

    // Write node data
    writeln!(file, "{}", note.text.as_str()).expect("Couldn't write to temp file");

    // Do vim thingy
    vim(file_path.as_path().clone().to_str().unwrap());

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    println!("{}", data);

    drop(file);
    dir.close().expect("Failed to close directory");

    return Note {
        title: note.clone().title,
        text: data
    };

}

pub async fn poll_nodes() -> Vec<Note> {

    let db = database::Database::new();
    let graph = db.graph().await;

    return tokio::spawn(async move {
        let mut result = graph.execute(
            query("MATCH (n:Note) RETURN n;")
        ).await.unwrap();

        let mut notes: Vec<Note> = vec![];
        while let Ok(Some(row)) = result.next().await {
            let node: Node = row.get("n").unwrap();
            let note = Note {
                title: node.get("title").unwrap(),
                text: node.get("text").unwrap(),
            };
            notes.push(note.clone());
        }
        return notes;
    }).await.expect("Tokio panicked");
}

pub fn vim(path: &str) {
    std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("/opt/homebrew/bin/nvim {}", path))
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");

}