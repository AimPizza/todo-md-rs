use std::env;
mod todo; //https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
mod tools;
use crate::todo::*;

fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    // set up the configuration
    let conf_file: ConfigFile = ConfigFile::init();
    let conf_todo = TodoConfig::new(&conf_file);
    let complete_path = conf_file.path.todo_path.join(&conf_file.path.todo_filename); // TODO: nasty workaround but best until fixed

    // get the todos
    let mut todos = Todo::new(); // get empty parser
    todos.strings_to_todo(read_lines(&complete_path), &conf_todo); // populate parser

    // process arguments
    if args.len() > 1 {
        let operation = &args[1];
        match &operation[..] {
            "list" | "l" | "ls" => todos.list(),
            "add" | "a" => todos.add(args[2..].to_vec(), &conf_todo, &conf_file),
            "done" | "d" => todos.done(args[2..].to_vec()),
            "remove" | "rm" => todos.remove(args[2..].to_vec(), complete_path),
            "help" => todo::print_info(todo::Info::Help),
            &_ => todo::print_info(todo::Info::Help),
        }
    } else {
        todos.list();
    }
}
