extern crate dotenv;
mod command;
mod database;

use std::io::{Write};
use clap::{arg, Command};
use dotenv::dotenv;
use crate::command::Note;

fn cli() -> Command {
    Command::new("Rel")
        .about("Relational note taking")
        .subcommand_required(true)
        .subcommand(
            Command::new("n")
                .about("Create a note")
                .arg(arg!(<ID> "The node's id"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("e")
                .arg(arg!(<ID> "The node's id"))
                .arg_required_else_help(true)
                .about("Edit a note")
        )
        .subcommand(
            Command::new("d")
                .arg(arg!(<ID> "The node's id"))
                .arg_required_else_help(true)
                .about("Delete a note")
        )
        .subcommand(
            Command::new("l")
                .about("List all notes")
        )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("n", sub_matches)) => {
            let id = sub_matches.get_one::<String>("ID").expect("required");
            let db = database::Database::new();
            let note = db.get_note(&id).await;
            match note {
                Some(n) => {
                    println!("Note already exists, id: {}", id);
                    return Ok(());
                },
                None => {
                    let note = Note {
                        title: id.parse().unwrap(),
                        text: "".parse().unwrap()
                    };
                    let new_note = command::edit_node(&note).await;
                    db.save_note(&new_note).await;
                }
            }
            Ok(())
        },
        Some(("d", sub_matches)) => {
            let id = sub_matches.get_one::<String>("ID").expect("required");
            let db = database::Database::new();
            let note = db.get_note(&id).await;
            match note {
                Some(n) => {
                    db.delete_note(&n).await;
                    println!("Deleted note");
                },
                None => {
                    println!("Failed to delete a note with id: {}", id);
                    return Ok(());
                }
            }
            Ok(())
        },
        Some(("e", sub_matches)) => {
            let id = sub_matches.get_one::<String>("ID").expect("required");
            let db = database::Database::new();
            let note = db.get_note(&id).await;
            match note {
                Some(n) => {
                    let new_note = command::edit_node(&n).await;
                    db.update_note(&new_note).await;
                },
                None => {
                    println!("Failed to find a note with id: {}", id);
                    return Ok(());
                }
            }
            Ok(())
        },
        Some(("l", sub_matches)) => {
            let notes = command::poll_nodes().await;
            notes.iter().for_each(|n| {
                println!("{}", n.title);
            });
            Ok(())
        },
        _ => unreachable!()
    }
}
