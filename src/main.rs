use std::env;
mod database;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rodo [add|rm|ls] [args]");
        return;
    }

    let mut db = database::Database::open(".rodorc");

    let command = &args[1];
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: rodo add [contents]");
                return;
            }
            let contents = &args[2..].join(" ");
            let len = db.read_records().len();
            db.add_record(&database::Record {
                id: len as i32 + 1,
                content: contents.to_string(),
            });
        }
        "rm" => {
            if args.len() < 3 {
                println!("Usage: rodo rm [id]");
                return;
            }
            let id = args[2].parse::<i32>().unwrap();
            db.remove_record(id);
        }
        "ls" => {
            let records = db.read_records();
            if records.is_empty() {
                println!("No records. You can add one with `rodo add [content]`");
                return;
            }
            for record in records {
                println!(" ⬜️ {}: {}", record.id, record.content);
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
