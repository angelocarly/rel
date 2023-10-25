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
    text: String,
}

pub async fn edit_node(note: &Note) {
    // Create a directory inside of `std::env::temp_dir()`.
    let dir = tempdir().unwrap();

    let file_path = dir.path().join("note");
    let mut file = File::create(file_path.clone()).unwrap();

    // Write node data
    writeln!(file, "{}", note.text.as_str()).expect("Couldn't write to temp file");

    // Do vim thingy
    vim(file_path.as_path().clone().to_str().unwrap());

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    println!("{}", data);

    let new_note = Note {
        title: note.clone().title,
        text: data
    };
    update_note(&new_note).await;

    drop(file);
    dir.close().expect("Failed to close directory");
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

pub async fn update_note(note: &Note){

    let db = database::Database::new();
    let graph = db.graph();

    let mut result = graph.await.execute(
        query( "MATCH (n:Note {title: $title}) SET n.text = $text RETURN n")
            .param("title", note.title.as_str())
            .param("text", note.text.as_str())
    ).await.unwrap();

    result.next().await.unwrap().unwrap();
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