extern crate dotenv;
mod command;
mod database;

use std::env;
use std::fs::File;
use std::io::Write;
use clap::Command;
use dotenv::dotenv;
fn cli() -> Command {
    Command::new("Rel")
        .about("Relational note taking")
        .subcommand_required(true)
        .subcommand(
            Command::new("log")
                // .arg(arg!(id: [ID]))
                .about("Register working on something")
        )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("log", sub_matches)) => {
            let notes = command::poll_nodes().await;
            command::edit_node(notes.first().unwrap()).await;
            Ok(())
        },
        _ => unreachable!()
    }
}
