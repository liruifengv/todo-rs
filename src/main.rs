#[warn(unused_variables)]
mod cli;
mod database;

use clap::Parser;
use cli::{Cli, Commands};
use database::Database;

fn main() {
    let args = Cli::parse();

    let mut db = Database::open(".rodorc");

    match args.command {
        Commands::Info => {
            println!("Rodo is a simple todo list manager.");
        }
        Commands::Add { content } => {
            if let Some(content) = content {
                println!("Adding a todo item: {}", content);
                let len = db.read_records().len();
                db.add_record(&database::Record {
                    id: len as i32 + 1,
                    content: content,
                });
            } else {
                println!("You need to specify the content of the todo item.");
            }
        }
        Commands::Remove { id } => {
            if id.is_none() {
                println!("You need to specify the id of the todo item.");
                return;
            }
            println!("Removing a todo item: {}", id.clone().unwrap());
            db.remove_record(id.unwrap().parse::<i32>().unwrap());
        }
        Commands::List => {
            let records = db.read_records();
            if records.is_empty() {
                println!("No records. You can add one with `rodo add [content]`");
                return;
            }
            for record in records {
                println!(" ⬜️ {}: {}", record.id, record.content);
            }
        }
    }
}
