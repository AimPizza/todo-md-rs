use std::env;
mod todo; //https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
mod tools;
use crate::tools::directory;
// TODO:
// [ ] get the date thing right
//  [ ] see: https://stackoverflow.com/questions/50072055/converting-unix-timestamp-to-readable-time-string-in-rust
//
// think of a system to check all the tasks and assign IDs for new ones
// [ ] at initialization, serialize all lines into a vec of todos. where should they be held?
//       see todo.md for reference
//       make indentation according to tag of todo item to be added, mby two spaces per indent
// [ ] write some proper documentation to get a clear structure to this project
// [ ] organize all files other than main.rs and put them into the tools/
// [ ] flags
//  [ ] configuration wizard

// https://docs.rs/toml/latest/toml/
// https://doc.rust-lang.org/std/fs/index.html

fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    let config = directory::get_config();
    let todos = todo::TodoHandler::init(&config);

    //populate TodoParser
    let mut parser = todo::TodoParser::new(&config);
    parser.strings_to_todo(directory::read_lines(&todos.path.join(&todos.filename)));

    if args.len() > 1 {
        let operation = &args[1];
        match &operation[..] {
            "list" | "l" | "ls" => todos.list(parser.todo_list),
            "add" | "a" => todos.add(args[2..].to_vec()),
            "done" | "d" => todos.done(args[2..].to_vec()),
            "remove" | "rm" => todos.remove(args[2..].to_vec()), // it should remove all tasks with the given ids'
            "help" => todo::print_info(5),
            &_ => todo::print_info(4),
        }
    } else {
        todos.list(parser.todo_list);
    }
}
