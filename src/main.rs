use std::env;
mod todo; //https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
mod tools;
use crate::tools::directory;

fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    let config: directory::Config = directory::get_config();
    let handler = todo::TodoHandler::init(&config);

    // get the todos
    let mut parser = todo::TodoParser::new(&config); // get empty parser
    parser.strings_to_todo(directory::read_lines(&handler.complete_path)); // populate parser

    if args.len() > 1 {
        let operation = &args[1];
        match &operation[..] {
            "list" | "l" | "ls" => handler.list(parser.todo_list),
            "add" | "a" => handler.add(args[2..].to_vec(), parser),
            "done" | "d" => handler.done(args[2..].to_vec()),
            "remove" | "rm" => {
                handler.remove(args[2..].to_vec(), handler.complete_path.clone(), parser)
            } // it should remove all tasks with the given ids'
            "help" => todo::print_info(todo::Info::Help),
            &_ => todo::print_info(todo::Info::Help),
        }
    } else {
        handler.list(parser.todo_list);
    }
}
