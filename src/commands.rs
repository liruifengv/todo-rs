use crate::database::{Database, Record};
use crate::utils::get_db_file_path;
use std::io;

// Show rodo info
pub fn info() -> Result<(), io::Error> {
    let db_file = get_db_file_path();
    println!("Rodo version: {}", env!("CARGO_PKG_VERSION"));
    println!("Rodo is a simple todo list manager.");
    println!("Your todo list is stored at: {}", db_file.display());
    Ok(())
}

// Add a todo item
pub fn add(db: &mut Database, content: Option<String>) -> Result<(), io::Error> {
    if let Some(content) = content {
        println!("Adding a todo item: {}", content);

        let id = db.read_records().last().map(|r| r.id + 1).unwrap_or(1);

        db.add_record(&Record {
            id,
            content: content.clone(),
        })?;
        println!("📝 Item added: {}", content);
        Ok(())
    } else {
        eprintln!("You need to specify the content of the todo item.");
        std::process::exit(1);
    }
}

// Remove a todo item
pub fn remove(db: &mut Database, id: Option<String>) -> Result<(), io::Error> {
    if id.is_none() {
        println!("You need to specify the id of the todo item.");
        std::process::exit(1);
    }
    println!("Removing a todo item: {}", id.clone().unwrap());
    db.remove_record(id.unwrap().parse::<i32>().unwrap())?;
    println!(" ❌ Item removed!\n");
    Ok(())
}

// List all todo items
pub fn list(db: &Database) -> Result<(), io::Error> {
    let records = db.read_records();
    if records.is_empty() {
        eprintln!("No records. You can add one with `rodo add [content]`");
        std::process::exit(1);
    }
    for record in records {
        println!(" ⬜️ {}: {}", record.id, record.content);
    }
    Ok(())
}
