use std::io::{self, Write};

pub fn print_usage() {
    let usage = "\
    Usage: rodo [command] [options]\n\n\
    Commands:\n\
    \tadd [contents]\t\tAdd a todo item\n\
    \trm [item_id]\t\tRemove a todo item\n\
    \tls\t\t\tList all the todo items\n\
";
    let mut stderr = io::stderr();
    writeln!(stderr, "{}", usage).unwrap();
}
