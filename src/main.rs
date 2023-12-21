use std::env;
mod todo; //https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
mod tools;
use crate::tools::directory;

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
            "add" | "a" => todos.add(args[2..].to_vec(), parser),
            "done" | "d" => todos.done(args[2..].to_vec()),
            "remove" | "rm" => todos.remove(args[2..].to_vec()), // it should remove all tasks with the given ids'
            "help" => todo::print_info(todo::Info::Help),
            &_ => todo::print_info(todo::Info::Help),
        }
    } else {
        todos.list(parser.todo_list);
    }
}
